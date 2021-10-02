use std::sync::Arc;

use atium::respond::RespondRequestExt;
use atium::router::Router;
use atium::{endpoint, Request, StatusCode};

use crate::library::scanner::ScanOptions;
use crate::library::LibraryScanner;

pub fn routes(router: &mut Router) {
    router.route("/scanner/start").post(start_scan);
}

#[endpoint]
async fn start_scan(req: &mut Request) -> eyre::Result<()> {
    let scanner: &Arc<LibraryScanner> = req.ext().unwrap();

    scanner.clone().start_scan(ScanOptions {
        rescan_files: true,
        refresh_metadata: false,
    });

    req.respond(StatusCode::OK);

    Ok(())
}
