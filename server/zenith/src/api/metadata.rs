use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_codegen::post;

use crate::api::ApiResult;
use crate::db::media::MediaItemType;
use crate::db::{self, Db};
use crate::metadata::{MetadataManager, RefreshRequest};

use super::ext::OptionExt;

#[post("/metadata/:id/refresh")]
async fn refresh_metadata(
    Path(id): Path<i64>,
    metadata: Extension<MetadataManager>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let item_type = db::media::get_item_type(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    let refresh_req = match item_type {
        MediaItemType::Movie => RefreshRequest::Movie(id),
        MediaItemType::TvShow => RefreshRequest::TvShow(id),
        MediaItemType::TvSeason => RefreshRequest::TvSeason(id),
        MediaItemType::TvEpisode => RefreshRequest::TvEpisode(id),
    };

    metadata.enqueue(refresh_req);

    Ok(StatusCode::OK)
}
