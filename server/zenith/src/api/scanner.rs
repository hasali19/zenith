use std::sync::Arc;

use axum::extract::Extension;

use crate::library::LibraryScanner;

/// POST /scanner/start
pub async fn start_scan(Extension(scanner): Extension<Arc<LibraryScanner>>) {
    scanner.start_scan();
}
