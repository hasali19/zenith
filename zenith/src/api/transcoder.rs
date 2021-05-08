use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder, Scope};
use serde::Deserialize;
use serde_json::json;

use crate::transcoder::{Job, Transcoder};

pub fn service(path: &str) -> Scope {
    web::scope(path).service(
        web::resource("")
            .route(web::get().to(get_state))
            .route(web::post().to(transcode)),
    )
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
    video_id: i64,
}

async fn transcode(
    req: HttpRequest,
    query: web::Query<TranscodeParams>,
) -> actix_web::Result<impl Responder> {
    let params = query.into_inner();
    let transcoder: &Arc<Transcoder> = req.app_data().unwrap();

    transcoder
        .enqueue(Job {
            video_id: params.video_id,
        })
        .await;

    Ok(HttpResponse::Ok())
}
