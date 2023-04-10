use std::collections::HashSet;
use std::io;
use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use eyre::eyre;
use sqlx::Connection;
use time::{Date, Instant, OffsetDateTime};
use tokio::sync::Mutex;
use walkdir::WalkDir;

use crate::config::{Config, ImportMatcher, ImportMatcherTarget};
use crate::db::media::MediaItemType;
use crate::db::{self, Db};
use crate::library::movies::NewMovie;
use crate::library::shows::{NewEpisode, NewSeason, NewShow};
use crate::library::{video_info, MediaLibrary};
use crate::video_prober::VideoProber;

pub struct LibraryScanner {
    db: Db,
    inner: Mutex<LibraryScannerImpl>,
}

struct LibraryScannerImpl {
    db: Db,
    library: Arc<MediaLibrary>,
    config: Arc<Config>,
    video_prober: Arc<dyn VideoProber>,
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

impl FileScanResult {
    pub fn id(&self) -> Option<i64> {
        match self {
            FileScanResult::Added(id) | FileScanResult::Updated(id) => Some(*id),
            FileScanResult::Removed | FileScanResult::Ignored => None,
        }
    }
}

impl LibraryScanner {
    pub fn new(
        db: Db,
        library: Arc<MediaLibrary>,
        config: Arc<Config>,
        video_prober: Arc<dyn VideoProber>,
    ) -> LibraryScanner {
        LibraryScanner {
            db: db.clone(),
            inner: Mutex::new(LibraryScannerImpl {
                db,
                library,
                config,
                video_prober,
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

            tracing::info!("starting library scan");

            async fn log_error(fut: impl std::future::Future<Output = eyre::Result<()>>) {
                if let Err(e) = fut.await {
                    tracing::error!("{e:?}");
                }
            }

            log_error(scanner.validate_subtitles()).await;

            log_error(scanner.library.validate_movies()).await;
            log_error(scanner.scan_movies(&scanner.config.libraries.movies, options)).await;

            log_error(scanner.library.validate_shows()).await;
            log_error(scanner.scan_shows(&scanner.config.libraries.tv_shows, options)).await;

            let duration = Instant::now() - start_time;
            let seconds = duration.as_seconds_f32();

            tracing::info!("completed scan in {seconds:.3}s");
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
            MediaItemType::Episode => VideoFileType::Episode,
            _ => return Ok(None),
        };

        self.scan_file_path(video_type, &info.path, options)
            .await
            .map(Some)
    }

    /// Scans a single video file.
    #[tracing::instrument(skip(self))]
    pub async fn scan_file_path(
        &self,
        video_type: VideoFileType,
        path: &Utf8Path,
        options: &ScanOptions,
    ) -> eyre::Result<FileScanResult> {
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
    async fn scan_movies(&self, path: &Utf8Path, options: &ScanOptions) -> eyre::Result<()> {
        tracing::info!("scanning movies");
        self.scan_library_dir(path, VideoFileType::Movie, options)
            .await
    }

    /// Recursively scans a folder for tv episode files.
    async fn scan_shows(&self, path: &Utf8Path, options: &ScanOptions) -> eyre::Result<()> {
        tracing::info!("scanning shows");
        self.scan_library_dir(path, VideoFileType::Episode, options)
            .await
    }

    /// Recursively scans a folder for video files of the specified type.
    async fn scan_library_dir(
        &self,
        path: &Utf8Path,
        video_type: VideoFileType,
        options: &ScanOptions,
    ) -> eyre::Result<()> {
        if !path.is_dir() {
            return Err(eyre!("directory {path} does not exist"));
        }

        for file_path in get_video_files(path) {
            self.scan_video_file(video_type, &file_path, options)
                .await?;
        }

        Ok(())
    }

    /// Scans a single video file.
    async fn scan_video_file(
        &self,
        video_type: VideoFileType,
        path: &Utf8Path,
        options: &ScanOptions,
    ) -> eyre::Result<FileScanResult> {
        tracing::trace!(?video_type, %path, "scanning file");
        let result = match video_type {
            VideoFileType::Movie => self.scan_movie_file(path, options).await,
            VideoFileType::Episode => self.scan_episode_file(path, options).await,
        }?;

        if let Some(id) = result.id() {
            let video_file_name = path.file_stem().unwrap();
            let parent = path.parent().unwrap();
            let subtitles = WalkDir::new(parent)
                .max_depth(1)
                .into_iter()
                .flatten()
                .filter(|it| it.file_type().is_file())
                .filter(|it| {
                    let ext = it.path().extension().and_then(|it| it.to_str());
                    ext == Some("vtt") || ext == Some("srt")
                });

            fn split_ext(path: &str) -> (&str, Option<&str>) {
                if let Some(index) = path.rfind('.') {
                    let (head, tail) = path.split_at(index);
                    (head, Some(&tail[1..]))
                } else {
                    (path, None)
                }
            }

            let mut conn = self.db.acquire().await?;
            let subs = db::subtitles::get_for_video(&mut conn, id)
                .await?
                .into_iter()
                .filter_map(|it| it.path)
                .collect::<HashSet<_>>();

            for entry in subtitles {
                let Some(sub_path) = Utf8Path::from_path(entry.path()) else { continue };
                if subs.contains(sub_path) {
                    continue;
                }

                let Some(mut sub_file_name) = entry.path().file_stem().and_then(|it|it.to_str()) else {
                    continue
                };

                let format = match entry.path().extension().and_then(|it| it.to_str()) {
                    Some("srt") => "srt",
                    Some("vtt") => "webvtt",
                    _ => continue,
                };

                let mut sdh = false;
                let mut forced = false;

                loop {
                    let (name, ext) = split_ext(sub_file_name);
                    let Some(ext) = ext else { break };

                    if ext == "sdh" {
                        sdh = true;
                    } else if ext == "forced" {
                        forced = true;
                    } else {
                        break;
                    }

                    sub_file_name = name;
                }

                let (sub_file_name, lang) = split_ext(sub_file_name);

                if sub_file_name == video_file_name {
                    let subtitle = db::subtitles::NewSubtitle {
                        video_id: id,
                        stream_index: None,
                        path: Some(sub_path),
                        title: entry.path().to_str(),
                        language: lang,
                        format: Some(format),
                        sdh,
                        forced,
                    };

                    tracing::info!(video_id=%id, path=?entry.path(), "adding subtitle: ");

                    let mut tx = conn.begin().await?;
                    db::subtitles::insert(&mut tx, &subtitle).await?;
                    tx.commit().await?;
                }
            }
        }

        Ok(result)
    }

    async fn scan_movie_file(
        &self,
        path: &Utf8Path,
        options: &ScanOptions,
    ) -> eyre::Result<FileScanResult> {
        let name = path.file_name().unwrap();

        if let Some(id) = self.library.get_movie_id_by_path(path).await? {
            if !path.is_file() {
                // Remove movie from database if file no longer exists
                self.library.remove_movie(id).await?;
                return Ok(FileScanResult::Removed);
            }

            if options.rescan_files {
                self.rescan_video_file_path(id, path).await?;
            }

            return Ok(FileScanResult::Updated(id));
        }

        if !path.is_file() {
            return Ok(FileScanResult::Ignored);
        }

        let Some((title, release_date)) = parse_movie_filename(self.matchers(), name) else {
            return Ok(FileScanResult::Ignored);
        };

        tracing::info!("adding movie: {name}");

        let movie = NewMovie {
            parent_path: path.parent().unwrap(),
            path,
            title: &title,
            release_date: release_date.map(|dt| dt.unix_timestamp()),
        };

        let id = self.library.add_movie(&movie).await?;

        Ok(FileScanResult::Added(id))
    }

    async fn scan_episode_file(
        &self,
        path: &Utf8Path,
        options: &ScanOptions,
    ) -> eyre::Result<FileScanResult> {
        let Some(EpisodePathInfo {
            show_name,
            show_path,
            season,
            episode,
            name,
        }) = parse_episode_path(self.matchers(), path) else {
            return Ok(FileScanResult::Ignored);
        };

        let show_id = self.library.get_show_id_by_path(show_path).await?;
        let season_id = match show_id {
            None => None,
            Some(show_id) => self.library.get_season_id(show_id, season).await?,
        };

        if let Some(season_id) = season_id {
            if let Some(id) = self.library.get_episode_id(season_id, episode).await? {
                if !path.is_file() {
                    // Remove episode from database if file no longer exists
                    self.library.remove_episode(id).await?;
                    // Cleanup any empty shows/seasons after episode removed
                    self.library.remove_empty_collections().await?;
                    return Ok(FileScanResult::Removed);
                }

                if options.rescan_files {
                    self.rescan_video_file_path(id, path).await?;
                }

                return Ok(FileScanResult::Updated(id));
            }
        }

        if !path.is_file() {
            return Ok(FileScanResult::Ignored);
        }

        let show_id = match show_id {
            Some(id) => id,
            None => {
                tracing::info!("adding show: {show_name}");

                let show = NewShow {
                    path: show_path,
                    name: &show_name,
                };

                self.library.add_show(show).await?
            }
        };

        let season_id = match season_id {
            Some(id) => id,
            None => {
                tracing::info!("adding season: {show_name} ({season})");

                let season = NewSeason {
                    show_id,
                    season_number: season,
                };

                self.library.add_season(season).await?
            }
        };

        tracing::info!("adding episode: {show_name} ({season}:{episode})");

        let episode = NewEpisode {
            show_id,
            season_id,
            season_number: season,
            episode_number: episode,
            name: &name.unwrap_or_else(|| format!("S{season:02}E{episode:02}")),
            path,
        };

        let id = self.library.add_episode(episode).await?;

        Ok(FileScanResult::Added(id))
    }

    async fn rescan_video_file_path(&self, id: i64, path: &Utf8Path) -> eyre::Result<()> {
        tracing::debug!(id, %path, "rescanning video file");

        let info = self.video_prober.probe(path).await?;

        let mut transaction = self.db.begin().await?;
        video_info::update_video_info(&mut transaction, id, &info).await?;
        transaction.commit().await?;

        Ok(())
    }

    async fn validate_subtitles(&self) -> eyre::Result<()> {
        let mut conn = self.db.acquire().await?;

        tracing::info!("validating external subtitles");

        let subtitles: Vec<(i64, String)> =
            sqlx::query_as("SELECT id, path FROM subtitles WHERE stream_index IS NULL")
                .fetch_all(&mut conn)
                .await?;

        for (id, path) in subtitles {
            if let Err(e) = tokio::fs::metadata(&path).await
                && e.kind() == io::ErrorKind::NotFound
            {
                tracing::info!(%id, %path, "removing external subtitle");
                sqlx::query("DELETE FROM subtitles WHERE id = ?")
                    .bind(id)
                    .execute(&mut conn)
                    .await?;
            }
        }

        tracing::info!("validating embedded subtitles");

        let sql = "
            SELECT id, path FROM subtitles
            WHERE stream_index IS NOT NULL AND PATH IS NOT NULL
        ";

        let subtitles: Vec<(i64, String)> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

        for (id, path) in subtitles {
            if let Err(e) = tokio::fs::metadata(&path).await
                && e.kind() == io::ErrorKind::NotFound
            {
                tracing::info!(%id, %path, "invalid file for embedded subtitle");
                sqlx::query("UPDATE subtitles SET path = NULL WHERE id = ?")
                    .bind(id)
                    .execute(&mut conn)
                    .await?;
            }
        }

        Ok(())
    }

    fn matchers(&self) -> &[ImportMatcher] {
        &self.config.import.matchers
    }
}

/// Extracts a title and (optional) year from a filename.
fn parse_movie_filename(
    matchers: &'_ [ImportMatcher],
    name: &str,
) -> Option<(String, Option<OffsetDateTime>)> {
    let movie_matchers = matchers
        .iter()
        .filter(|m| m.target == ImportMatcherTarget::Movie);

    match parse_video_filename(movie_matchers, name)? {
        VideoFilenameMeta::Movie { title, year } => {
            let year = year
                .and_then(|year| Date::from_ordinal_date(year as i32, 1).ok())
                .and_then(|date| date.with_hms(0, 0, 0).ok())
                .map(|dt| dt.assume_utc());

            Some((title, year))
        }
        _ => None,
    }
}

/// Recursively searches a directory for video files
fn get_video_files(path: &Utf8Path) -> impl Iterator<Item = Utf8PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let path = Utf8PathBuf::from_path_buf(e.into_path()).ok()?;
            if is_video_file_path(&path) {
                Some(path)
            } else {
                None
            }
        })
}

struct EpisodePathInfo<'a> {
    show_name: String,
    show_path: &'a Utf8Path,
    season: u32,
    episode: u32,
    name: Option<String>,
}

