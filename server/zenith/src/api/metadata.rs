use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use speq::axum::post;
use tmdb::TmdbClient;

use crate::api::ApiResult;
use crate::db::Db;
use crate::metadata;

use super::error;

#[post("/metadata/:id/refresh")]
#[path(i64)]
#[response(status = 200)]
async fn refresh_metadata(
    Path(id): Path<i64>,
    tmdb: Extension<TmdbClient>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    metadata::refresh(&mut conn, &tmdb, id)
        .await
        .map_err(|e| match e {
            metadata::RefreshError::NotFound => error::not_found("media item not found"),
            metadata::RefreshError::Other(e) => e.into(),
        })?;

    Ok(StatusCode::OK)
}
