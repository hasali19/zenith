use std::path::Path;
use std::sync::Arc;

use eyre::eyre;
use once_cell::sync::OnceCell;
use regex::Regex;
use time::{Date, Instant, OffsetDateTime};
use tokio::sync::Mutex;
use walkdir::{DirEntry, WalkDir};

use crate::config::{Config, ImportMatcher, ImportMatcherTarget};
use crate::db::media::MediaItemType;
use crate::db::{self, Db};
use crate::ffprobe::VideoInfoProvider;
use crate::library::movies::NewMovie;
use crate::library::shows::{NewEpisode, NewSeason, NewShow};
use crate::library::{video_info, MediaLibrary};
use crate::metadata::{MetadataManager, RefreshRequest};
use crate::transcoder::{self, Transcoder};

pub struct LibraryScanner {
    db: Db,
    inner: Mutex<LibraryScannerImpl>,
}

struct LibraryScannerImpl {
    db: Db,
    library: Arc<MediaLibrary>,
    metadata: MetadataManager,
    config: Arc<Config>,
    video_info: Arc<dyn VideoInfoProvider>,
    transcoder: Arc<Transcoder>,
}

#[derive(Clone, Copy, Debug)]
pub enum VideoFileType {
    Movie,
    Episode,
}

#[derive(Debug, Default)]
pub struct ScanOptions {
    /// Whether to rescan existing video files to extract metadata and embedded subtitles.
    /// New files will always be scanned.
    pub rescan_files: bool,
    /// Whether to refresh metadata for existing media items.
    /// Metadata will always be refreshed for new items.
    pub refresh_metadata: bool,
}

impl ScanOptions {
    pub fn quick() -> Self {
        ScanOptions {
            rescan_files: false,
            refresh_metadata: false,
        }
    }

    pub fn rescan_files() -> Self {
        ScanOptions {
            rescan_files: true,
            refresh_metadata: false,
        }
    }
}

#[derive(Debug)]
pub enum FileScanResult {
    Added(i64),
    Updated(i64),
    Removed,
    Ignored,
}

impl LibraryScanner {
    pub fn new(
        db: Db,
        library: Arc<MediaLibrary>,
        metadata: MetadataManager,
        config: Arc<Config>,
        video_info: Arc<dyn VideoInfoProvider>,
        transcoder: Arc<Transcoder>,
    ) -> LibraryScanner {
        LibraryScanner {
            db: db.clone(),
            inner: Mutex::new(LibraryScannerImpl {
                db,
                library,
                metadata,
                config,
                video_info,
                transcoder,
            }),
        }
    }

    /// Starts a library scan if one is not already running.
    ///
    /// Returns immediately without waiting for the scan to finish.
    pub fn start_scan(self: Arc<Self>, options: ScanOptions) {
        tokio::spawn(async move { self.run_scan(&options).await });
    }

    /// Runs a library scan if one is not already running.
    ///
    /// This will validate libraries by checking that existing media items still exist on disk, and
    /// scan the disk for new files to be added to the library.
    ///
    /// If a scan is already running, this will return immediately without waiting for it
    /// to finish.
    #[tracing::instrument(skip(self))]
    pub async fn run_scan(self: Arc<Self>, options: &ScanOptions) {
        if let Ok(scanner) = self.inner.try_lock() {
            let start_time = Instant::now();

            tracing::info!(?options, "starting library scan");

            async fn log_error(fut: impl std::future::Future<Output = eyre::Result<()>>) {
                if let Err(e) = fut.await {
                    tracing::error!("{:?}", e);
                }
            }

            log_error(scanner.library.movies().validate()).await;
            log_error(scanner.scan_movies(&scanner.config.libraries.movies, options)).await;

            log_error(scanner.library.shows().validate()).await;
            log_error(scanner.scan_shows(&scanner.config.libraries.tv_shows, options)).await;

            let duration = Instant::now() - start_time;
            let seconds = duration.as_seconds_f32();

            tracing::info!("completed scan in {:.3}s", seconds);
        } else {
            tracing::info!("scan is already in progress");
        }
    }

    /// Scans a single video file.
    #[tracing::instrument(skip(self))]
    pub async fn scan_file(
        &self,
        id: i64,
        options: &ScanOptions,
    ) -> eyre::Result<Option<FileScanResult>> {
        let mut conn = self.db.acquire().await?;

        let item_type = match db::media::get_item_type(&mut conn, id).await? {
            Some(v) => v,
            None => return Ok(None),
        };

        let info = match db::videos::get_basic_info(&mut conn, id).await? {
            Some(path) => path,
            None => return Ok(None),
        };

        let video_type = match item_type {
            MediaItemType::Movie => VideoFileType::Movie,
            MediaItemType::TvEpisode => VideoFileType::Episode,
            _ => return Ok(None),
        };

        self.scan_file_path(video_type, info.path, options)
            .await
            .map(Some)
    }

