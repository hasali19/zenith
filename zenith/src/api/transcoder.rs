use std::sync::Arc;

use actix_web::error::ErrorBadRequest;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::transcoder::{Job, Transcoder};

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        web::resource("/transcoder")
            .route(web::get().to(get_state))
            .route(web::post().to(transcode)),
    );
}

async fn get_state(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let transcoder: &Arc<Transcoder> = req.app_data().unwrap();
    let current = transcoder.current().await;
    let queue = transcoder.queue().await;

    Ok(HttpResponse::Ok().json(json!({
        "current": current.map(|j| j.video_id),
        "queue": queue.iter().map(|j| j.video_id).collect::<Vec<_>>(),
    })))
}

#[derive(Deserialize)]
pub struct TranscodeParams {
    #[serde(default)]
    video_id: Option<i64>,
    #[serde(default)]
    all: bool,
}

async fn transcode(
    req: HttpRequest,
    query: web::Query<TranscodeParams>,
) -> actix_web::Result<impl Responder> {
    let params = query.into_inner();
    let transcoder: &Arc<Transcoder> = req.app_data().unwrap();

    match params.video_id {
        Some(id) => transcoder.enqueue(Job { video_id: id }).await,
        None if params.all => transcoder.enqueue_all().await,
        None => return Err(ErrorBadRequest("no video to transcode")),
    }

    Ok(HttpResponse::Ok())
}
