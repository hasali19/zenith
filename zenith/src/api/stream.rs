use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Bytes;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use tokio::stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::config::Config;
use crate::db::Db;
use crate::ffmpeg::{Ffmpeg, TranscodeOptions};

use super::{ApiError, ApiResult};

pub fn service(path: &str) -> impl HttpServiceFactory {
    web::scope(path)
        .route("/{id}/original", web::get().to(get_original))
        .route("/{id}/transcode", web::get().to(get_transcoded_stream))
        .route("/{id}/info", web::get().to(get_stream_info))
        .default_service(web::route().to(HttpResponse::NotFound))
}

async fn get_original(path: web::Path<(i64,)>, db: Db) -> ApiResult<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = db.acquire().await?;

    let path: Option<(String,)> = sqlx::query_as("SELECT path FROM video_files WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await?;

    let path = match path {
        Some((path,)) => path,
        None => return Err(ApiError::NotFound),
    };

    Ok(NamedFile::open(path))
}

#[derive(serde::Deserialize)]
struct Query {
    #[serde(default)]
    start: u64,
}

async fn get_transcoded_stream(
    req: HttpRequest,
    path: web::Path<(i64,)>,
    query: web::Query<Query>,
    db: Db,
) -> ApiResult<impl Responder> {
    let (id,) = path.into_inner();
    let query = query.into_inner();
    let config: &Arc<Config> = req.app_data().unwrap();
    let mut conn = db.acquire().await?;

    let path: Option<(String,)> = sqlx::query_as("SELECT path FROM video_files WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await?;

    let path = match path {
        Some((path,)) => path,
        None => return Err(ApiError::NotFound),
    };

    let (mut tx, rx) = tokio::sync::mpsc::channel(1);

    let ffmpeg = Ffmpeg::new(config.ffmpeg_path());
    let mut child = ffmpeg.spawn_transcode(&TranscodeOptions {
        input_path: &path,
        start_time: query.start,
        use_hw_encoder: config.use_hw_encoder,
    })?;

    actix_web::rt::spawn(async move {
        let stdout = child.stdout.as_mut().unwrap();
        let mut stream = FramedRead::new(stdout, BytesCodec::new());

        while let Some(v) = stream.next().await {
            let v = v.map(Bytes::from).map_err(actix_web::Error::from);
            if tx.send(v).await.is_err() {
                child.kill().unwrap();
                break;
            }
        }

        child.await.unwrap();
    });

    Ok(HttpResponse::Ok().content_type("video/mp4").streaming(rx))
}

#[derive(serde::Serialize)]
struct StreamInfo {
    path: String,
    duration: f64,
}

async fn get_stream_info(path: web::Path<(i64,)>, db: Db) -> ApiResult<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT path, duration FROM video_files
        WHERE id = ?
    ";

    let (path, duration): (String, f64) = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(HttpResponse::Ok().json(StreamInfo { path, duration }))
}
