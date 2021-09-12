use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::config::Config;
use crate::db;
use crate::db::subtitles::NewSubtitle;
use crate::db::subtitles::SubtitlePath;
use crate::db::Db;
use crate::library::scanner::{self, LibraryScanner, ScanOptions, VideoFileType};

use super::ApiResult;

pub fn configure(config: &mut ServiceConfig) {
    config
        .route("/import/queue", web::get().to(get_import_queue))
        .route("/import/subtitles", web::post().to(import_subtitle));
}

async fn get_import_queue(req: HttpRequest) -> ApiResult {
    let config: &Arc<Config> = req.app_data().unwrap();
    let import_path = match config.import.path.as_deref() {
        Some(path) => path,
        None => return Ok(HttpResponse::Ok().json(vec![(); 0])),
    };

    let mut entries = vec![];

    for entry in scanner::get_video_files(import_path) {
        let name = entry.file_name().to_str().unwrap();
        let path = entry.path().to_str().unwrap();
        let info = scanner::parse_video_filename(&config.import.matchers, name);

        entries.push(json!({
            "name": name,
            "path": path,
            "info": info,
        }));
    }

    Ok(HttpResponse::Ok().json(entries))
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ImportSource {
    Local { path: String, copy: Option<bool> },
}

#[derive(Deserialize)]
pub struct ImportSubtitleRequest {
    source: ImportSource,
    video_id: i64,
    title: Option<String>,
    language: Option<String>,
}

async fn import_subtitle(
    req: HttpRequest,
    data: web::Json<ImportSubtitleRequest>,
) -> ApiResult<impl Responder> {
    let data = data.into_inner();
    let config: &Arc<Config> = req.app_data().unwrap();
    let db: &Db = req.app_data().unwrap();

    let (src_path, copy) = match data.source {
        ImportSource::Local { path, copy } => (path, copy.unwrap_or(true)),
    };

    let src_path = PathBuf::from(src_path);
    let src_ext = src_path
        .extension()
        .ok_or_else(|| ErrorBadRequest("source file has no extension"))?;

    let dst_name = Uuid::new_v4().to_string();
    let dst_path = config.subtitles.path.join(dst_name).with_extension(src_ext);

    if dst_path.exists() {
        return Err(ErrorBadRequest(format!("{:?} already exists", dst_path)));
    }

    let mut transaction = db.begin().await.map_err(ErrorInternalServerError)?;
    let subtitles = NewSubtitle {
        video_id: data.video_id,
        path: SubtitlePath::External(Cow::Borrowed(dst_path.to_str().unwrap())),
        title: data.title.as_deref(),
        language: data.language.as_deref(),
    };

    db::subtitles::insert(&mut transaction, &subtitles)
        .await
        .map_err(ErrorInternalServerError)?;

    if !config.subtitles.path.exists() {
        std::fs::create_dir_all(&config.subtitles.path).map_err(ErrorInternalServerError)?;
    }

    if copy {
        tracing::info!("copying {:?} to {:?}", src_path, dst_path);
        std::fs::copy(&src_path, &dst_path).map_err(ErrorInternalServerError)?;
    } else {
        tracing::info!("moving {:?} to {:?}", src_path, dst_path);
        std::fs::rename(&src_path, &dst_path).map_err(ErrorInternalServerError)?;
    }

    transaction
        .commit()
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok())
}

#[derive(Deserialize)]
pub struct ImportMovieRequest {
    source: ImportSource,
    title: String,
    year: u32,
}

