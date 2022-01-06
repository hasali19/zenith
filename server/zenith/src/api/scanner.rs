use std::sync::Arc;

use axum::extract::{Extension, Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_codegen::post;
use serde::Deserialize;

use crate::api::ApiResult;
use crate::library::scanner::ScanOptions;
use crate::library::LibraryScanner;

#[derive(Deserialize)]
struct StartScanQuery {
    #[serde(default)]
    rescan_files: bool,
    #[serde(default)]
    refresh_metadata: bool,
}

#[post("/scanner/start")]
async fn start_scan(
    Query(query): Query<StartScanQuery>,
    Extension(scanner): Extension<Arc<LibraryScanner>>,
) -> impl IntoResponse {
    scanner.start_scan(ScanOptions {
        rescan_files: query.rescan_files,
        refresh_metadata: query.refresh_metadata,
    });

    StatusCode::OK
}

#[post("/scanner/run/:id")]
async fn scan_item(
    Path(id): Path<i64>,
    Extension(scanner): Extension<Arc<LibraryScanner>>,
) -> ApiResult<impl IntoResponse> {
    scanner.rescan_video_file(id).await?;
    Ok(StatusCode::OK)
}
