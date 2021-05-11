use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use once_cell::sync::OnceCell;
use regex::{Captures, Regex};
use time::{Date, OffsetDateTime};
use walkdir::{DirEntry, WalkDir};

use crate::config::Config;
use crate::library::movies::{MovieLibrary, NewMovie};
use crate::library::shows::{NewEpisode, NewSeason, NewShow, ShowLibrary};
use crate::library::MediaLibrary;
use crate::metadata::{MetadataManager, RefreshRequest};

pub struct LibraryScanner {
    library: Arc<MediaLibrary>,
    metadata: MetadataManager,
    config: Arc<Config>,
    is_running: Arc<AtomicBool>,
}

impl LibraryScanner {
    pub fn new(
        library: Arc<MediaLibrary>,
        metadata: MetadataManager,
        config: Arc<Config>,
    ) -> LibraryScanner {
        LibraryScanner {
            library,
            metadata,
            config,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Starts a library scan if one is not already running
    pub fn start_scan(&self) {
        if !self.is_running.swap(true, Ordering::SeqCst) {
            let library = self.library.clone();
            let metadata = self.metadata.clone();
            let config = self.config.clone();
            let is_running = self.is_running.clone();

            tokio::spawn(async move {
                tracing::info!("starting library scan");

                if let Err(e) =
                    scan_movies(library.movies(), &metadata, &config.libraries.movies).await
                {
                    tracing::error!("{}", e);
                };

                if let Err(e) =
                    scan_shows(library.shows(), &metadata, &config.libraries.tv_shows).await
                {
                    tracing::error!("{}", e);
                };

                is_running.store(false, Ordering::SeqCst);

                tracing::info!("scan complete");
            });
        }
    }
}

/// Recursively scans a folder for new movie files
pub async fn scan_movies(
    library: &MovieLibrary,
    metadata: &MetadataManager,
    path: &str,
) -> eyre::Result<()> {
    library.validate().await?;

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(is_video_file)
    {
        let name = entry.path().file_name().and_then(|v| v.to_str()).unwrap();
        let path = entry.path().to_str().unwrap();

        if library.exists_by_path(path).await? {
            continue;
        }

        let (title, release_date) = match parse_movie_filename(name) {
            Some(v) => v,
            None => return Ok(()),
        };

        tracing::info!("adding movie: {}", name);

        let movie = NewMovie {
            path,
            title: &title,
            release_date: release_date.map(|dt| dt.unix_timestamp()),
        };

        let id = library.add_movie(&movie).await?;
        metadata.enqueue(RefreshRequest::Movie(id));
    }

    Ok(())
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
        .and_then(|year| Date::try_from_yo(year, 1).ok())
        .and_then(|date| date.try_with_hms(0, 0, 0).ok())
        .map(|dt| dt.assume_utc());

    Some((name, year))
}

/// Recursively scans a folder for tv shows
pub async fn scan_shows(
    library: &ShowLibrary,
    metadata: &MetadataManager,
    path: &str,
) -> eyre::Result<()> {
    library.validate().await?;

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(is_video_file)
    {
        let (show_name, season, episode) = match parse_episode_path(entry.path()) {
            Some(v) => v,
            None => continue,
        };

        let path = match entry.path().to_str() {
            Some(v) => v,
            None => continue,
        };

        let parent_path = match entry.path().parent().and_then(|v| v.to_str()) {
            Some(v) => v,
            None => continue,
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
                metadata.enqueue(RefreshRequest::TvShow(id));
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
                metadata.enqueue(RefreshRequest::TvSeason(id));
                id
            }
        };

        if library.get_episode_id(season_id, episode).await?.is_none() {
            tracing::info!("adding episode: {} ({}:{})", show_name, season, episode);

            let episode = NewEpisode {
                season_id,
                episode_number: episode,
                path,
            };

            let id = library.add_episode(episode).await?;
            metadata.enqueue(RefreshRequest::TvEpisode(id));
        }
    }

    Ok(())
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
