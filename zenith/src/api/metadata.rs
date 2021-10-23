use actix_web::web::Path;
use actix_web::{post, HttpResponse, Responder};

use crate::api::ApiResult;
use crate::db::media::MediaItemType;
use crate::db::{self, Db};
use crate::metadata::{MetadataManager, RefreshRequest};
use crate::Ext;

use super::ext::OptionExt;

#[post("/metadata/{id}/refresh")]
async fn refresh_metadata(
    id: Path<i64>,
    metadata: Ext<MetadataManager>,
    db: Db,
) -> ApiResult<impl Responder> {
    let id = id.into_inner();
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

    Ok(HttpResponse::Ok())
}
