use std::borrow::Cow;

use async_trait::async_trait;
use axum::extract::{FromRequestParts, Request};
use axum::http::{request, Method};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::PrivateCookieJar;
use db::Db;
use serde::Deserialize;
use speq::axum::post;
use tower::Service;

use crate::password_utils::verify_password;

use super::error::{unauthorized, ApiError};
use super::ApiResult;

#[derive(Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for User {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .remove()
            .ok_or_else(|| unauthorized("unauthorized"))
    }
}

// TODO: These should not be hardcoded here
const IGNORED_PATHS: &[&str] = &["/auth/login"];
const ALLOWED_PATHS: &[(&str, &[Method])] = &[("/users", &[Method::POST])];

pub async fn middleware(
    cookies: PrivateCookieJar,
    db: Extension<Db>,
    mut req: Request,
    mut next: Next,
) -> impl IntoResponse {
    if IGNORED_PATHS.contains(&req.uri().path()) {
        return Ok(next.call(req).await);
    }

    let user = match try_extract_user(cookies, &db).await {
        Ok(user) => user,
        Err(e) => {
            if ALLOWED_PATHS
                .iter()
                .any(|(p, m)| req.uri().path() == *p && m.contains(req.method()))
            {
                return Ok(next.call(req).await);
            } else {
                return Err(e);
            }
        }
    };

    req.extensions_mut().insert(super::auth::User {
        id: user.id,
        username: user.username,
    });

    Ok(next.call(req).await)
}

async fn try_extract_user(cookies: PrivateCookieJar, db: &Db) -> ApiResult<User> {
    let Some(user_id) = cookies.get("auth") else {
        return Err(unauthorized("invalid auth token"));
    };

    let user_id: i64 = match user_id.value().parse() {
        Ok(user_id) => user_id,
        Err(e) => {
            tracing::error!("{}", e);
            return Err(unauthorized("invalid auth token"));
        }
    };

    let mut conn = db.acquire().await?;

    let user = db::users::get_by_id(&mut conn, user_id)
        .await?
        .ok_or_else(|| unauthorized("invalid auth token"))?;

    Ok(User {
        id: user.id,
        username: user.username,
    })
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/auth/login")]
async fn login(
    cookies: PrivateCookieJar,
    db: Extension<Db>,
    credentials: Json<Credentials>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let user = db::users::get_by_username(&mut conn, &credentials.username)
        .await?
        .ok_or_else(|| unauthorized("invalid credentials"))?;

    verify_password(&credentials.password, &user.password_hash).map_err(|e| {
        tracing::error!("{e:?}");
        unauthorized("invalid credentials")
    })?;

    Ok(cookies.add(build_auth_cookie(Cow::Owned(user.id.to_string()))))
}

#[post("/auth/logout")]
async fn logout(cookies: PrivateCookieJar) -> ApiResult<impl IntoResponse> {
    Ok(cookies.remove(build_auth_cookie(Cow::Borrowed(""))))
}

fn build_auth_cookie(value: Cow<str>) -> Cookie {
    Cookie::build(("auth", value))
        .same_site(SameSite::Lax)
        .path("/")
        .http_only(true)
        .permanent()
        .build()
}
