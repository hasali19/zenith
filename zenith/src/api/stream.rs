use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Responder};

use crate::db::Db;
use crate::ffmpeg;

use super::{ApiError, ApiResult};

pub fn service(path: &str) -> impl HttpServiceFactory {
    web::scope(path)
        .route("/{id}", web::get().to(get_stream))
        .route("/{id}/info", web::get().to(get_stream_info))
}

#[derive(serde::Deserialize)]
struct StreamQuery {
    #[serde(default)]
    start: f64,
}

async fn get_stream(
    path: web::Path<(i64,)>,
    query: web::Query<StreamQuery>,
    db: Db,
) -> ApiResult<impl Responder> {
    let (id,) = path.into_inner();
    let query = query.into_inner();
    let mut conn = db.acquire().await?;

    let path: Option<(String,)> = sqlx::query_as("SELECT path FROM video_files WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await?;

    let path = match path {
        Some((path,)) => path,
        None => return Err(ApiError::NotFound),
    };

    let stream = ffmpeg::begin_transcode(query.start, path);

    Ok(HttpResponse::Ok().streaming(stream))
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
