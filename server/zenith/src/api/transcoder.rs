use std::convert::Infallible;
use std::sync::Arc;

use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::web::{Json, Query};
use actix_web::{get, post, HttpResponse, Responder};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

use crate::api::error::bad_request;
use crate::api::ApiResult;
use crate::transcoder::{self, Job, Transcoder};
use crate::Ext;

#[derive(Serialize)]
struct State {
    queue: Vec<Job>,
}

#[get("/transcoder")]
pub async fn get_state(transcoder: Ext<Arc<Transcoder>>) -> ApiResult<impl Responder> {
    Ok(Json(State {
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
pub async fn get_events(transcoder: Ext<Arc<Transcoder>>) -> ApiResult<impl Responder> {
    let queue = transcoder.queue().await;

    let initial = {
        tokio_stream::once(Ok::<_, Infallible>(sse_data(&Event::InitialState {
            queue,
        })))
    };

    let events = BroadcastStream::new(transcoder.subscribe()).filter_map(|e| {
        let event = match e.ok()? {
            transcoder::Event::Queued(id) => Event::Queued { id },
            transcoder::Event::Started(id) => Event::Started { id },
            transcoder::Event::Progress(id, progress) => Event::Progress { id, progress },
            transcoder::Event::Success(id) => Event::Success { id },
            transcoder::Event::Error(id) => Event::Error { id },
        };

        Some(Ok::<_, Infallible>(sse_data(&event)))
    });

    Ok(HttpResponse::Ok()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .content_type(mime::TEXT_EVENT_STREAM)
        .streaming(initial.chain(events)))
}

fn sse_data(val: &impl Serialize) -> Bytes {
    let mut data = vec![];
    data.extend_from_slice(b"data: ");
    serde_json::to_writer(&mut data, val).unwrap();
    data.extend_from_slice(b"\n\n");
    data.into()
}

#[derive(Deserialize)]
pub struct TranscodeParams {
    #[serde(default)]
    video_id: Option<i64>,
    #[serde(default)]
    all: bool,
}

#[post("/transcoder")]
pub async fn transcode(
    query: Query<TranscodeParams>,
    transcoder: Ext<Arc<Transcoder>>,
) -> ApiResult<impl Responder> {
    match query.video_id {
        Some(id) => transcoder.enqueue(Job::new(id)).await,
        None if query.all => transcoder.enqueue_all().await,
        None => return Err(bad_request("no video to transcode")),
    }

    Ok(HttpResponse::Ok())
}