    /// Scans a single video file.
    #[tracing::instrument(skip(self, path), fields(path = ?path.as_ref()))]
    pub async fn scan_file_path(
        &self,
        video_type: VideoFileType,
        path: impl AsRef<Path>,
        options: &ScanOptions,
    ) -> eyre::Result<FileScanResult> {
        let path = path.as_ref();
        let scanner = self.inner.lock().await;

        // Bail if path is not a video file
        if (path.exists() && !path.is_file()) || !is_video_file_path(path) {
            return Ok(FileScanResult::Ignored);
        }

        tracing::info!("scanning file");

        let result = scanner.scan_video_file(video_type, path, options).await?;

        tracing::info!(?result, "scan completed");

        Ok(result)
    }
}

impl LibraryScannerImpl {
    /// Recursively scans a folder for movie files.
    async fn scan_movies(&self, path: impl AsRef<Path>, options: &ScanOptions) -> eyre::Result<()> {
        self.scan_library_dir(path, VideoFileType::Movie, options)
            .await
    }

    /// Recursively scans a folder for tv episode files.
    async fn scan_shows(&self, path: &impl AsRef<Path>, options: &ScanOptions) -> eyre::Result<()> {
        self.scan_library_dir(path, VideoFileType::Episode, options)
            .await
    }

    /// Recursively scans a folder for video files of the specified type.
    async fn scan_library_dir(
        &self,
        path: impl AsRef<Path>,
        video_type: VideoFileType,
        options: &ScanOptions,
    ) -> eyre::Result<()> {
        let path = path.as_ref();
        if !path.is_dir() {
            return Err(eyre!("directory {} does not exist", path.display()));
        }

        for entry in get_video_files(path) {
            self.scan_video_file(video_type, entry.path(), options)
                .await?;
        }

        Ok(())
    }

    /// Scans a single video file.
    async fn scan_video_file(
        &self,
        video_type: VideoFileType,
        path: &Path,
        options: &ScanOptions,
    ) -> eyre::Result<FileScanResult> {
        match video_type {
            VideoFileType::Movie => self.scan_movie_file(path, options).await,
            VideoFileType::Episode => self.scan_episode_file(path, options).await,
        }
    }

    async fn scan_movie_file(
        &self,
        path: &Path,
        options: &ScanOptions,
    ) -> eyre::Result<FileScanResult> {
        let name = path.file_name().and_then(|v| v.to_str()).unwrap();
        let path_str = path.to_str().unwrap();
        let library = self.library.movies();

        if let Some(id) = library.get_id_by_path(path_str).await? {
            if !path.is_file() {
                // Remove movie from database if file no longer exists
                library.remove_movie(id).await?;
                return Ok(FileScanResult::Removed);
            }

            if options.rescan_files {
                self.rescan_video_file_path(id, path_str).await?;
            }

            return Ok(FileScanResult::Updated(id));
        }

        let (title, release_date) = match parse_movie_filename(name) {
            Some(v) => v,
            None => return Ok(FileScanResult::Ignored),
        };

        tracing::info!("adding movie: {}", name);

        let movie = NewMovie {
            path: path_str,
            title: &title,
            release_date: release_date.map(|dt| dt.unix_timestamp()),
        };

        let id = library.add_movie(&movie).await?;
        self.metadata.enqueue(RefreshRequest::Movie(id));
        self.transcoder.enqueue(transcoder::Job::new(id)).await;

        Ok(FileScanResult::Added(id))
    }

    async fn scan_episode_file(
        &self,
        path: &Path,
        options: &ScanOptions,
    ) -> eyre::Result<FileScanResult> {
        let library = self.library.shows();

        let (show_name, season, episode, name) = match parse_episode_path(self.matchers(), path) {
            Some(v) => v,
            None => return Ok(FileScanResult::Ignored),
        };

        let parent_path = match path.parent().and_then(|v| v.to_str()) {
            Some(v) => v,
            None => return Ok(FileScanResult::Ignored),
        };

        let path_str = match path.to_str() {
            Some(v) => v,
            None => return Ok(FileScanResult::Ignored),
        };

        let show_id = match library.get_show_id_by_path(parent_path).await? {
            Some(id) => id,
            None => {
                tracing::info!("adding show: {}", show_name);

                let show = NewShow {
                    path: parent_path,
                    name: &show_name,
                };

                let id = library.add_show(show).await?;
                self.metadata.enqueue(RefreshRequest::TvShow(id));
                id
            }
        };

        let season_id = match library.get_season_id(show_id, season).await? {
            Some(id) => id,
            None => {
                tracing::info!("adding season: {} ({})", show_name, season);

                let season = NewSeason {
                    show_id,
                    season_number: season,
                };

                let id = library.add_season(season).await?;
                self.metadata.enqueue(RefreshRequest::TvSeason(id));
                id
            }
        };

        if let Some(id) = library.get_episode_id(season_id, episode).await? {
            if !path.is_file() {
                // Remove episode from database if file no longer exists
                library.remove_episode(id).await?;
                // Cleanup any empty shows/seasons after episode removed
                library.remove_empty_collections().await?;
                return Ok(FileScanResult::Removed);
            }

            if options.rescan_files {
                self.rescan_video_file_path(id, path_str).await?;
            }

            return Ok(FileScanResult::Updated(id));
        }

        tracing::info!("adding episode: {} ({}:{})", show_name, season, episode);

        let episode = NewEpisode {
            season_id,
            episode_number: episode,
            name: name.as_deref(),
            path: path_str,
        };

        let id = library.add_episode(episode).await?;
        self.metadata.enqueue(RefreshRequest::TvEpisode(id));
        self.transcoder.enqueue(transcoder::Job::new(id)).await;

        Ok(FileScanResult::Added(id))
    }

