use std::borrow::Cow;
use std::time::Duration;

use async_trait::async_trait;
use axum::extract::{FromRequestParts, Request};
use axum::http::{request, Method};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::PrivateCookieJar;
use db::Db;
use rand::distributions::Distribution;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_qs::axum::QsQuery;
use speq::axum::post;
use speq::Reflect;
use tower::Service;

use crate::password_utils::verify_password;

use super::error::{not_found, unauthorized, ApiError};
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
const IGNORED_PATHS: &[(&str, &[Method])] =
    &[("/auth/login", &[Method::POST]), ("/users", &[Method::GET])];
const ALLOWED_PATHS: &[(&str, &[Method])] = &[("/users", &[Method::POST])];

#[derive(Deserialize)]
pub struct AuthMiddlewareQuery {
    token: Option<String>,
}

pub async fn middleware(
    query: QsQuery<AuthMiddlewareQuery>,
    cookies: PrivateCookieJar,
    db: Extension<Db>,
    mut req: Request,
    mut next: Next,
) -> impl IntoResponse {
    if IGNORED_PATHS
        .iter()
        .any(|(p, m)| req.uri().path() == *p && m.contains(req.method()))
    {
        return Ok(next.call(req).await);
    }

    let auth_token = req
        .headers()
        .get("authorization")
        .and_then(|it| it.to_str().ok())
        .or_else(|| query.token.as_deref());

    let user = match try_extract_user(&auth_token, cookies, &db).await {
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

async fn try_extract_user(
    auth_token: &Option<&str>,
    cookies: PrivateCookieJar,
    db: &Db,
) -> ApiResult<User> {
    let user_id = match cookies.get("auth") {
        Some(user_id) => match user_id.value().parse() {
            Ok(user_id) => user_id,
            Err(e) => {
                tracing::error!("{}", e);
                return Err(unauthorized("invalid auth token"));
            }
        },
        None => {
            if let Some(token) = auth_token {
                let mut conn = db.acquire().await?;

                let token = db::access_tokens::get(&mut conn, token)
                    .await?
                    .ok_or_else(|| unauthorized("invalid auth token"))?;

                if let Some(expires_at) = token.expires_at {
                    let now = std::time::UNIX_EPOCH
                        .elapsed()
                        .map_err(|e| eyre::eyre!(e))?;

                    if now > Duration::from_secs(expires_at as u64) {
                        return Err(unauthorized("invalid auth token"));
                    }
                }

                token.user_id
            } else {
                return Err(unauthorized("invalid auth token"));
            }
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

#[derive(Clone, Copy, Serialize, Deserialize, Reflect)]
#[serde(rename_all = "snake_case")]
enum AccessTokenOwner {
    System,
    User,
}

#[derive(Deserialize, Reflect)]
struct TokenQuery {
    owner: AccessTokenOwner,
    name: String,
    #[serde(default)]
    create: bool,
}

#[derive(Serialize, Reflect)]
struct AccessToken {
    owner: AccessTokenOwner,
    name: String,
    user_id: i64,
    token: String,
    created_at: i64,
    expires_at: Option<i64>,
}

const ASCII_PRINTABLE: [u8; 126 - 33 + 1] = {
    let mut chars = [0; 126 - 33 + 1];
    let mut i = 0;
    while i <= 126 - 33 {
        chars[i as usize] = i + 33;
        i += 1;
    }
    chars
};

struct AsciiPrintable;

impl Distribution<char> for AsciiPrintable {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        *ASCII_PRINTABLE.choose(rng).unwrap() as char
    }
}

#[post("/auth/token")]
async fn get_token(
    #[query] query: QsQuery<TokenQuery>,
    db: Extension<Db>,
    user: User,
) -> ApiResult<impl IntoResponse> {
    let mut transaction = db.begin_write().await?;

    let owner = match query.owner {
        AccessTokenOwner::System => db::access_tokens::AccessTokenOwner::System,
        AccessTokenOwner::User => db::access_tokens::AccessTokenOwner::User,
    };

    let token =
        db::access_tokens::get_named(transaction.as_read(), user.id, owner, &query.name).await?;

    let token = match token {
        Some(token) => token,
        None => {
            if query.create {
                let token = rand::thread_rng()
                    .sample_iter(&AsciiPrintable)
                    .take(32)
                    .collect::<String>();

                db::access_tokens::create(
                    &mut transaction,
                    db::access_tokens::NewAccessToken {
                        user_id: user.id,
                        owner,
                        name: &query.name,
                        token: &token,
                        duration: None,
                    },
                )
                .await?
            } else {
                return Err(not_found("token not found"));
            }
        }
    };

    transaction.commit().await?;

    Ok(Json(AccessToken {
        owner: query.owner,
        name: token.name,
        token: token.token,
        user_id: token.user_id,
        created_at: token.created_at,
        expires_at: token.expires_at,
    }))
}
