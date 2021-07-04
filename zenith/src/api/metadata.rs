use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db::media::MediaItemType;
use crate::db::Db;
use crate::metadata::{MetadataManager, RefreshRequest};

/// POST /api/metadata/{id}/refresh
pub async fn refresh_metadata(
    req: HttpRequest,
    path: web::Path<(i64,)>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();

    let metadata: &MetadataManager = req.app_data().unwrap();
    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let item_type: MediaItemType =
        sqlx::query_scalar("SELECT item_type FROM media_items WHERE id = ?")
            .bind(id)
            .fetch_optional(&mut conn)
            .await
            .map_err(ErrorInternalServerError)?
            .ok_or_else(|| ErrorNotFound(""))?;

    let req = match item_type {
        MediaItemType::Movie => RefreshRequest::Movie(id),
        MediaItemType::TvShow => RefreshRequest::TvShow(id),
        MediaItemType::TvSeason => RefreshRequest::TvSeason(id),
        MediaItemType::TvEpisode => RefreshRequest::TvEpisode(id),
    };

    metadata.enqueue(req);

    Ok(HttpResponse::Ok())
}
