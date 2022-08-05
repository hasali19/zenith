use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_qs::axum::QsQuery;
use speq::{post, Reflect};

use crate::api::error::not_found;
use crate::api::ApiResult;
use crate::library::scanner::{FileScanResult, ScanOptions};
use crate::library::LibraryScanner;

#[derive(Deserialize, Reflect)]
struct ScanOptionsQuery {
    #[serde(default)]
    rescan_files: bool,
    #[serde(default)]
    refresh_metadata: bool,
}

impl From<ScanOptionsQuery> for ScanOptions {
    fn from(query: ScanOptionsQuery) -> Self {
        ScanOptions {
            rescan_files: query.rescan_files,
            refresh_metadata: query.refresh_metadata,
        }
    }
}

#[post("/scanner/start")]
#[response(status = 200)]
async fn start_scan(
    #[query] QsQuery(query): QsQuery<ScanOptionsQuery>,
    Extension(scanner): Extension<Arc<LibraryScanner>>,
) {
    scanner.start_scan(query.into());
}

#[derive(Serialize, Reflect)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ItemScanResult {
    Added { id: i64 },
    Updated { id: i64 },
    Removed,
    Ignored,
}

#[post("/scanner/run/:id")]
#[path(i64)]
#[response(model = ItemScanResult)]
async fn scan_item(
    Path(id): Path<i64>,
    #[query] QsQuery(query): QsQuery<ScanOptionsQuery>,
    Extension(scanner): Extension<Arc<LibraryScanner>>,
) -> ApiResult<impl IntoResponse> {
    let result = match scanner.scan_file(id, &query.into()).await? {
        Some(res) => res,
        None => return Err(not_found("no video found with the given id")),
    };

    let result = match result {
        FileScanResult::Added(id) => ItemScanResult::Added { id },
        FileScanResult::Updated(id) => ItemScanResult::Updated { id },
        FileScanResult::Removed => ItemScanResult::Removed,
        FileScanResult::Ignored => ItemScanResult::Ignored,
    };

    Ok(Json(result))
}
