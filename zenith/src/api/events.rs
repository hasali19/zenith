use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder, Scope};

use crate::broadcaster::Broadcaster;

pub fn service(path: &str) -> Scope {
    web::scope(path).route("", web::get().to(connect))
}

async fn connect(req: HttpRequest) -> impl Responder {
    let broadcaster: &Arc<Broadcaster> = req.app_data().unwrap();
    let stream = broadcaster.new_client().await;

    HttpResponse::Ok()
        .append_header(("cache-control", "no-cache"))
        .append_header(("content-type", "text/event-stream"))
        .streaming(stream)
}
