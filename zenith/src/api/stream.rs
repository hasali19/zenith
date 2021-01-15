use actix_files::NamedFile;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db::Db;
use crate::transcoder::Transcoder;

use super::{ApiError, ApiResult};

pub fn service(path: &str) -> impl HttpServiceFactory {
    web::scope(path)
        .route("/{id}/original", web::get().to(get_original))
        .route("/{id}/hls", web::get().to(get_hls_playlist))
        .route("/{id}/hls/{segment}.ts", web::get().to(get_hls_segment))
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

async fn get_hls_playlist(path: web::Path<(i64,)>, db: Db) -> ApiResult<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = db.acquire().await?;

    let (_path, duration): (String, f64) =
        sqlx::query_as("SELECT path, duration FROM video_files WHERE id = ?")
            .bind(id)
            .fetch_optional(&mut conn)
            .await?
            .ok_or(ApiError::NotFound)?;

    let segments = (duration / 3.0).ceil() as i32;
    let mut playlist = String::new();

    playlist.push_str("#EXTM3U\n");
    playlist.push_str("#EXT-X-PLAYLIST-TYPE:VOD\n");
    playlist.push_str("#EXT-X-VERSION:3\n");
    playlist.push_str("#EXT-X-TARGETDURATION:3\n");
    playlist.push_str("#EXT-X-MEDIA-SEQUENCE:0\n");

    for i in 0..segments {
        let length = if i == segments - 1 {
            3.0 * (1 - segments) as f64 + duration
        } else {
            3.0
        };

        playlist.push_str(&format!("#EXTINF:{:.8},\n", length));
        playlist.push_str(&format!("hls/{}.ts\n", i));
    }

    playlist.push_str("#EXT-X-ENDLIST\n");

    Ok(HttpResponse::Ok()
        .content_type("application/x-mpegURL")
        .header("Access-Control-Allow-Origin", "*")
        .body(playlist))
}

async fn get_hls_segment(
    req: HttpRequest,
    path: web::Path<(i64, u32)>,
    mut transcoder: Transcoder,
) -> ApiResult<impl Responder> {
    let (id, segment) = path.into_inner();
    let segment_path = transcoder.transcode_segment(id, segment).await;

    Ok(NamedFile::open(segment_path)?
        .into_response(&req)?
        .with_header("Access-Control-Allow-Origin", "*")
        .respond_to(&req)
        .await)
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