pub async fn import_movie(
    req: HttpRequest,
    data: web::Json<ImportMovieRequest>,
) -> ApiResult<impl Responder> {
    let data = data.into_inner();
    let config: &Arc<Config> = req.app_data().unwrap();
    let scanner: &Arc<LibraryScanner> = req.app_data().unwrap();

    let src_path = match data.source {
        ImportSource::Local { path, copy: _ } => path,
    };

    let src_path = PathBuf::from(src_path);
    let src_ext = src_path
        .extension()
        .ok_or_else(|| ErrorBadRequest("source file has no extension"))?;

    let dst_name = format!("{} ({})", data.title, data.year);
    let dst_dir = Path::new(&config.libraries.movies).join(&dst_name);

    if dst_dir.exists() {
        return Err(ErrorBadRequest(format!("{:?} already exists", dst_dir)));
    }

    let dst_path = dst_dir.join(dst_name).with_extension(src_ext);

    tracing::info!("moving {:?} to {:?}", src_path, dst_path);
    std::fs::create_dir(&dst_dir).map_err(ErrorInternalServerError)?;
    std::fs::rename(&src_path, &dst_path).map_err(ErrorInternalServerError)?;

    scanner
        .scan_file(VideoFileType::Movie, &dst_path, ScanOptions::quick())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok())
}

#[derive(Deserialize)]
pub struct ImportShowRequest {
    name: String,
    episodes: Vec<ImportEpisodeRequest>,
}

pub async fn import_show(
    req: HttpRequest,
    data: web::Json<ImportShowRequest>,
) -> ApiResult<impl Responder> {
    let data = data.into_inner();
    let config: &Arc<Config> = req.app_data().unwrap();
    let scanner: &Arc<LibraryScanner> = req.app_data().unwrap();

    if data.episodes.is_empty() {
        return Err(ErrorBadRequest("show must have at least one episode"));
    }

    let show_path = Path::new(&config.libraries.tv_shows).join(&data.name);
    if show_path.exists() {
        return Err(ErrorBadRequest(format!("{:?} already exists", show_path)));
    }

    std::fs::create_dir(&show_path).map_err(ErrorInternalServerError)?;

    for episode in data.episodes {
        let path = inner::import_episode(&show_path, episode).await?;
        scanner
            .scan_file(VideoFileType::Episode, &path, ScanOptions::quick())
            .await
            .map_err(ErrorInternalServerError)?;
    }

    Ok(HttpResponse::Ok())
}

#[derive(Deserialize)]
pub struct ImportEpisodeRequest {
    source: ImportSource,
    season_number: u32,
    episode_number: u32,
}

pub async fn import_episode(
    req: HttpRequest,
    path: web::Path<(i64,)>,
    data: web::Json<ImportEpisodeRequest>,
) -> ApiResult<impl Responder> {
    let (show_id,) = path.into_inner();
    let data = data.into_inner();
    let db: &Db = req.app_data().unwrap();
    let scanner: &Arc<LibraryScanner> = req.app_data().unwrap();

    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;
    let show_path: String = sqlx::query_scalar("SELECT path from tv_shows WHERE item_id = ?")
        .bind(show_id)
        .fetch_optional(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("show not found"))?;

    let path = inner::import_episode(Path::new(&show_path), data).await?;

    scanner
        .scan_file(VideoFileType::Episode, &path, ScanOptions::quick())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok())
}

mod inner {
    use std::path::{Path, PathBuf};

    use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};

    use crate::api::ApiResult;

    use super::{ImportEpisodeRequest, ImportSource};

    pub async fn import_episode(show_path: &Path, req: ImportEpisodeRequest) -> ApiResult<PathBuf> {
        let src_path = match req.source {
            ImportSource::Local { path, copy: _ } => path,
        };

        let src_path = PathBuf::from(src_path);
        let src_ext = src_path
            .extension()
            .ok_or_else(|| ErrorBadRequest("source file has no extension"))?;

        let dst_name = format!("S{:02}E{:02}", req.season_number, req.episode_number);
        let dst_path = Path::new(&show_path).join(dst_name).with_extension(src_ext);

        // Just move the file into the library and let the fs watcher
        // take care of the rest
        tracing::info!("moving {:?} to {:?}", src_path, dst_path);
        std::fs::rename(&src_path, &dst_path).map_err(ErrorInternalServerError)?;

        Ok(dst_path)
    }
}
