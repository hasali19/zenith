use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_codegen::post;

use crate::api::ApiResult;
use crate::library::scanner::ScanOptions;
use crate::library::LibraryScanner;

#[post("/scanner/start")]
pub async fn start_scan(Extension(scanner): Extension<Arc<LibraryScanner>>) -> impl IntoResponse {
    scanner.start_scan(ScanOptions {
        rescan_files: true,
        refresh_metadata: false,
    });

    StatusCode::OK
}

#[post("/scanner/run/:id")]
pub async fn scan_item(
    Path(id): Path<i64>,
    Extension(scanner): Extension<Arc<LibraryScanner>>,
) -> ApiResult<impl IntoResponse> {
    scanner.rescan_video_file(id).await?;
    Ok(StatusCode::OK)
}
