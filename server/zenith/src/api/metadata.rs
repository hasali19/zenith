use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use speq::axum::post;
use tmdb::TmdbClient;

use crate::api::ApiResult;
use crate::db::media::MetadataProvider;
use crate::db::{self, Db};
use crate::metadata::{self, MetadataManager};
use crate::MediaItemType;

use super::error;
use super::ext::OptionExt;

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
    tmdb_id: Option<i32>,
    season_number: Option<i32>,
    episode_number: Option<i32>,
}

#[post("/metadata/:id/set_match")]
async fn set_match(
    Path(id): Path<i64>,
    body: Json<SetMetadataMatch>,
    tmdb: Extension<TmdbClient>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let item = db::items::get(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    let key = match item.kind {
        MediaItemType::Movie | MediaItemType::Show => format!(
            "{}",
            body.tmdb_id
                .or_bad_request("tmdb_id is required for movie or show")?
        ),
        MediaItemType::Season => format!(
            "{}:{}",
            body.tmdb_id
                .or_bad_request("tmdb_id is required for season")?,
            body.season_number
                .or_bad_request("season_number is required for season")?
        ),
        MediaItemType::Episode => {
            let tmdb_id = match body.tmdb_id {
                Some(tmdb_id) => tmdb_id,
                None => {
                    let tmdb_id = item
                        .metadata_provider_key
                        .as_deref()
                        .and_then(|key| key.split(':').next())
                        .and_then(|id| id.parse().ok());

                    match tmdb_id {
                        Some(tmdb_id) => tmdb_id,
                        None => db::items::get(&mut conn, item.parent.unwrap().id)
                            .await?
                            .unwrap()
                            .metadata_provider_key
                            .and_then(|key| key.parse().ok())
                            .or_bad_request("tmdb_id required since show is unmatched")?,
                    }
                }
            };

            format!(
                "{tmdb_id}:{}:{}",
                body.season_number
                    .or_bad_request("season_number is required for episode")?,
                body.episode_number
                    .or_bad_request("episode_number is required for episode")?
            )
        }
    };

    let metadata = db::items::UpdateMetadata {
        metadata_provider: Some(Some(MetadataProvider::Tmdb)),
        metadata_provider_key: Some(Some(&key)),
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
