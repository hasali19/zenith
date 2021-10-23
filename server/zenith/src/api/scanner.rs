use std::sync::Arc;

use actix_web::web::Path;
use actix_web::{post, HttpResponse, Responder};

use crate::api::ApiResult;
use crate::library::scanner::ScanOptions;
use crate::library::LibraryScanner;
use crate::Ext;

#[post("/scanner/start")]
pub async fn start_scan(Ext(scanner): Ext<Arc<LibraryScanner>>) -> impl Responder {
    scanner.start_scan(ScanOptions {
        rescan_files: true,
        refresh_metadata: false,
    });

    HttpResponse::Ok()
}

#[post("/scanner/run/{id}")]
pub async fn scan_item(
    id: Path<i64>,
    Ext(scanner): Ext<Arc<LibraryScanner>>,
) -> ApiResult<impl Responder> {
    scanner.rescan_video_file(*id).await?;
    Ok(HttpResponse::Ok())
}
