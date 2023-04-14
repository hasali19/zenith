use std::sync::Arc;

use axum::extract::Extension;
use speq::axum::post;

use crate::library::LibraryScanner;

#[post("/scanner/start")]
#[response(status = 200)]
async fn start_scan(Extension(scanner): Extension<Arc<LibraryScanner>>) {
    scanner.start_scan();
}
