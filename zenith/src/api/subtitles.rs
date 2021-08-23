use std::process::Stdio;
use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpRequest, HttpResponse};
use tokio::process::Command;

use crate::config::Config;
use crate::db::{self, Db};
use crate::ext::CommandExt;

use super::ApiResult;

pub fn configure(config: &mut ServiceConfig) {
    config.route("/subtitles/{id}", web::get().to(get_subtitle));
}

async fn get_subtitle(req: HttpRequest, path: web::Path<i64>) -> ApiResult {
    let subtitle_id = path.into_inner();

    let config: &Arc<Config> = req.app_data().unwrap();
    let db: &Db = req.app_data().unwrap();

    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let subtitle = db::subtitles::get_by_id(&mut conn, subtitle_id)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("subtitle not found"))?;

    let path = db::videos::get_path(&mut conn, subtitle.video_id)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("video not found"))?;

    match subtitle.path {
        db::subtitles::SubtitlePath::External(path) => {
            // Subtitle is an external file, return it directly
            Ok(NamedFile::open(path.as_ref())?.into_response(&req))
        }
        db::subtitles::SubtitlePath::Embedded(index) => {
            // Subtitle is embedded, extract it from the video file
            Ok(HttpResponse::Ok()
                .content_type("text/vtt")
                .append_header(("access-control-allow-origin", "*"))
                .body(extract_embedded_subtitle(config, &path, index).await?))
        }
    }
}

async fn extract_embedded_subtitle(
    config: &Config,
    path: &str,
    index: u32,
) -> std::io::Result<Vec<u8>> {
    Command::new(&config.transcoding.ffmpeg_path)
        .arg_pair("-i", &path)
        .arg_pair("-map", format!("0:{}", index))
        .arg_pair("-c:s", "webvtt")
        .arg_pair("-f", "webvtt")
        .arg("pipe:1")
        .stdout(Stdio::piped())
        .output()
        .await
        .map(|output| output.stdout)
}
