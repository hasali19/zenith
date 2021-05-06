use std::sync::Arc;

use actix_files::NamedFile;
use actix_http::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::{web, HttpRequest, HttpResponse, Responder, Scope};
use serde_json::json;

use crate::config::Config;
use crate::db::Db;
use crate::ffprobe::Ffprobe;

pub fn service(path: &str) -> Scope {
    web::scope(path)
        .route("/{id}", web::get().to(get_video_content))
        .route("/{id}/info", web::get().to(get_video_info))
}

async fn get_video_content(
    req: HttpRequest,
    path: web::Path<(i64,)>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();

    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let sql = "
        SELECT path
        FROM video_files
        WHERE item_id = ?
    ";

    let path: String = sqlx::query_scalar(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("video not found"))?;

    Ok(NamedFile::open(path))
}

async fn get_video_info(
    req: HttpRequest,
    path: web::Path<(i64,)>,
) -> actix_web::Result<impl Responder> {
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
        .ok_or_else(|| ErrorNotFound("video not found"))?;

    let info = Ffprobe::new(&config.transcoding.ffprobe_path)
        .probe(&path)
        .await
        .map_err(ErrorInternalServerError)?;

    let video = info
        .streams
        .iter()
        .find(|stream| stream.codec_type == "video")
        .map(|stream| {
            json!({
                "codec": stream.codec_name,
                "profile": stream.properties.get("profile").unwrap().as_str().unwrap(),
                "width": stream.properties.get("width").and_then(|v| v.as_u64()).unwrap(),
                "height": stream.properties.get("height").and_then(|v| v.as_u64()).unwrap(),
            })
        });

    let audio = info
        .streams
        .iter()
        .find(|stream| stream.codec_type == "audio")
        .map(|stream| {
            json!({
                "codec": stream.codec_name,
            })
        });

    Ok(HttpResponse::Ok().json(&json!({
        "path": path,
        "format": info.format.format_name,
        "duration": info.format.duration.parse::<f64>().unwrap(),
        "position": position,
        "video": video,
        "audio": audio,
    })))
}
