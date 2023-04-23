use axum::response::IntoResponse;
use axum::{Extension, Json};
use db::Db;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use speq::axum::{get, post};

use super::ApiResult;

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

#[derive(Deserialize)]
struct NewUser {
    username: String,
}

#[post("/users")]
async fn create(db: Extension<Db>, Json(body): Json<NewUser>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let user = db::users::NewUser {
        username: &body.username,
    };

    let id = db::users::create(&mut conn, user).await?;

    Ok(Json(User {
        id,
        username: body.username,
    }))
}
