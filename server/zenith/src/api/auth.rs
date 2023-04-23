use async_trait::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_extra::extract::cookie::{Cookie, Key, SameSite};
use axum_extra::extract::PrivateCookieJar;
use db::Db;
use serde::Deserialize;
use speq::axum::post;

use super::error::{unauthorized, ApiError};
use super::ApiResult;

pub struct User {
    pub id: i64,
    pub username: String,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for User
where
    Key: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let cookies = PrivateCookieJar::<Key>::from_request_parts(parts, state)
            .await
            .unwrap();

        let db = parts.extensions.get::<Db>().unwrap();

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
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
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

    Ok(cookies.add(
        Cookie::build("auth", user.id.to_string())
            .same_site(SameSite::Lax)
            .path("/")
            .http_only(true)
            .permanent()
            .finish(),
    ))
}
