use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use atium::endpoint;
use atium::headers::ContentType;
use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::Body;
use atium::Request;
use bytes::Buf;
use eyre::eyre;
use mime::Mime;
use multer::Multipart;
use serde::Deserialize;
use serde_json::json;
use tokio::fs::File;
use tokio::io::BufWriter;
use uuid::Uuid;

use crate::api::error::bad_request;
use crate::api::ext::OptionExt;
use crate::config::Config;
use crate::db::subtitles::{NewSubtitle, SubtitlePath};
use crate::db::Db;
use crate::library::scanner::{self, LibraryScanner, ScanOptions, VideoFileType};
use crate::{db, subtitles, util};

pub fn routes(router: &mut Router) {
    router.route("/import/queue").get(get_import_queue);
    router.route("/import/subtitle").post(import_subtitle);
}

#[endpoint]
async fn get_import_queue(req: &mut Request) -> eyre::Result<()> {
    let config: &Arc<Config> = req.ext().unwrap();
    let import_path = match config.import.path.as_deref() {
        Some(path) => path,
        None => {
            req.ok().json(&vec![(); 0])?;
            return Ok(());
        }
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

    req.ok().json(&entries)?;

    Ok(())
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

async fn multipart(req: &Request, body: Body) -> eyre::Result<Multipart<'static>> {
    let content_type: Mime = req
        .header::<ContentType>()
        .or_bad_request("invalid content-type header")?
        .into();

    if content_type.essence_str() != "multipart/form-data" {
        return Err(eyre!(bad_request(
            "content-type must be multipart/form-data"
        )));
    }

    let boundary = content_type
        .get_param(mime::BOUNDARY)
        .or_bad_request("missing boundary in content-type")?;

    Ok(Multipart::new(body, boundary.as_str()))
}

#[endpoint]
async fn import_subtitle(req: &mut Request) -> eyre::Result<()> {
    let body = req.body();
    let config: &Arc<Config> = req.ext().unwrap();
    let db: &Db = req.ext().unwrap();

    let mut multipart = multipart(req, body).await?;

    let ImportSubtitleRequest { source, data } = {
        let field = multipart
            .next_field()
            .await?
            .or_bad_request("missing data field in mutipart body")?;

        if !matches!(field.name(), Some("data")) {
            return Err(eyre!(bad_request(
                "first field in multipart body must be 'data'"
            )));
        }

        serde_json::from_reader(field.bytes().await?.reader())?
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

                    subtitles::convert(config, input_file, output_file).await?;

                    output_path
                }
                _ => return Err(eyre!(bad_request("unsupported subtitle file extension",))),
            }
        }
        ImportSource::Upload => {
            let field = multipart
                .next_field()
                .await?
                .or_bad_request("upload import source specified but no file found in request")?;

            let content_type = field
                .content_type()
                .map(|c| c.essence_str())
                .or_bad_request("missing content-type for file upload");

            if !Path::new("data/tmp").exists() {
                std::fs::create_dir_all("data/tmp")?;
            }

            let src_path = format!("data/tmp/{}.vtt", Uuid::new_v4());
            let file = BufWriter::new(File::create(&src_path).await?);

            match content_type? {
                // vtt subtitles can be directly written to the file
                "text/vtt" => util::copy_stream(field, file).await?,
                // srt subtitles need to be converted first
                "application/x-subrip" => subtitles::convert(config, field, file).await?,
                _ => return Err(eyre!(bad_request("unsupported subtitle content-type",))),
            }

            PathBuf::from(src_path)
        }
    };

    let dst_name = Uuid::new_v4().to_string();
    let dst_path = subtitles_dir.join(dst_name).with_extension("vtt");
    if dst_path.exists() {
        return Err(bad_request(format!("{:?} already exists", dst_path)).into());
    }

    let mut transaction = db.begin().await?;
    let subtitles = NewSubtitle {
        video_id: data.video_id,
        path: SubtitlePath::External {
            path: Cow::Borrowed(dst_path.to_str().unwrap()),
        },
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
    req.ok();

    Ok(())
}

#[derive(Deserialize)]
pub struct ImportMovieRequest {
    source: ImportSource,
    title: String,
    year: u32,
}

