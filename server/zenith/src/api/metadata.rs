use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use speq::post;
use tmdb::TmdbClient;

use crate::api::ApiResult;
use crate::db::media::MediaItemType;
use crate::db::{self, Db};
use crate::metadata::{self, RefreshRequest};

use super::ext::OptionExt;

#[post("/metadata/:id/refresh")]
#[path(i64)]
#[response(status = 200)]
async fn refresh_metadata(
    Path(id): Path<i64>,
    tmdb: Extension<TmdbClient>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let item_type = db::media::get_item_type(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    let refresh_req = match item_type {
        MediaItemType::Movie => RefreshRequest::Movie(id),
        MediaItemType::Show => RefreshRequest::TvShow(id),
        MediaItemType::Season => RefreshRequest::TvSeason(id),
        MediaItemType::Episode => RefreshRequest::TvEpisode(id),
    };

    metadata::refresh(&mut conn, &*tmdb, refresh_req).await?;

    Ok(StatusCode::OK)
}
