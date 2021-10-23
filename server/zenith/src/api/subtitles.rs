use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::http::header::ContentType;
use actix_web::web::Path;
use actix_web::{delete, get, HttpResponse, Responder};
use mime::Mime;

use crate::api::error::bad_request;
use crate::api::ApiResult;
use crate::config::Config;
use crate::db::subtitles::SubtitlePath;
use crate::db::{self, Db};
use crate::{subtitles, Ext};

use super::ext::OptionExt;

#[get("/subtitles/{id}")]
async fn get_subtitle(
    id: Path<i64>,
    config: Ext<Arc<Config>>,
    db: Db,
) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;

    let subtitle = db::subtitles::get_by_id(&mut conn, *id)
        .await?
        .or_not_found("subtitle not found")?;

    let info = db::videos::get_basic_info(&mut conn, subtitle.video_id)
        .await?
        .or_not_found("video not found")?;

    let res = match subtitle.path {
        SubtitlePath::External { path } => SubtitleResponse::File(path.as_ref().into()),
        SubtitlePath::Embedded { index } => {
            let cached_path = config
                .subtitles
                .path
                .join(subtitle.video_id.to_string())
                .join(format!("{}.extracted.vtt", index));

            if cached_path.is_file() {
                // Return directly if embedded subtitle has already been extracted
                tracing::info!("using cached subtitle");
                SubtitleResponse::File(cached_path)
            } else {
                // Otherwise extract now
                // TODO: Cache the extracted subtitle?
                tracing::info!("extracting subtitle");
                SubtitleResponse::Bytes(
                    subtitles::extract_embedded(&config, &info.path, index).await?,
                )
            }
        }
    };

    Ok(res)
}

enum SubtitleResponse {
    File(PathBuf),
    Bytes(Vec<u8>),
}

impl Responder for SubtitleResponse {
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        match self {
            SubtitleResponse::File(path) => NamedFile::open(path).unwrap().respond_to(req),
            SubtitleResponse::Bytes(bytes) => HttpResponse::Ok()
                .insert_header(ContentType(Mime::from_str("text/vtt").unwrap()))
                .body(bytes),
        }
    }
}

#[delete("/subtitles/{id}")]
pub async fn delete_subtitle(id: Path<i64>, db: Db) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;

    let subtitle = db::subtitles::get_by_id(&mut conn, *id)
        .await?
        .or_not_found("subtitle not found")?;

    match subtitle.path {
        SubtitlePath::Embedded { .. } => {
            return Err(bad_request("embedded subtitles cannot be deleted"))
        }
        SubtitlePath::External { path } => {
            db::subtitles::delete(&mut conn, *id).await?;
            std::fs::remove_file(path.as_ref())?;
        }
    }

    Ok(HttpResponse::Ok())
}
