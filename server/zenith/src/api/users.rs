use axum::response::IntoResponse;
use axum::{Extension, Json};
use db::Db;
use eyre::Context;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use speq::axum::{get, post};

use crate::password_utils::hash_password;

use super::error::ApiError;
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
}

#[post("/users")]
async fn create(
    db: Extension<Db>,
    user: Result<auth::User, ApiError>,
    Json(body): Json<NewUser>,
) -> ApiResult<impl IntoResponse> {
    let mut transaction = db.begin().await?;

    // Must be authenticated to create a user, unless no users exist
    if let Err(e) = user {
        let users = db::users::get_all(&mut transaction).await?;
        if !users.is_empty() {
            return Err(e);
        }
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
