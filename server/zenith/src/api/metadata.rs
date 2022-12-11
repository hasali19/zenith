use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use speq::axum::post;
use tmdb::TmdbClient;

use crate::api::ApiResult;
use crate::db::Db;
use crate::metadata::{self, MetadataManager};

use super::error;

#[post("/metadata/match_all")]
#[response(status = 200)]
async fn match_all(
    metadata: Extension<MetadataManager>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    metadata.enqueue_all_unmatched(&mut conn).await?;
    Ok(())
}

#[post("/metadata/refresh_outdated")]
#[response(status = 200)]
async fn refresh_outdated(
    metadata: Extension<MetadataManager>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    metadata.enqueue_all_outdated(&mut conn).await?;
    Ok(())
}

#[post("/metadata/:id/find_match")]
#[path(i64)]
#[response(status = 200)]
async fn find_match(
    Path(id): Path<i64>,
    tmdb: Extension<TmdbClient>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    metadata::find_match(&mut conn, &tmdb, id)
        .await
        .map_err(|e| match e {
            metadata::Error::NotFound => error::not_found("media item not found"),
            metadata::Error::Other(e) => e.into(),
        })?;

    Ok(StatusCode::OK)
}

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
            metadata::Error::NotFound => error::not_found("media item not found"),
            metadata::Error::Other(e) => e.into(),
        })?;

    Ok(StatusCode::OK)
}
