use std::sync::Arc;

use actix_files::NamedFile;
use actix_http::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::{web, HttpRequest, HttpResponse, Responder, Scope};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use tokio::process::Child;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::config::Config;
use crate::db::Db;
use crate::ffmpeg::{Ffmpeg, Ffprobe, TranscodeOptions};

pub fn service(path: &str) -> Scope {
    web::scope(path)
        .route("/{id}/original", web::get().to(get_original))
        .route("/{id}/transcode", web::get().to(get_transcoded_stream))
        .route("/{id}/info", web::get().to(get_info))
}

async fn get_original(
    req: HttpRequest,
    path: web::Path<(i64,)>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();

    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let path: String = sqlx::query_scalar("SELECT path FROM video_files WHERE item_id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(""))?;

    Ok(NamedFile::open(path))
}

#[derive(serde::Deserialize)]
struct TranscodeQuery {
    #[serde(default)]
    start: u64,
}

async fn get_transcoded_stream(
    req: HttpRequest,
    path: web::Path<(i64,)>,
    query: web::Query<TranscodeQuery>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();
    let query = query.into_inner();

    let config: &Arc<Config> = req.app_data().unwrap();
    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let path: String = sqlx::query_scalar("SELECT path FROM video_files WHERE item_id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(""))?;

    let config = &config.transcoding;
    let info = Ffprobe::new(&config.ffprobe_path)
        .get_video_info(&path)
        .await
        .map_err(ErrorInternalServerError)?;

    let mut transcode_video = false;
    let video_stream = info
        .streams
        .iter()
        .find(|stream| stream.codec_type == "video");

    if let Some(video_stream) = video_stream {
        let codec = &video_stream.codec_name;
        if codec == "h264" {
            tracing::info!("copying existing h264 video stream");
        } else {
            transcode_video = true;
            tracing::info!("transcoding video due to unsupported codec ({})", codec);
        }
    }

    let ffmpeg = Ffmpeg::new(&config.ffmpeg_path);
    let options = TranscodeOptions {
        input_path: &path,
        start_time: query.start,
        transcode_video,
        use_hw_encoder: config.use_hw_encoder,
    };

    let child = ffmpeg
        .transcode(&options)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("video/mp4")
        .streaming(stream_stdout(child)))
}

#[derive(serde::Serialize)]
struct StreamInfo {
    duration: f64,
    position: Option<f64>,
    subtitles: Vec<SubtitleInfo>,
}

#[derive(serde::Serialize)]
struct SubtitleInfo {
    index: u32,
    title: Option<String>,
    language: Option<String>,
}

async fn get_info(req: HttpRequest, path: web::Path<(i64,)>) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();

    let config: &Arc<Config> = req.app_data().unwrap();
    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let sql = "
        SELECT file.path, data.position
        FROM video_files AS file
        LEFT JOIN user_item_data AS data ON file.item_id = data.item_id
        WHERE file.item_id = ?
    ";

    let (path, position): (String, Option<f64>) = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(""))?;

    let config = &config.transcoding;
    let ffprobe = Ffprobe::new(&config.ffprobe_path);
    let info = ffprobe
        .get_video_info(&path)
        .await
        .map_err(ErrorInternalServerError)?;

    let duration = info
        .format
        .duration
        .parse::<f64>()
        .map_err(ErrorInternalServerError)?;

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

    Ok(HttpResponse::Ok().json(&StreamInfo {
        duration,
        position,
        subtitles,
    }))
}

fn stream_stdout(mut child: Child) -> impl Stream<Item = actix_web::Result<Bytes>> {
    let (sender, receiver) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        let stdout = child.stdout.as_mut().unwrap();
        let mut stream = FramedRead::new(stdout, BytesCodec::new());

        while let Some(Ok(v)) = stream.next().await {
            if sender.send(v.into()).await.is_err() {
                tracing::warn!("client has disconnected, killing child process");
                child.kill().await.unwrap();
                return;
            }
        }

        child.wait().await.unwrap();
    });

    ReceiverStream::new(receiver).map(Ok::<_, actix_web::Error>)
}
