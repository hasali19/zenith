use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder, Scope};
use serde::Deserialize;

use crate::transcoder::{Job, Transcoder};

pub fn service(path: &str) -> Scope {
    web::scope(path).route("", web::post().to(transcode))
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
