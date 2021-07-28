use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::{json, Value};

use crate::config::Config;
use crate::db::media::MediaItemType;
use crate::db::subtitles::{Subtitle, SubtitlePath};
use crate::db::{self, Db};
use crate::ffprobe::Ffprobe;

use super::ApiResult;

pub fn configure(config: &mut ServiceConfig) {
    config
        .route("/videos/{id}", web::get().to(get_video_content))
        .route("/videos/{id}/info", web::get().to(get_video_info))
        .route("/videos/{id}/subtitles", web::get().to(get_subtitles));
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
        SELECT file.path, item.item_type, data.position
        FROM video_files AS file
        JOIN media_items AS item ON item.id = file.item_id
        LEFT JOIN user_item_data AS data ON file.item_id = data.item_id
        WHERE file.item_id = ?
    ";

    let (path, item_type, position): (String, MediaItemType, Option<f64>) = sqlx::query_as(sql)
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

    let subtitles = db::subtitles::get_for_video(&mut conn, id)
        .await
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(subtitle_to_json);

    Ok(HttpResponse::Ok().json(&json!({
        "path": path,
        "type": item_type,
        "format": info.format.format_name,
        "duration": info.format.duration.parse::<f64>().unwrap(),
        "position": position,
        "video": video,
        "audio": audio,
        "subtitles": subtitles.collect::<Vec<_>>(),
    })))
}

async fn get_subtitles(req: HttpRequest, path: web::Path<(i64,)>) -> ApiResult {
    let (id,) = path.into_inner();

    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let subtitles = db::subtitles::get_for_video(&mut conn, id)
        .await
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(subtitle_to_json);

    Ok(HttpResponse::Ok().json(&subtitles.collect::<Vec<_>>()))
}

fn subtitle_to_json(subtitle: Subtitle) -> Value {
    let (subtitle_type, path_key, path_val) = match &subtitle.path {
        SubtitlePath::External(path) => ("external", "path", json!(path.as_ref())),
        SubtitlePath::Embedded(index) => ("embedded", "index", json!(index)),
    };

    json!({
        "id": subtitle.id,
        "title": subtitle.title,
        "language": subtitle.language,
        "type": subtitle_type,
        path_key: path_val,
    })
}
