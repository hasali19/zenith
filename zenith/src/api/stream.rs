use futures::StreamExt;
use hyper::{header, Body, Response, StatusCode};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::ffmpeg::{Ffmpeg, TranscodeOptions};
use crate::server::{App, JsonResponse, Request};
use crate::AppState;

use super::{ApiError, ApiResult};

pub fn configure(app: &mut App<AppState>) {
    app.get("/api/stream/:id/original", get_original);
    app.get("/api/stream/:id/transcode", get_transcoded_stream);
    app.get("/api/stream/:id/info", get_stream_info);
}

async fn get_original(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await.unwrap();

    let (path,): (String,) = sqlx::query_as("SELECT path FROM video_files WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(|_| ApiError::internal_server_error())?
        .ok_or_else(ApiError::not_found)?;

    let file = File::open(path)
        .await
        .map_err(|_| ApiError::internal_server_error())?;

    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);

    Ok(Response::new(body))
}

#[derive(serde::Deserialize)]
struct Query {
    #[serde(default)]
    start: u64,
}

async fn get_transcoded_stream(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let query: Query = req.query().map_err(|_| ApiError::bad_request())?;
    let mut conn = state.db.acquire().await.unwrap();

    let (path,): (String,) = sqlx::query_as("SELECT path FROM video_files WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(|_| ApiError::internal_server_error())?
        .ok_or_else(ApiError::not_found)?;

    let options = TranscodeOptions {
        input_path: &path,
        start_time: query.start,
        use_hw_encoder: state.config.use_hw_encoder,
    };

    let ffmpeg = Ffmpeg::new(state.config.ffmpeg_path());
    let mut child = ffmpeg
        .spawn_transcode(&options)
        .map_err(|e| ApiError::internal_server_error().body(e.to_string()))?;

    let (mut sender, body) = Body::channel();

    tokio::spawn(async move {
        let stdout = child.stdout.as_mut().unwrap();
        let mut stream = FramedRead::new(stdout, BytesCodec::new());

        while let Some(Ok(v)) = stream.next().await {
            if sender.send_data(v.into()).await.is_err() {
                log::warn!("client has disconnected, killing ffmpeg");
                child.kill().await.unwrap();
                return;
            }
        }

        child.wait().await.unwrap();
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "video/mp4")
        .body(body)
        .unwrap())
}

#[derive(serde::Serialize)]
struct StreamInfo {
    path: String,
    duration: f64,
}

async fn get_stream_info(state: AppState, req: Request) -> ApiResult<JsonResponse> {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await.unwrap();

    let sql = "
        SELECT path, duration FROM video_files
        WHERE id = ?
    ";

    let (path, duration): (String, f64) = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(|_| ApiError::internal_server_error())?
        .ok_or_else(ApiError::not_found)?;

    Ok(StreamInfo { path, duration }.into())
}
