use zenith_http::{App, Request, Response};

use crate::db::media::MediaItemType;
use crate::metadata::RefreshRequest;
use crate::AppState;

use super::{ApiError, ApiResult};

pub fn configure(app: &mut App<AppState>) {
    app.post("/api/metadata/:id/refresh", refresh_metadata);
}

async fn refresh_metadata(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

    let item_type: MediaItemType =
        sqlx::query_scalar("SELECT item_type FROM media_items WHERE id = ?")
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
