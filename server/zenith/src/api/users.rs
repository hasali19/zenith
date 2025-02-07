use std::time::{Duration, SystemTime};

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use db::Db;
use eyre::{eyre, Context};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use speq::axum::{delete, get, post};
use speq::Reflect;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::password_utils::hash_password;

use super::error::{bad_request, not_found, ApiError};
use super::ext::OptionExt;
use super::{auth, ApiResult};

#[derive(Serialize)]
struct User {
    id: i64,
    username: String,
}

#[get("/users")]
async fn get_all(db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let users = db::users::get_all(&mut conn)
        .await?
        .into_iter()
        .map(|user| User {
            id: user.id,
            username: user.username,
        })
        .collect_vec();

    Ok(Json(users))
}

#[get("/users/me")]
async fn get_authenticated_user(user: auth::User) -> ApiResult<impl IntoResponse> {
    Ok(Json(User {
        id: user.id,
        username: user.username,
    }))
}

#[derive(Deserialize, Reflect)]
struct NewUser {
    username: String,
    password: String,
    registration_code: Option<String>,
}

#[post("/users")]
async fn create(
    db: Extension<Db>,
    user: Result<auth::User, ApiError>,
    Json(body): Json<NewUser>,
) -> ApiResult<impl IntoResponse> {
    let mut transaction = db.begin_write().await?;

    // Must be authenticated to create a user, unless a registration code is provided or no users have been created yet.
    if let Err(e) = user {
        match body.registration_code {
            Some(code) => {
                let registration = db::user_registrations::get(transaction.as_read(), &code)
                    .await?
                    // Reject if code is invalid
                    .or_bad_request("invalid registration code")?;

                let expires_at =
                    SystemTime::UNIX_EPOCH + Duration::from_secs(registration.expires_at as u64);

                // Delete to prevent reuse
                db::user_registrations::delete(&mut transaction, &registration.id).await?;

                // Reject if code has expired
                if SystemTime::now() > expires_at {
                    return Err(bad_request("invalid registration code"));
                }

                tracing::info!("creating new user {}", body.username);
            }
            None => {
                let users = db::users::get_all(transaction.as_read()).await?;
                if !users.is_empty() {
                    return Err(e);
                }
                tracing::info!("creating initial user {}", body.username);
            }
        };
    }

    let password_hash = hash_password(&body.password).wrap_err("failed to hash password")?;

    let user = db::users::NewUser {
        username: &body.username,
        password_hash: &password_hash,
    };

    let id = db::users::create(&mut transaction, user).await?;

    transaction.commit().await?;

    Ok(Json(User {
        id,
        username: body.username,
    }))
}

#[derive(Serialize)]
struct UserRegistration {
    code: String,
    created_at: String,
    expires_at: String,
}

#[get("/users/registrations")]
async fn get_registrations(_user: auth::User, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let registrations: Vec<_> = db::user_registrations::get_all(&mut conn)
        .await?
        .into_iter()
        .map(UserRegistration::try_from)
        .try_collect()?;

    Ok(Json(registrations))
}

#[post("/users/registrations")]
async fn create_registration(_user: auth::User, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut transaction = db.begin_write().await?;

    let id = Uuid::new_v4().to_string();

    let data = db::user_registrations::NewUserRegistration {
        id: &id,
        duration: Duration::from_secs(7 * 24 * 60 * 60),
    };

    let registration = db::user_registrations::create(&mut transaction, data).await?;

    transaction.commit().await?;

    Ok(Json(UserRegistration::try_from(registration)?))
}

#[delete("/users/registrations/{id}")]
async fn delete_registration(
    _user: auth::User,
    Path(code): Path<String>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire_write().await?;

    let is_deleted = db::user_registrations::delete(&mut conn, &code).await?;

    if !is_deleted {
        return Err(not_found("no registration found with the given id"));
    }

    Ok(StatusCode::NO_CONTENT)
}

impl TryFrom<db::user_registrations::UserRegistration> for UserRegistration {
    type Error = eyre::Report;

    fn try_from(value: db::user_registrations::UserRegistration) -> Result<Self, Self::Error> {
        let id = value.id;
        let created_at = value.created_at;
        let expires_at = value.expires_at;
        Ok(UserRegistration {
            code: id,
            created_at: OffsetDateTime::from_unix_timestamp(created_at)
                .wrap_err_with(|| eyre!("failed to create DateTime for timestamp: {created_at}"))?
                .format(&Iso8601::DEFAULT)?,
            expires_at: OffsetDateTime::from_unix_timestamp(expires_at)
                .wrap_err_with(|| eyre!("failed to create DateTime for timestamp: {expires_at}"))?
                .format(&Iso8601::DEFAULT)?,
        })
    }
}
