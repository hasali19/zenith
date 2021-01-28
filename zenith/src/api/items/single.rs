use serde::Deserialize;
use zenith_server::{Request, Response};

use crate::api::{ApiError, ApiResult};
use crate::db::media::MediaItemType;
use crate::metadata::RefreshRequest;
use crate::AppState;

use super::common::MediaItem;

pub(super) async fn get(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

    let sql = "
        SELECT *
        FROM media_items AS m
        LEFT JOIN user_item_data AS u ON m.id = u.item_id
        WHERE id = ?
    ";

    let item: MediaItem = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .ok_or_else(ApiError::not_found)?;

    Ok(Response::new().json(&item)?)
}

pub(super) async fn refresh_metadata(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

    let (item_type,): (MediaItemType,) =
        sqlx::query_as("SELECT item_type FROM media_items WHERE id = ?")
            .bind(id)
            .fetch_optional(&mut conn)
            .await?
            .ok_or_else(ApiError::not_found)?;

    let req = match item_type {
        MediaItemType::Movie => RefreshRequest::Movie(id),
        MediaItemType::TvShow => RefreshRequest::TvShow(id),
        MediaItemType::TvSeason => RefreshRequest::TvSeason(id),
        MediaItemType::TvEpisode => RefreshRequest::TvEpisode(id),
    };

    state.metadata.enqueue(req);

    Ok(Response::new())
}

#[derive(Deserialize)]
struct ProgressUpdate {
    position: f64,
}

pub(super) async fn update_progress(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let data: ProgressUpdate = req
        .query()
        .map_err(|e| ApiError::bad_request().body(e.to_string()))?;

    let mut conn = state.db.acquire().await?;

    let (item_type, duration): (MediaItemType, f64) =
        sqlx::query_as("SELECT item_type, duration FROM media_items WHERE id = ?")
            .bind(id)
            .fetch_optional(&mut conn)
            .await?
            .ok_or_else(ApiError::not_found)?;

    if !matches!(item_type, MediaItemType::Movie | MediaItemType::TvEpisode) {
        let msg = format!("cannot set progress for item of type {:?}", item_type);
        return Err(ApiError::bad_request().body(msg));
    }

    let sql = "
        INSERT INTO user_item_data (item_id, position, is_watched)
        VALUES (?, ?, ?)
        ON CONFLICT (item_id) DO UPDATE
        SET position = excluded.position,
            is_watched = is_watched OR excluded.is_watched
    ";

    sqlx::query(sql)
        .bind(id)
        .bind(data.position)
        .bind((data.position / duration) >= 0.9)
        .execute(&mut conn)
        .await?;

    Ok(Response::new())
}
