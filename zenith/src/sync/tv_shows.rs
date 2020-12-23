use std::collections::HashSet;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use sqlx::{Connection, SqliteConnection, Transaction};

use crate::db::tv_shows::{self, NewTvEpisode, NewTvShow};
use crate::metadata;
use crate::tmdb::TmdbClient;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^S(\d\d)E(\d\d)\.\S+$").unwrap();
}

pub async fn sync_tv_shows(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    path: &str,
) -> eyre::Result<()> {
    let mut tv_shows = tv_shows::get_all_ids(&mut *db)
        .await?
        .into_iter()
        .collect::<HashSet<_>>();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;

        if !entry.file_type().unwrap().is_dir() {
            continue;
        }

        let file_path = entry.path();
        let file_name = match file_path.file_name().and_then(|v| v.to_str()) {
            Some(name) => name,
            None => continue,
        };

        let episodes = find_episodes(&file_path)?;
        if episodes.is_empty() {
            continue;
        }

        let path = file_path.to_str().unwrap().to_owned();
        let id = sync_show(&mut *db, tmdb, file_name, &path, &episodes).await?;

        tv_shows.remove(&id);
    }

    for id in tv_shows {
        log::info!("removing tv show: {}", id);
        let mut transaction: Transaction<_> = db.begin().await?;
        tv_shows::delete(&mut transaction, id).await?;
        transaction.commit().await?;
    }

    Ok(())
}

fn find_episodes(path: &Path) -> eyre::Result<Vec<(u32, u32, String)>> {
    let mut episodes = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;

        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        let file_path = entry.path();
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

        let season: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let episode: u32 = captures.get(2).unwrap().as_str().parse().unwrap();

        episodes.push((season, episode, file_path.to_str().unwrap().to_owned()));
    }

    Ok(episodes)
}

async fn sync_show(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    name: &str,
    path: &str,
    episodes: &[(u32, u32, String)],
) -> eyre::Result<i64> {
    let id = match tv_shows::get_id_for_path(&mut *db, &path).await? {
        Some(id) => id,
        None => {
            log::info!("found tv show: {}", name);
            let tv_show = NewTvShow { path, name };
            let id = tv_shows::create(&mut *db, &tv_show).await?;

            if let Err(e) = metadata::refresh_tv_show_metadata(&mut *db, tmdb, id).await {
                log::error!("failed to update metadata: {}", e);
            }

            id
        }
    };

    sync_episodes(&mut *db, tmdb, id, &episodes).await?;

    Ok(id)
}

async fn sync_episodes(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    show_id: i64,
    episodes: &[(u32, u32, String)],
) -> sqlx::Result<()> {
    let mut ids = tv_shows::get_episode_ids(&mut *db, show_id)
        .await?
        .into_iter()
        .collect::<HashSet<_>>();

    for (season, episode, path) in episodes {
        let id = tv_shows::get_episode_id_for_number(&mut *db, show_id, *season, *episode).await?;
        match id {
            Some(id) => {
                ids.remove(&id);
            }
            None => {
                log::info!(
                    "found tv episode: S{:02}E{:02} (show_id: {})",
                    season,
                    episode,
                    show_id
                );

                let new_episode = NewTvEpisode {
                    season: *season,
                    episode: *episode,
                    video_path: path,
                };

                let id = tv_shows::create_episode(&mut *db, show_id, &new_episode).await?;

                if let Err(e) = metadata::refresh_tv_episode_metadata(&mut *db, tmdb, id).await {
                    log::error!("failed to update metadata: {}", e);
                }
            }
        }
    }

    for id in ids {
        tv_shows::delete_episode(&mut *db, id).await?;
    }

    Ok(())
}
