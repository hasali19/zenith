use std::time::{Duration, SystemTime};

use axum::response::IntoResponse;
use axum::{Extension, Json};
use db::Db;
use eyre::Context;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use speq::axum::{get, post};
use uuid::Uuid;

use crate::password_utils::hash_password;

use super::error::{bad_request, ApiError};
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

#[derive(Deserialize)]
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
    let mut transaction = db.begin().await?;

    // Must be authenticated to create a user, unless a registration code is provided or no users exist
    if let Err(e) = user {
        match body.registration_code {
            Some(code) => {
                let registration = db::user_registrations::get(&mut transaction, &code)
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
            }
            None => {
                let users = db::users::get_all(&mut transaction).await?;
                if !users.is_empty() {
                    return Err(e);
                }
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
}

#[post("/users/registrations")]
async fn create_registration(_user: auth::User, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut transaction = db.begin().await?;

    let id = Uuid::new_v4().to_string();

    let data = db::user_registrations::NewUserRegistration {
        id: &id,
        duration: Duration::from_secs(7 * 24 * 60 * 60),
    };

    db::user_registrations::create(&mut transaction, data).await?;

    transaction.commit().await?;

    Ok(Json(UserRegistration { code: id }))
}
