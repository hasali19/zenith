use std::io::SeekFrom;
use std::ops::Bound;
use std::str::FromStr;

use futures::StreamExt;
use mime::Mime;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::process::Child;
use tokio_util::codec::{BytesCodec, FramedRead};
use zenith_http::headers::{AcceptRanges, ContentLength, ContentRange, ContentType, Range};
use zenith_http::{App, Body, Request, Response, StatusCode};

use crate::ffmpeg::{Ffmpeg, Ffprobe, SubtitleOptions, TranscodeOptions};
use crate::AppState;

use super::{ApiError, ApiResult};

pub fn configure(app: &mut App<AppState>) {
    app.get("/api/stream/:id/original", get_original);
    app.get("/api/stream/:id/transcode", get_transcoded_stream);
    app.get("/api/stream/:id/subtitles/:index", get_subtitles_stream);
    app.get("/api/stream/:id/info", get_info);
}

async fn get_original(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

    let path: String = sqlx::query_scalar("SELECT path FROM video_files WHERE item_id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let mut file = File::open(path)
        .await
        .map_err(|_| ApiError::internal_server_error())?;

    let length = file.metadata().await.unwrap().len();
    let range = req.header::<Range>().and_then(|range| range.iter().next());

    if let Some((from, to)) = range {
        let from = match from {
            Bound::Included(n) => n,
            Bound::Excluded(n) => n + 1,
            Bound::Unbounded => 0,
        };

        let to = match to {
            Bound::Included(n) => n,
            Bound::Excluded(n) => n - 1,
            Bound::Unbounded => length - 1,
        };

        file.seek(SeekFrom::Start(from))
            .await
            .map_err(|e| ApiError::internal_server_error().body(e.to_string()))?;

        let total_length = length;
        let length = u64::min(length - from, to - from + 1);
        let reader = file.take(length);
        let stream = FramedRead::new(reader, BytesCodec::new());
        let body = Body::wrap_stream(stream);

        Ok(Response::new()
            .with_status(StatusCode::PARTIAL_CONTENT)
            .with_header(AcceptRanges::bytes())
            .with_header(ContentLength(length))
            .with_header(ContentRange::bytes(from..=from + length - 1, total_length).unwrap())
            .with_body(body))
    } else {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);

        Ok(Response::new()
            .with_header(AcceptRanges::bytes())
            .with_header(ContentLength(length))
            .with_body(body))
    }
}

#[derive(serde::Deserialize)]
struct TranscodeQuery {
    #[serde(default)]
    start: u64,
}

async fn get_transcoded_stream(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let query: TranscodeQuery = req.query().map_err(|_| ApiError::bad_request())?;
    let mut conn = state.db.acquire().await?;

    let path: String = sqlx::query_scalar("SELECT path FROM video_files WHERE item_id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let config = &state.config.transcoding;
    let ffmpeg = Ffmpeg::new(&config.ffmpeg_path);
    let options = TranscodeOptions {
        input_path: &path,
        start_time: query.start,
    };

    let child = ffmpeg
        .transcode(&options)
        .map_err(|e| ApiError::internal_server_error().body(e.to_string()))?;

    Ok(Response::new()
        .with_header(ContentType::from(Mime::from_str("video/mp4").unwrap()))
        .with_body(stream_stdout(child)))
}

#[derive(serde::Deserialize)]
struct SubtitlesQuery {
    #[serde(default)]
    start: u64,
}

async fn get_subtitles_stream(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let stream_index: u32 = req
        .param("index")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let query: SubtitlesQuery = req.query().map_err(|_| ApiError::bad_request())?;
    let mut conn = state.db.acquire().await?;

    let path: String = sqlx::query_scalar("SELECT path FROM video_files WHERE item_id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let config = &state.config.transcoding;
    let ffmpeg = Ffmpeg::new(&config.ffmpeg_path);
    let options = SubtitleOptions {
        input_path: &path,
        start_time: query.start,
        stream_index,
    };

    let child = ffmpeg
        .extract_subtitles(&options)
        .map_err(|e| ApiError::internal_server_error().body(e.to_string()))?;

    Ok(Response::new()
        .with_header(ContentType::from(Mime::from_str("text/vtt").unwrap()))
        .with_body(stream_stdout(child)))
}

#[derive(serde::Serialize)]
struct StreamInfo {
    duration: f64,
    subtitles: Vec<SubtitleInfo>,
}

#[derive(serde::Serialize)]
struct SubtitleInfo {
    index: u32,
    title: Option<String>,
    language: Option<String>,
}

async fn get_info(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

    let path: String = sqlx::query_scalar("SELECT path FROM video_files WHERE item_id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let config = &state.config.transcoding;
    let ffprobe = Ffprobe::new(&config.ffprobe_path);
    let info = ffprobe.get_video_info(&path).await?;

    let duration = info
        .format
        .duration
        .parse::<f64>()
        .map_err(|e| ApiError::internal_server_error().body(e.to_string()))?;

    let subtitles = info
        .streams
        .into_iter()
        .filter(|stream| stream.codec_type == "subtitle")
        .map(|mut stream| SubtitleInfo {
            index: stream.index,
            title: stream.tags.remove("title"),
            language: stream.tags.remove("language"),
        })
        .collect::<Vec<_>>();

    Ok(Response::new().json(&StreamInfo {
        duration,
        subtitles,
    })?)
}

fn stream_stdout(mut child: Child) -> Body {
    let (mut sender, body) = Body::channel();

    tokio::spawn(async move {
        let stdout = child.stdout.as_mut().unwrap();
        let mut stream = FramedRead::new(stdout, BytesCodec::new());

        while let Some(Ok(v)) = stream.next().await {
            if sender.send_data(v.into()).await.is_err() {
                log::warn!("client has disconnected, killing child process");
                child.kill().await.unwrap();
                return;
            }
        }

        child.wait().await.unwrap();
    });

    body
}
