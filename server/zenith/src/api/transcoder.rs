use std::sync::Arc;

use axum::extract::{Extension, Query};
use axum::http::StatusCode;
use axum::response::{sse, IntoResponse};
use axum::Json;
use axum_codegen::{get, post};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

use crate::api::error::bad_request;
use crate::api::ApiResult;
use crate::transcoder::{self, Job, Transcoder};

#[derive(Serialize, JsonSchema)]
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
struct ProgressEvent {
    id: i64,
    progress: f64,
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

#[derive(Deserialize)]
pub struct TranscodeParams {
    #[serde(default)]
    video_id: Option<i64>,
    #[serde(default)]
    all: bool,
}

#[post("/transcoder")]
#[query(name = "video_id", model = Option<i64>)]
#[query(name = "all", model = bool)]
#[response(status = 200)]
pub async fn transcode(
    query: Query<TranscodeParams>,
    transcoder: Extension<Arc<Transcoder>>,
) -> ApiResult<impl IntoResponse> {
    match query.video_id {
        Some(id) => transcoder.enqueue(Job::new(id)).await,
        None if query.all => transcoder.enqueue_all().await,
        None => return Err(bad_request("no video to transcode")),
    }

    Ok(StatusCode::OK)
}
