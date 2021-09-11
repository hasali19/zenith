use std::sync::Arc;

use atium::query::QueryRequestExt;
use atium::respond::RespondRequestExt;
use atium::router::Router;
use atium::{endpoint, Request};
use serde::Deserialize;
use serde_json::json;

use crate::transcoder::{Job, Transcoder};

use super::error::bad_request;

pub fn routes(router: &mut Router) {
    router.route("/transcoder").get(get_state).post(transcode);
}

#[endpoint]
async fn get_state(req: &mut Request) -> eyre::Result<()> {
    let transcoder: &Arc<Transcoder> = req.ext().unwrap();
    let current = transcoder.current().await;
    let queue = transcoder.queue().await;

    req.ok().json(&json!({
        "current": current.map(|j| j.video_id),
        "queue": queue.iter().map(|j| j.video_id).collect::<Vec<_>>(),
    }))?;

    Ok(())
}

#[derive(Deserialize)]
pub struct TranscodeParams {
    #[serde(default)]
    video_id: Option<i64>,
    #[serde(default)]
    all: bool,
}

#[endpoint]
async fn transcode(req: &mut Request) -> eyre::Result<()> {
    let params: TranscodeParams = req.query()?;
    let transcoder: &Arc<Transcoder> = req.ext().unwrap();

    match params.video_id {
        Some(id) => transcoder.enqueue(Job { video_id: id }).await,
        None if params.all => transcoder.enqueue_all().await,
        None => return Err(bad_request("no video to transcode").into()),
    }

    req.ok();

    Ok(())
}
