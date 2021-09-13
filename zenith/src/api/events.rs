use std::sync::Arc;

use atium::headers::{CacheControl, ContentType};
use atium::respond::RespondRequestExt;
use atium::router::Router;
use atium::{endpoint, Body, Request};

use crate::broadcaster::Broadcaster;

pub fn routes(router: &mut Router) {
    router.route("/events").get(connect);
}

#[endpoint]
async fn connect(req: &mut Request) -> eyre::Result<()> {
    let broadcaster: &Arc<Broadcaster> = req.ext().unwrap();
    let stream = broadcaster.new_client().await;

    req.ok()
        .header(CacheControl::new().with_no_cache())
        .header(ContentType::from(mime::TEXT_EVENT_STREAM))
        .body(Body::wrap_stream(stream));

    Ok(())
}
