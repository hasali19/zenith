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

    db::trakt::connect(&mut conn, user.id, &body.refresh_token).await?;

    Ok(NoContent)
}
