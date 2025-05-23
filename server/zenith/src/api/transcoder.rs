use std::sync::Arc;

use axum::Json;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::{IntoResponse, sse};
use serde::{Deserialize, Serialize};
use serde_qs::axum::QsQuery;
use speq::Reflect;
use speq::axum::{get, post};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

use crate::api::ApiResult;
use crate::api::error::bad_request;
use crate::transcoder::{self, Job, Transcoder};

#[derive(Serialize, Reflect)]
struct TranscoderState {
    queue: Vec<Job>,
}

#[get("/transcoder")]
#[response(model = TranscoderState)]
pub async fn get_state(transcoder: Extension<Arc<Transcoder>>) -> ApiResult<impl IntoResponse> {
    Ok(Json(TranscoderState {
        queue: transcoder.queue().await,
    }))
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Event {
    InitialState { queue: Vec<Job> },
    Queued { id: i64 },
    Started { id: i64 },
    Progress { id: i64, progress: f64 },
    Success { id: i64 },
    Error { id: i64 },
}

#[get("/transcoder/events")]
#[response(status = 200)]
pub async fn get_events(transcoder: Extension<Arc<Transcoder>>) -> ApiResult<impl IntoResponse> {
    let queue = transcoder.queue().await;

    let initial =
        tokio_stream::once(sse::Event::default().json_data(Event::InitialState { queue }));

    let events = BroadcastStream::new(transcoder.subscribe()).filter_map(|e| {
        let event = match e.ok()? {
            transcoder::Event::Queued(id) => Event::Queued { id },
            transcoder::Event::Started(id) => Event::Started { id },
            transcoder::Event::Progress(id, progress) => Event::Progress { id, progress },
            transcoder::Event::Success(id) => Event::Success { id },
            transcoder::Event::Error(id) => Event::Error { id },
        };

        Some(sse::Event::default().json_data(event))
    });

    Ok(sse::Sse::new(initial.chain(events)))
}

#[derive(Deserialize, Reflect)]
pub struct TranscodeParams {
    #[serde(default)]
    video_id: Option<i64>,
    #[serde(default)]
    all: bool,
}

#[post("/transcoder")]
#[response(status = 200)]
pub async fn transcode(
    query: QsQuery<TranscodeParams>,
    transcoder: Extension<Arc<Transcoder>>,
) -> ApiResult<impl IntoResponse> {
    match query.video_id {
        Some(id) => transcoder.enqueue(Job::new(id)).await,
        None if query.all => transcoder.enqueue_all().await,
        None => return Err(bad_request("no video to transcode")),
    }

    Ok(StatusCode::OK)
}
