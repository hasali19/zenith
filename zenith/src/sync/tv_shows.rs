use std::collections::{HashMap, HashSet};
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::ffmpeg::Ffprobe;
use crate::fs::{DirEntryType, FileSystem};
use crate::library::{MediaLibrary, NewEpisode, NewSeason, NewShow};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^S(\d\d)E(\d\d)\.\S+$").unwrap();
}

pub(super) async fn sync_tv_shows(
    library: &impl MediaLibrary,
    fs: &impl FileSystem,
    ffprobe: &Ffprobe,
    path: &str,
) -> eyre::Result<()> {
    let mut tv_shows = library
        .get_show_ids()
        .await?
        .into_iter()
        .collect::<HashSet<_>>();

    for entry in fs.list_dir(Path::new(path))? {
        if !matches!(entry.entry_type, DirEntryType::Directory) {
            continue;
        }

        let file_path = entry.path;
        let file_name = match file_path.file_name().and_then(|v| v.to_str()) {
            Some(name) => name,
            None => continue,
        };

        let episodes = find_episodes(fs, &file_path)?;
        if episodes.is_empty() {
            continue;
        }

        let path = file_path.to_str().unwrap().to_owned();
        let id = sync_show(library, ffprobe, file_name, &path, &episodes).await?;

        tv_shows.remove(&id);
    }

    for id in tv_shows {
        library.remove_show(id).await?;
    }

    Ok(())
}

fn find_episodes(
    fs: &impl FileSystem,
    path: &Path,
) -> eyre::Result<HashMap<i32, Vec<(i32, String)>>> {
    let mut episodes = HashMap::new();

    for entry in fs.list_dir(path)? {
        if !matches!(entry.entry_type, DirEntryType::File) {
            continue;
        }

        let file_path = entry.path;
        let file_name = match file_path.file_name().and_then(|v| v.to_str()) {
            Some(name) => name,
            None => continue,
        };

        if !file_name.ends_with(".mkv") && !file_name.ends_with(".mp4") {
            continue;
        }

        let captures: regex::Captures = match REGEX.captures(file_name) {
            Some(captures) => captures,
            None => continue,
        };

        let season: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let episode: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

        episodes
            .entry(season)
            .or_insert_with(Vec::new)
            .push((episode, file_path.to_str().unwrap().to_owned()));
    }

    Ok(episodes)
}

async fn sync_show(
    library: &impl MediaLibrary,
    ffprobe: &Ffprobe,
    name: &str,
    path: &str,
    episodes: &HashMap<i32, Vec<(i32, String)>>,
) -> eyre::Result<i64> {
    let res = library.get_show_id(path).await?;

    let id = match res {
        Some(id) => id,
        None => {
            log::info!("adding tv show: {}", name);
            library.add_show(NewShow { path, name }).await?
        }
    };

    let mut seasons = library
        .get_season_ids(id)
        .await?
        .into_iter()
        .collect::<HashSet<_>>();

    for (season, episodes) in episodes {
        seasons.remove(&sync_season(library, ffprobe, id, *season, episodes).await?);
    }

    for season in seasons {
        library.remove_season(season).await?;
    }

    Ok(id)
}

async fn sync_season(
    library: &impl MediaLibrary,
    ffprobe: &Ffprobe,
    show_id: i64,
    season: i32,
    episodes: &[(i32, String)],
) -> eyre::Result<i64> {
    let res = library.get_season_id(show_id, season).await?;

    let id = match res {
        Some(id) => id,
        None => {
            log::info!("adding tv season: {} (show_id: {})", season, show_id);

            let season = NewSeason {
                show_id,
                season_number: season,
            };

            library.add_season(season).await?
        }
    };

    let mut episode_ids = library
        .get_episode_ids(id)
        .await?
        .into_iter()
        .collect::<HashSet<i64>>();

    for (episode, path) in episodes {
        match sync_episode(library, ffprobe, id, *episode, path).await {
            Ok(id) => {
                episode_ids.remove(&id);
            }
            Err(e) => {
                log::warn!("{}", e);
            }
        }
    }

    for episode in episode_ids {
        library.remove_episode(episode).await?;
    }

    Ok(id)
}

async fn sync_episode(
    library: &impl MediaLibrary,
    ffprobe: &Ffprobe,
    season_id: i64,
    episode: i32,
    path: &str,
) -> eyre::Result<i64> {
    let res = library.get_episode_id(season_id, episode).await?;

    let id = match res {
        Some(id) => id,
        None => {
            log::info!("adding tv episode: {} (season_id: {})", episode, season_id);

            let info = ffprobe.get_video_info(path).await?;
            let duration: f64 = info.format.duration.parse()?;

            let episode = NewEpisode {
                season_id,
                episode_number: episode,
                path,
                duration,
            };

            library.add_episode(episode).await?
        }
    };

    Ok(id)
}
