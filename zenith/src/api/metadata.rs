use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request, StatusCode};

use crate::db::media::MediaItemType;
use crate::db::{self, Db};
use crate::metadata::{MetadataManager, RefreshRequest};

use super::ext::OptionExt;

pub fn routes(router: &mut Router) {
    router.route("/metadata/:id/refresh").post(refresh_metadata);
}

#[endpoint]
async fn refresh_metadata(req: &mut Request) -> eyre::Result<impl Responder> {
    let id: i64 = req.param("id")?;

    let metadata: &MetadataManager = req.ext().unwrap();
    let db: &Db = req.ext().unwrap();
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
