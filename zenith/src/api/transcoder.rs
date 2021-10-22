use std::convert::Infallible;
use std::sync::Arc;

use atium::headers::{CacheControl, ContentType};
use atium::query::QueryRequestExt;
use atium::respond::RespondRequestExt;
use atium::responder::Json;
use atium::router::Router;
use atium::{endpoint, Body, Request, Responder, StatusCode};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

use crate::transcoder::{self, Job, Transcoder};

use super::error::bad_request;

#[derive(Serialize)]
struct State {
    queue: Vec<Job>,
}

pub fn routes(router: &mut Router) {
    router.route("/transcoder").get(get_state).post(transcode);
    router.route("/transcoder/events").get(get_events);
}

#[endpoint]
async fn get_state(req: &mut Request) -> eyre::Result<impl Responder> {
    let transcoder: &Arc<Transcoder> = req.ext().unwrap();
    let queue = transcoder.queue().await;
    Ok(Json(State { queue }))
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

#[endpoint]
async fn get_events(req: &mut Request) -> eyre::Result<()> {
    let transcoder: &Arc<Transcoder> = req.ext().unwrap();

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

    req.ok()
        .header(CacheControl::new().with_no_cache())
        .header(ContentType::from(mime::TEXT_EVENT_STREAM))
        .body(Body::wrap_stream(initial.chain(events)));

    Ok(())
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

#[endpoint]
async fn transcode(req: &mut Request) -> eyre::Result<impl Responder> {
    let params: TranscodeParams = req.query()?;
    let transcoder: &Arc<Transcoder> = req.ext().unwrap();

    match params.video_id {
        Some(id) => transcoder.enqueue(Job::new(id)).await,
        None if params.all => transcoder.enqueue_all().await,
        None => return Err(bad_request("no video to transcode").into()),
    }

    Ok(StatusCode::OK)
}
