use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Responder};

use crate::sync::SyncService;

use super::ApiResult;

pub fn service(path: &str) -> impl HttpServiceFactory {
    web::scope(path)
        .route("/sync", web::post().to(sync))
        .default_service(web::route().to(HttpResponse::NotFound))
}

async fn sync(mut sync_service: SyncService) -> ApiResult<impl Responder> {
    sync_service.start_full_sync();
    Ok(HttpResponse::Ok())
}