    async fn rescan_video_file_path(&self, id: i64, path: &str) -> eyre::Result<()> {
        tracing::debug!(id, path, "rescanning video file");

        let mut transaction = self.db.begin().await?;
        let info = self.video_info.get_video_info(path).await?;

        video_info::update_video_info(&mut transaction, id, &info).await?;

        transaction.commit().await?;

        Ok(())
    }

    fn matchers(&self) -> &[ImportMatcher] {
        &self.config.import.matchers
    }
}

/// Extracts a title and (optional) year from a filename.
///
/// Supported formats:
/// - `This is the title.mp4`
/// - `This is the title (2021).mp4`
pub fn parse_movie_filename(name: &str) -> Option<(String, Option<OffsetDateTime>)> {
    static REGEX: OnceCell<Regex> = OnceCell::new();

    let captures = REGEX
        .get_or_init(|| Regex::new(r"^(\S.*?)(?: \((\d\d\d\d)\))?(?:\.\w+)?$").unwrap())
        .captures(name)?;

    let name = captures.get(1)?.as_str().to_owned();
    let year = captures
        .get(2)
        .map(|m| m.as_str().parse::<i32>().ok())
        .flatten()
        .and_then(|year| Date::from_ordinal_date(year, 1).ok())
        .and_then(|date| date.with_hms(0, 0, 0).ok())
        .map(|dt| dt.assume_utc());

    Some((name, year))
}

/// Recursively searches a directory for video files
pub fn get_video_files(path: impl AsRef<Path>) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| is_video_file_path(e.path()))
}

/// Extracts a show name, season and episode number from an episode path.
fn parse_episode_path(
    matchers: &[ImportMatcher],
    path: &Path,
) -> Option<(String, u32, u32, Option<String>)> {
    let file_name = path.file_name()?.to_str()?;
    let folder_name = path.parent()?.file_name()?.to_str()?;

    let episode_matchers = matchers
        .iter()
        .filter(|m| m.target == ImportMatcherTarget::Episode);

    match parse_video_filename(episode_matchers, file_name)? {
        VideoFilenameMeta::Episode {
            show_name,
            name,
            season,
            episode,
        } => Some((
            show_name.unwrap_or_else(|| folder_name.to_owned()),
            season,
            episode,
            name,
        )),
        _ => None,
    }
}

#[derive(serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VideoFilenameMeta {
    Movie {
        title: String,
        year: Option<u32>,
    },
    Episode {
        show_name: Option<String>,
        name: Option<String>,
        season: u32,
        episode: u32,
    },
}

/// Extracts info about a video file from a filename, using a list of matchers.
fn parse_video_filename<'a>(
    matchers: impl IntoIterator<Item = &'a ImportMatcher>,
    filename: &str,
) -> Option<VideoFilenameMeta> {
    matchers.into_iter().find_map(|matcher| {
        let captures = matcher.regex.captures(filename)?;
        let result = match matcher.target {
            ImportMatcherTarget::Movie => {
                let title = captures.name("title")?.as_str().replace('.', " ");
                let year = captures.name("year").and_then(|v| v.as_str().parse().ok());

                VideoFilenameMeta::Movie { title, year }
            }
            ImportMatcherTarget::Episode => {
                let show_name = captures
                    .name("show_name")
                    .map(|s| s.as_str().replace('.', " "));
                let name = captures.name("name").map(|s| s.as_str().replace('.', " "));
                let season = captures.name("season")?.as_str().parse().ok()?;
                let episode = captures.name("episode")?.as_str().parse().ok()?;

                VideoFilenameMeta::Episode {
                    show_name,
                    name,
                    season,
                    episode,
                }
            }
        };

        Some(result)
    })
}

fn is_video_file_path(path: &Path) -> bool {
    const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv"];

    let ext = match path.extension().and_then(|v| v.to_str()) {
        Some(ext) => ext,
        None => return false,
    };

    VIDEO_EXTENSIONS.contains(&ext)
}
