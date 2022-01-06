use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use once_cell::sync::OnceCell;
use regex::Regex;
use time::{Date, Instant, OffsetDateTime};
use walkdir::{DirEntry, WalkDir};

use crate::config::{Config, ImportMatcher, ImportMatcherTarget};
use crate::db::{self, Db};
use crate::ffprobe::VideoInfoProvider;
use crate::library::movies::NewMovie;
use crate::library::shows::{NewEpisode, NewSeason, NewShow};
use crate::library::{video_info, MediaLibrary};
use crate::metadata::{MetadataManager, RefreshRequest};
use crate::transcoder::{self, Transcoder};

pub struct LibraryScanner {
    db: Db,
    library: Arc<MediaLibrary>,
    metadata: MetadataManager,
    config: Arc<Config>,
    video_info: Arc<dyn VideoInfoProvider>,
    transcoder: Arc<Transcoder>,
    is_running: Arc<AtomicBool>,
}

#[derive(Debug)]
pub enum VideoFileType {
    Movie,
    Episode,
}

#[derive(Debug, Default)]
pub struct ScanOptions {
    pub rescan_files: bool,
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
            db,
            library,
            metadata,
            config,
            video_info,
            transcoder,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Starts a library scan if one is not already running.
    ///
    /// Returns immediately without waiting for the scan to finish.
    pub fn start_scan(self: Arc<Self>, options: ScanOptions) {
        tokio::spawn(self.run_scan(options));
    }

    /// Starts a library scan if one is not already running
    pub async fn run_scan(self: Arc<Self>, options: ScanOptions) {
        if !self.is_running.swap(true, Ordering::SeqCst) {
            let start_time = Instant::now();

            tracing::info!(?options, "starting library scan");

            if let Err(e) = self
                .scan_movies(&self.config.libraries.movies, &options)
                .await
            {
                tracing::error!("{:?}", e);
            };

            if let Err(e) = self
                .scan_shows(&self.config.libraries.tv_shows, &options)
                .await
            {
                tracing::error!("{:?}", e);
            };

            self.is_running.store(false, Ordering::SeqCst);

            let duration = Instant::now() - start_time;
            let seconds = duration.as_seconds_f32();

            tracing::info!("completed scan in {:.3}s", seconds);
        }
    }

    /// Recursively scans a folder for new movie files
    pub async fn scan_movies(&self, path: &str, options: &ScanOptions) -> eyre::Result<()> {
        if !Path::new(path).is_dir() {
            return Err(eyre::eyre!("directory {} does not exist", path));
        }

        self.library.movies().validate().await?;

        for entry in get_video_files(path) {
            self.scan_movie_file(entry.path(), options).await?;
        }

        Ok(())
    }

    /// Recursively scans a folder for tv shows
    pub async fn scan_shows(&self, path: &str, options: &ScanOptions) -> eyre::Result<()> {
        if !Path::new(path).is_dir() {
            return Err(eyre::eyre!("directory {} does not exist", path));
        }

        self.library.shows().validate().await?;

        for entry in get_video_files(path) {
            self.scan_episode_file(entry.path(), options).await?;
        }

        Ok(())
    }

    /// Scans a single video file
    #[tracing::instrument(skip(self, path), fields(path = ?path.as_ref()))]
    pub async fn scan_file(
        &self,
        video_type: VideoFileType,
        path: impl AsRef<Path>,
        options: ScanOptions,
    ) -> eyre::Result<Option<i64>> {
        let path = path.as_ref();

        // Bail if path is not a video file
        if (path.exists() && !path.is_file()) || !is_video_file_path(path) {
            return Ok(None);
        }

        tracing::info!("scanning file");

        match video_type {
            VideoFileType::Movie => self.scan_movie_file(path, &options).await,
            VideoFileType::Episode => self.scan_episode_file(path, &options).await,
        }
    }

    async fn scan_movie_file(
        &self,
        path: &Path,
        options: &ScanOptions,
    ) -> eyre::Result<Option<i64>> {
        let name = path.file_name().and_then(|v| v.to_str()).unwrap();
        let path_str = path.to_str().unwrap();
        let library = self.library.movies();

        if let Some(id) = library.get_id_by_path(path_str).await? {
            if !path.is_file() {
                // Remove movie from database if file no longer exists
                self.library.movies().remove_movie(id).await?;
                return Ok(Some(id));
            }

            if options.rescan_files {
                self.rescan_video_file_path(id, path_str).await?;
            }

            return Ok(Some(id));
        }

        let (title, release_date) = match parse_movie_filename(name) {
            Some(v) => v,
            None => return Ok(None),
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

        Ok(Some(id))
    }

    async fn scan_episode_file(
        &self,
        path: &Path,
        options: &ScanOptions,
    ) -> eyre::Result<Option<i64>> {
        let library = self.library.shows();

        let (show_name, season, episode, name) = match parse_episode_path(self.matchers(), path) {
            Some(v) => v,
            None => return Ok(None),
        };

        let parent_path = match path.parent().and_then(|v| v.to_str()) {
            Some(v) => v,
            None => return Ok(None),
        };

        let path_str = match path.to_str() {
            Some(v) => v,
            None => return Ok(None),
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
                return Ok(Some(id));
            }

            if options.rescan_files {
                self.rescan_video_file_path(id, path_str).await?;
            }

            return Ok(Some(id));
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

        Ok(Some(id))
    }

    #[tracing::instrument(skip(self))]
    pub async fn rescan_video_file(&self, id: i64) -> eyre::Result<()> {
        tracing::info!("rescanning video file");

        let mut conn = self.db.acquire().await?;
        let info = match db::videos::get_basic_info(&mut conn, id).await? {
            Some(path) => path,
            None => {
                tracing::warn!("video not found in db");
                return Ok(());
            }
        };

        self.rescan_video_file_path(id, &info.path).await
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
///
/// Supported formats:
/// - `Show Name/S01E02.mp4`
pub fn parse_episode_path(
    matchers: &[ImportMatcher],
    path: &Path,
) -> Option<(String, u32, u32, Option<String>)> {
    let file_name = path.file_name()?.to_str()?;
    let folder_name = path.parent()?.file_name()?.to_str()?;

    let meta = parse_video_filename(matchers, file_name)?;
    let (season, episode, name) = match meta {
        VideoFilenameMeta::Movie { .. } => return None,
        VideoFilenameMeta::Episode {
            name,
            season,
            episode,
        } => (season, episode, name),
    };

    Some((folder_name.to_owned(), season, episode, name))
}

#[derive(serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VideoFilenameMeta {
    Movie {
        title: String,
        year: Option<u32>,
    },
    Episode {
        name: Option<String>,
        season: u32,
        episode: u32,
    },
}

/// Extracts info about a video file from a filename, using a list of matchers.
pub fn parse_video_filename(
    matchers: &[ImportMatcher],
    filename: &str,
) -> Option<VideoFilenameMeta> {
    matchers.iter().find_map(|matcher| {
        let captures = matcher.regex.captures(filename)?;
        let result = match matcher.target {
            ImportMatcherTarget::Movie => {
                let title = captures.name("title")?.as_str().replace('.', " ");
                let year = captures.name("year").and_then(|v| v.as_str().parse().ok());

                VideoFilenameMeta::Movie { title, year }
            }
            ImportMatcherTarget::Episode => {
                let name = captures.name("name").map(|s| s.as_str().replace('.', " "));
                let season = captures.name("season")?.as_str().parse().ok()?;
                let episode = captures.name("episode")?.as_str().parse().ok()?;

                VideoFilenameMeta::Episode {
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
