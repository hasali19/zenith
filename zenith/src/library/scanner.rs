use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use once_cell::sync::OnceCell;
use regex::{Captures, Regex};
use time::{Date, Instant, OffsetDateTime};
use walkdir::{DirEntry, WalkDir};

use crate::config::Config;
use crate::db::subtitles::{NewSubtitle, SubtitlePath};
use crate::db::videos::UpdateVideo;
use crate::db::{self, Db};
use crate::ffprobe::VideoInfoProvider;
use crate::library::movies::NewMovie;
use crate::library::shows::{NewEpisode, NewSeason, NewShow};
use crate::library::MediaLibrary;
use crate::metadata::{MetadataManager, RefreshRequest};

pub struct LibraryScanner {
    db: Db,
    library: Arc<MediaLibrary>,
    metadata: MetadataManager,
    config: Arc<Config>,
    video_info: Arc<dyn VideoInfoProvider>,
    is_running: Arc<AtomicBool>,
}

pub enum VideoFileType {
    Movie,
    Episode,
}

#[derive(Debug)]
pub struct ScanOptions {
    pub rescan_files: bool,
    pub refresh_metadata: bool,
}

impl Default for ScanOptions {
    fn default() -> Self {
        ScanOptions {
            rescan_files: false,
            refresh_metadata: false,
        }
    }
}

impl ScanOptions {
    pub fn quick() -> Self {
        ScanOptions {
            rescan_files: false,
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
    ) -> LibraryScanner {
        LibraryScanner {
            db,
            library,
            metadata,
            config,
            video_info,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Starts a library scan if one is not already running
    pub fn start_scan(self: Arc<Self>, options: ScanOptions) {
        if !self.is_running.swap(true, Ordering::SeqCst) {
            tokio::spawn(async move {
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
            });
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
    pub async fn scan_file(
        &self,
        video_type: VideoFileType,
        path: impl AsRef<Path>,
        options: ScanOptions,
    ) -> eyre::Result<()> {
        let path = path.as_ref();
        match video_type {
            VideoFileType::Movie => self.scan_movie_file(path, &options).await,
            VideoFileType::Episode => self.scan_episode_file(path, &options).await,
        }
    }

    async fn scan_movie_file(&self, path: &Path, options: &ScanOptions) -> eyre::Result<()> {
        let name = path.file_name().and_then(|v| v.to_str()).unwrap();
        let path_str = path.to_str().unwrap();
        let library = self.library.movies();

        if let Some(id) = library.get_id_by_path(path_str).await? {
            // Remove movie from database if file no longer exists
            if !path.is_file() {
                // TODO: Remove movie from database
                return Ok(());
            }

            if options.rescan_files {
                self.rescan_video_file(id, path_str).await?;
            }

            return Ok(());
        }

        let (title, release_date) = match parse_movie_filename(name) {
            Some(v) => v,
            None => return Ok(()),
        };

        tracing::info!("adding movie: {}", name);

        let movie = NewMovie {
            path: path_str,
            title: &title,
            release_date: release_date.map(|dt| dt.unix_timestamp()),
        };

        let id = library.add_movie(&movie).await?;
        self.metadata.enqueue(RefreshRequest::Movie(id));

        Ok(())
    }

    async fn scan_episode_file(&self, path: &Path, options: &ScanOptions) -> eyre::Result<()> {
        let library = self.library.shows();

        let (show_name, season, episode) = match parse_episode_path(path) {
            Some(v) => v,
            None => return Ok(()),
        };

        let parent_path = match path.parent().and_then(|v| v.to_str()) {
            Some(v) => v,
            None => return Ok(()),
        };

        let path_str = match path.to_str() {
            Some(v) => v,
            None => return Ok(()),
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
                // TODO: Remove episode (and possibly season/show) from database
                // For now it should be fine since this is only called
                // in a library scan, after validating
                return Ok(());
            }

            if options.rescan_files {
                self.rescan_video_file(id, path_str).await?;
            }

            return Ok(());
        }

        tracing::info!("adding episode: {} ({}:{})", show_name, season, episode);

        let episode = NewEpisode {
            season_id,
            episode_number: episode,
            path: path_str,
        };

        let id = library.add_episode(episode).await?;
        self.metadata.enqueue(RefreshRequest::TvEpisode(id));

        Ok(())
    }

    async fn rescan_video_file(&self, id: i64, path: &str) -> eyre::Result<()> {
        let mut transaction = self.db.begin().await?;

        tracing::debug!(id, path, "rescanning video file");

        let info = self.video_info.get_video_info(path).await?;
        let data = UpdateVideo {
            duration: info.format.duration.parse()?,
        };

        db::videos::update(&mut transaction, id, data).await?;

        for stream in info
            .streams
            .iter()
            .filter(|stream| stream.codec_type == "subtitle")
        {
            let tags = stream.properties.get("tags").unwrap().as_object().unwrap();
            let subtitle = NewSubtitle {
                video_id: id,
                path: SubtitlePath::Embedded(stream.index),
                title: tags.get("title").map(|v| v.as_str().unwrap()),
                language: tags.get("language").map(|v| v.as_str().unwrap()),
            };

            db::subtitles::insert(&mut transaction, &subtitle).await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}

/// Extracts a title and (optional) year from a filename
pub fn parse_movie_filename(name: &str) -> Option<(String, Option<OffsetDateTime>)> {
    static REGEX: OnceCell<Regex> = OnceCell::new();

    let captures: Captures = REGEX
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
        .filter(is_video_file)
}

/// Extracts a show name, season and episode number from an episode path
pub fn parse_episode_path(path: &Path) -> Option<(String, u32, u32)> {
    static REGEX: OnceCell<Regex> = OnceCell::new();

    let file_name = path.file_name()?.to_str()?;
    let folder_name = path.parent()?.file_name()?.to_str()?;
    let captures: Captures = REGEX
        .get_or_init(|| Regex::new(r"^S(\d\d)E(\d\d)\.\S+$").unwrap())
        .captures(file_name)?;

    let season = captures.get(1)?.as_str().parse().ok()?;
    let episode = captures.get(2)?.as_str().parse().ok()?;

    Some((folder_name.to_owned(), season, episode))
}

fn is_video_file(entry: &DirEntry) -> bool {
    const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv"];

    let ext = match entry.path().extension().and_then(|v| v.to_str()) {
        Some(ext) => ext,
        None => return false,
    };

    VIDEO_EXTENSIONS.contains(&ext)
}