#[endpoint]
pub async fn import_movie(req: &mut Request) -> eyre::Result<()> {
    let data: ImportMovieRequest = req.body_json().await?;
    let config: &Arc<Config> = req.ext().unwrap();
    let scanner: &Arc<LibraryScanner> = req.ext().unwrap();

    let src_path = match data.source {
        ImportSource::Local { path, copy: _ } => path,
        _ => return Err(eyre!(bad_request("unsupported import source"))),
    };

    let src_path = PathBuf::from(src_path);
    let src_ext = src_path
        .extension()
        .or_bad_request("source file has no extension")?;

    let dst_name = format!("{} ({})", data.title, data.year);
    let dst_dir = Path::new(&config.libraries.movies).join(&dst_name);

    if dst_dir.exists() {
        return Err(bad_request(format!("{:?} already exists", dst_dir)).into());
    }

    let dst_path = dst_dir.join(dst_name).with_extension(src_ext);

    tracing::info!("moving {:?} to {:?}", src_path, dst_path);
    std::fs::create_dir(&dst_dir)?;
    std::fs::rename(&src_path, &dst_path)?;

    scanner
        .scan_file(VideoFileType::Movie, &dst_path, ScanOptions::quick())
        .await?;

    req.ok();

    Ok(())
}

#[derive(Deserialize)]
pub struct ImportShowRequest {
    name: String,
    episodes: Vec<ImportEpisodeRequest>,
}

#[endpoint]
pub async fn import_show(req: &mut Request) -> eyre::Result<()> {
    let data: ImportShowRequest = req.body_json().await?;
    let config: &Arc<Config> = req.ext().unwrap();
    let scanner: &Arc<LibraryScanner> = req.ext().unwrap();

    if data.episodes.is_empty() {
        return Err(bad_request("show must have at least one episode").into());
    }

    let show_path = Path::new(&config.libraries.tv_shows).join(&data.name);
    if show_path.exists() {
        return Err(bad_request(format!("{:?} already exists", show_path)).into());
    }

    std::fs::create_dir(&show_path)?;

    for episode in data.episodes {
        let path = inner::import_episode(&show_path, episode).await?;
        scanner
            .scan_file(VideoFileType::Episode, &path, ScanOptions::quick())
            .await?;
    }

    req.ok();

    Ok(())
}

#[derive(Deserialize)]
pub struct ImportEpisodeRequest {
    source: ImportSource,
    season_number: u32,
    episode_number: u32,
}

#[endpoint]
pub async fn import_episode(req: &mut Request) -> eyre::Result<()> {
    let show_id: i64 = req.param("show_id")?;
    let data: ImportEpisodeRequest = req.body_json().await?;
    let db: &Db = req.ext().unwrap();
    let scanner: &Arc<LibraryScanner> = req.ext().unwrap();

    let mut conn = db.acquire().await?;
    let show_path = db::shows::get_path(&mut conn, show_id)
        .await?
        .or_not_found("show not found")?;

    let path = inner::import_episode(Path::new(&show_path), data).await?;

    scanner
        .scan_file(VideoFileType::Episode, &path, ScanOptions::quick())
        .await?;

    req.ok();

    Ok(())
}

mod inner {
    use std::path::{Path, PathBuf};

    use eyre::eyre;

    use crate::api::error::bad_request;
    use crate::api::ext::OptionExt;

    use super::{ImportEpisodeRequest, ImportSource};

    pub async fn import_episode(
        show_path: &Path,
        req: ImportEpisodeRequest,
    ) -> eyre::Result<PathBuf> {
        let src_path = match req.source {
            ImportSource::Local { path, copy: _ } => path,
            _ => return Err(eyre!(bad_request("unsupported import source"))),
        };

        let src_path = PathBuf::from(src_path);
        let src_ext = src_path
            .extension()
            .or_bad_request("source file has no extension")?;

        let dst_name = format!("S{:02}E{:02}", req.season_number, req.episode_number);
        let dst_path = Path::new(&show_path).join(dst_name).with_extension(src_ext);

        // Just move the file into the library and let the fs watcher
        // take care of the rest
        tracing::info!("moving {:?} to {:?}", src_path, dst_path);
        std::fs::rename(&src_path, &dst_path)?;

        Ok(dst_path)
    }
}
