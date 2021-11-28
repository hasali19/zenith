use axum::extract::{Extension, Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_codegen::post;
use serde::Deserialize;

use crate::api::ApiResult;
use crate::db::videos::UpdateVideoUserData;
use crate::db::{self, Db};

use super::ext::OptionExt;

#[derive(Deserialize)]
struct ProgressUpdate {
    position: f64,
}

#[post("/progress/:id")]
async fn update_progress(
    id: Path<i64>,
    query: Query<ProgressUpdate>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let duration = db::videos::get_basic_info(&mut conn, *id)
        .await?
        .or_not_found("video not found")?
        .duration;

    let data = UpdateVideoUserData {
        position: Some(query.position),
        is_watched: Some((query.position / duration) >= 0.9),
    };

    db::videos::update_user_data(&mut conn, *id, data).await?;

    Ok(StatusCode::OK)
}