/// Extracts a show name, season and episode number from an episode path.
fn parse_episode_path<'a>(
    matchers: &'_ [ImportMatcher],
    path: &'a Utf8Path,
) -> Option<EpisodePathInfo<'a>> {
    let parent_path = path.parent()?;
    let parent_is_season = parent_path
        .file_name()
        .map(|name| name.starts_with("Season "))
        .unwrap_or(false);

    let show_path = if parent_is_season {
        parent_path.parent()?
    } else {
        parent_path
    };

    let file_name = path.file_name()?;
    let show_folder_name = show_path.file_name()?;

    let episode_matchers = matchers
        .iter()
        .filter(|m| m.target == ImportMatcherTarget::Episode);

    match parse_video_filename(episode_matchers, file_name)? {
        VideoFilenameMeta::Episode {
            show_name,
            name,
            season,
            episode,
        } => Some(EpisodePathInfo {
            show_name: show_name.unwrap_or_else(|| show_folder_name.to_owned()),
            show_path,
            season,
            episode,
            name,
        }),
        _ => None,
    }
}

enum VideoFilenameMeta {
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

fn is_video_file_path(path: &Utf8Path) -> bool {
    const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv"];

    let Some(ext) = path.extension() else {
        return false;
    };

    VIDEO_EXTENSIONS.contains(&ext)
}
