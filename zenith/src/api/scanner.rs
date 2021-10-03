use std::sync::Arc;

use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request, StatusCode};

use crate::library::scanner::ScanOptions;
use crate::library::LibraryScanner;

pub fn routes(router: &mut Router) {
    router.route("/scanner/start").post(start_scan);
    router.route("/scanner/run/:id").post(scan_item);
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

#[endpoint]
async fn scan_item(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;
    let scanner: &Arc<LibraryScanner> = req.ext().unwrap();

    scanner.rescan_video_file(id).await?;
    req.respond(StatusCode::OK);

    Ok(())
}
