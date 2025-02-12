use axum::response::{IntoResponse, NoContent};
use axum::{Extension, Json};
use db::Db;
use serde::Deserialize;
use speq::Reflect;
use speq::axum::post;

use super::{ApiResult, auth};

#[derive(Deserialize, Reflect)]
struct TraktConnectionRequest {
    refresh_token: String,
}

#[post("/trakt/connect")]
async fn connect(
    db: Extension<Db>,
    user: auth::User,
    body: Json<TraktConnectionRequest>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire_write().await?;

    sqlx::query("insert into trakt_user_auth (user_id, refresh_token) values (?, ?)")
        .bind(user.id)
        .bind(&body.refresh_token)
        .execute(&mut *conn)
        .await?;

    Ok(NoContent)
}
