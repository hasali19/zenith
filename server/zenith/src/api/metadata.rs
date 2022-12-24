use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use speq::axum::post;
use tmdb::TmdbClient;

use crate::api::ApiResult;
use crate::db::{self, Db};
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

#[derive(Deserialize)]
struct SetMetadataMatch {
    tmdb_id: i32,
}

#[post("/metadata/:id/set_match")]
async fn set_match(
    Path(id): Path<i64>,
    body: Json<SetMetadataMatch>,
    tmdb: Extension<TmdbClient>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let metadata = db::items::UpdateMetadata {
        tmdb_id: Some(Some(body.tmdb_id)),
        ..Default::default()
    };

    db::items::update_metadata(&mut conn, id, metadata).await?;

    metadata::refresh(&mut conn, &tmdb, id)
        .await
        .map_err(|e| match e {
            metadata::Error::NotFound => error::not_found("media item not found"),
            metadata::Error::Other(e) => e.into(),
        })?;

    Ok(())
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
