use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::broadcaster::Broadcaster;

/// GET /api/events
pub async fn connect(req: HttpRequest) -> impl Responder {
    let broadcaster: &Arc<Broadcaster> = req.app_data().unwrap();
    let stream = broadcaster.new_client().await;

    HttpResponse::Ok()
        .append_header(("cache-control", "no-cache"))
        .append_header(("content-type", "text/event-stream"))
        .streaming(stream)
}
