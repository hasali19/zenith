use std::path::{Path, PathBuf};
use std::sync::Arc;

use actix_multipart::Multipart;
use actix_web::web::{self, Json};
use actix_web::{get, post, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::fs::File;
use tokio::io::BufWriter;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::api::error::bad_request;
use crate::api::ext::OptionExt;
use crate::api::ApiResult;
use crate::config::Config;
use crate::db::subtitles::NewSubtitle;
use crate::db::Db;
use crate::library::scanner::{self, LibraryScanner, ScanOptions, VideoFileType};
use crate::transcoder::{Job, Transcoder};
use crate::{db, subtitles, util, Ext};

#[get("/import/queue")]
async fn get_import_queue(config: Ext<Arc<Config>>) -> ApiResult<Json<Vec<Value>>> {
    let import_path = match config.import.path.as_deref() {
        Some(path) => path,
        None => return Ok(Json(vec![])),
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

    Ok(Json(entries))
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ImportSource {
    Local { path: String, copy: Option<bool> },
    Upload,
}

#[derive(Deserialize)]
pub struct ImportSubtitleRequest {
    source: ImportSource,
    #[serde(flatten)]
    data: ImportSubtitleRequestData,
}

#[derive(Deserialize)]
pub struct ImportSubtitleRequestData {
    video_id: i64,
    title: Option<String>,
    language: Option<String>,
}

#[post("/import/subtitle")]
pub async fn import_subtitle(
    mut multipart: Multipart,
    config: Ext<Arc<Config>>,
    db: Db,
) -> ApiResult<impl Responder> {
    let ImportSubtitleRequest { source, data } = {
        let mut field = multipart
            .next()
            .await
            .or_bad_request("missing data field in mutipart body")?
            .map_err(bad_request)?;

        let content_disposition = field
            .content_disposition()
            .or_bad_request("missing content-disposition")?;

        if !matches!(content_disposition.get_name(), Some("data")) {
            return Err(bad_request("first field in multipart body must be 'data'"));
        }

        let mut bytes = vec![];
        while let Some(chunk) = field.next().await {
            bytes.extend_from_slice(chunk.map_err(bad_request)?.as_ref());
        }

        serde_json::from_slice(&bytes).map_err(bad_request)?
    };

    let subtitles_dir = config.subtitles.path.join(data.video_id.to_string());
    let src_path = match source {
        ImportSource::Local { path, copy: _ } => {
            let src_path = PathBuf::from(path);
            let src_ext = src_path
                .extension()
                .and_then(|ext| ext.to_str())
                .or_bad_request("source file has no extension")?;

            match src_ext {
                // vtt subtitles can be directly written to the file
                "vtt" => src_path,
                // srt subtitles need to be converted first
                "srt" => {
                    // TODO: Consider writing ffmpeg output directly to destination file to avoid
                    // the extra temporary file

                    let input_file = util::to_byte_stream(File::open(src_path).await?);
                    let output_path = PathBuf::from(format!("data/tmp/{}.vtt", Uuid::new_v4()));
                    let output_file = BufWriter::new(File::create(&output_path).await?);

                    subtitles::convert(&config, input_file, output_file).await?;

                    output_path
                }
                _ => return Err(bad_request("unsupported subtitle file extension")),
            }
        }
        ImportSource::Upload => {
            let field = multipart
                .next()
                .await
                .or_bad_request("upload import source specified but no file found in request")?
                .map_err(bad_request)?;

            if !Path::new("data/tmp").exists() {
                std::fs::create_dir_all("data/tmp")?;
            }

            let src_path = format!("data/tmp/{}.vtt", Uuid::new_v4());
            let file = BufWriter::new(File::create(&src_path).await?);

            match field.content_type().essence_str() {
                // vtt subtitles can be directly written to the file
                "text/vtt" => util::copy_stream(field, file).await?,
                // srt subtitles need to be converted first
                "application/x-subrip" => subtitles::convert(&config, field, file).await?,
                _ => return Err(bad_request("unsupported subtitle content-type")),
            }

            PathBuf::from(src_path)
        }
    };

    let dst_name = Uuid::new_v4().to_string();
    let dst_path = subtitles_dir.join(dst_name).with_extension("vtt");
    if dst_path.exists() {
        return Err(bad_request(format!("{:?} already exists", dst_path)));
    }

    let mut transaction = db.begin().await?;
    let subtitles = NewSubtitle {
        video_id: data.video_id,
        stream_index: None,
        path: dst_path.to_str(),
        title: data.title.as_deref(),
        language: data.language.as_deref(),
    };

    db::subtitles::insert(&mut transaction, &subtitles).await?;

    if !subtitles_dir.exists() {
        std::fs::create_dir_all(&subtitles_dir)?;
    }

    tracing::info!("copying {:?} to {:?}", src_path, dst_path);
    std::fs::copy(&src_path, &dst_path)?;

    transaction.commit().await?;

    Ok(HttpResponse::Ok())
}

#[derive(Deserialize)]
pub struct ImportMovieRequest {
    source: ImportSource,
    title: String,
    year: u32,
}

#[post("/movies")]
pub async fn import_movie(
    Json(data): Json<ImportMovieRequest>,
    config: Ext<Arc<Config>>,
    scanner: Ext<Arc<LibraryScanner>>,
    transcoder: Ext<Arc<Transcoder>>,
) -> ApiResult<impl Responder> {
    let src_path = match data.source {
        ImportSource::Local { path, copy: _ } => path,
        _ => return Err(bad_request("unsupported import source")),
    };

    let src_path = PathBuf::from(src_path);
    let src_ext = src_path
        .extension()
        .or_bad_request("source file has no extension")?;

    let dst_name = format!("{} ({})", data.title, data.year);
    let dst_dir = Path::new(&config.libraries.movies).join(&dst_name);

    if dst_dir.exists() {
        return Err(bad_request(format!("{:?} already exists", dst_dir)));
    }

    let dst_path = dst_dir.join(dst_name).with_extension(src_ext);

    tracing::info!("moving {:?} to {:?}", src_path, dst_path);
    std::fs::create_dir(&dst_dir)?;
    std::fs::rename(&src_path, &dst_path)?;

    let id = scanner
        .scan_file(VideoFileType::Movie, &dst_path, ScanOptions::quick())
        .await?;

    if let Some(id) = id {
        transcoder.enqueue(Job::new(id)).await;
    }

    Ok(HttpResponse::Ok())
}

#[derive(Deserialize)]
pub struct ImportShowRequest {
    name: String,
    episodes: Vec<ImportEpisodeRequest>,
}

#[post("/tv/shows")]
pub async fn import_show(
    Json(data): Json<ImportShowRequest>,
    config: Ext<Arc<Config>>,
    scanner: Ext<Arc<LibraryScanner>>,
    transcoder: Ext<Arc<Transcoder>>,
) -> ApiResult<impl Responder> {
    if data.episodes.is_empty() {
        return Err(bad_request("show must have at least one episode"));
    }

    let show_path = Path::new(&config.libraries.tv_shows).join(&data.name);
    if show_path.exists() {
        return Err(bad_request(format!("{:?} already exists", show_path)));
    }

    std::fs::create_dir(&show_path)?;

    for episode in data.episodes {
        let path = inner::import_episode(&show_path, episode).await?;
        let id = scanner
            .scan_file(VideoFileType::Episode, &path, ScanOptions::quick())
            .await?;

        if let Some(id) = id {
            transcoder.enqueue(Job::new(id)).await;
        }
    }

    Ok(HttpResponse::Ok())
}

#[derive(Deserialize)]
pub struct ImportEpisodeRequest {
    source: ImportSource,
    season_number: u32,
    episode_number: u32,
}

#[post("/tv/shows/{id}/episodes")]
pub async fn import_episode(
    show_id: web::Path<i64>,
    Json(data): Json<ImportEpisodeRequest>,
    db: Db,
    scanner: Ext<Arc<LibraryScanner>>,
    transcoder: Ext<Arc<Transcoder>>,
) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;
    let show_path = db::shows::get_path(&mut conn, *show_id)
        .await?
        .or_not_found("show not found")?;

    let path = inner::import_episode(Path::new(&show_path), data).await?;

    let id = scanner
        .scan_file(VideoFileType::Episode, &path, ScanOptions::quick())
        .await?;

    if let Some(id) = id {
        transcoder.enqueue(Job::new(id)).await;
    }

    Ok(HttpResponse::Ok())
}

mod inner {
    use super::*;

    pub async fn import_episode(show_path: &Path, req: ImportEpisodeRequest) -> ApiResult<PathBuf> {
        let src_path = match req.source {
            ImportSource::Local { path, copy: _ } => path,
            _ => return Err(bad_request("unsupported import source")),
        };

        let src_path = PathBuf::from(src_path);
        let src_ext = src_path
            .extension()
            .or_bad_request("source file has no extension")?;

        let dst_name = format!("S{:02}E{:02}", req.season_number, req.episode_number);
        let dst_path = Path::new(&show_path).join(dst_name).with_extension(src_ext);

        tracing::info!("moving {:?} to {:?}", src_path, dst_path);
        std::fs::rename(&src_path, &dst_path)?;

        Ok(dst_path)
    }
}
