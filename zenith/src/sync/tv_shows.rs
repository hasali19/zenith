use std::collections::HashSet;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use sqlx::{Connection, SqliteConnection, Transaction};

use crate::db::tv_shows::{self, NewTvEpisode, NewTvShow};
use crate::tmdb::{self, TmdbClient};

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

async fn get_ids_by_path(
    db: &mut SqliteConnection,
    path: &str,
) -> sqlx::Result<Option<(i64, Option<i32>)>> {
    sqlx::query_as("SELECT id, tmdb_id FROM tv_shows WHERE path = ?")
        .bind(path)
        .fetch_optional(db)
        .await
}

async fn sync_show(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    name: &str,
    path: &str,
    episodes: &[(u32, u32, String)],
) -> eyre::Result<i64> {
    let mut transaction = db.begin().await?;

    let (id, tmdb_id) = match get_ids_by_path(&mut transaction, &path).await? {
        Some(ids) => ids,
        None => {
            log::info!("found tv show: {}", name);

            let mut tv_show = NewTvShow {
                path,
                name,
                overview: None,
                poster_url: None,
                backdrop_url: None,
                tmdb_id: None,
            };

            let metadata = get_show_metadata(tmdb, name).await;
            if let Some(metadata) = &metadata {
                tv_show.name = &metadata.name;
                tv_show.overview = metadata.overview.as_deref();
                tv_show.poster_url = metadata.poster_path.as_deref();
                tv_show.backdrop_url = metadata.backdrop_path.as_deref();
                tv_show.tmdb_id = Some(metadata.id);
            }

            let id = tv_shows::create(&mut transaction, &tv_show).await?;
            let tmdb_id = metadata.map(|m| m.id);

            (id, tmdb_id)
        }
    };

    sync_episodes(&mut transaction, tmdb, id, tmdb_id, &episodes).await?;

    transaction.commit().await?;

    Ok(id)
}

async fn sync_episodes(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    show_id: i64,
    tmdb_id: Option<i32>,
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

                let mut new_episode = NewTvEpisode {
                    season: *season,
                    episode: *episode,
                    overview: None,
                    image_url: None,
                    tmdb_id: None,
                    video_path: path,
                };

                let metadata = match tmdb_id {
                    Some(tmdb_id) => get_episode_metadata(tmdb, tmdb_id, *season, *episode).await,
                    None => None,
                };

                if let Some((metadata, image)) = &metadata {
                    new_episode.overview = metadata.overview.as_deref();
                    new_episode.tmdb_id = Some(metadata.id);

                    if let Some(image) = image {
                        new_episode.image_url = Some(&image.file_path);
                    }
                }

                tv_shows::create_episode(&mut *db, show_id, &new_episode).await?;
            }
        }
    }

    for id in ids {
        tv_shows::delete_episode(&mut *db, id).await?;
    }

    Ok(())
}

async fn get_show_metadata(tmdb: &TmdbClient, name: &str) -> Option<tmdb::TvShowSearchResult> {
    let query = tmdb::TvShowSearchQuery {
        name,
        page: None,
        first_air_date_year: None,
    };

    let metadata = match tmdb.search_tv_shows(&query).await {
        Ok(metadata) => metadata,
        Err(_) => return None,
    };

    metadata.results.into_iter().next()
}

async fn get_episode_metadata(
    tmdb: &TmdbClient,
    show_id: i32,
    season: u32,
    episode: u32,
) -> Option<(tmdb::TvEpisodeResponse, Option<tmdb::Image>)> {
    let metadata = tmdb
        .get_tv_episode(show_id, season as i32, episode as i32)
        .await
        .ok()?;

    let image = tmdb
        .get_tv_episode_images(show_id, season as i32, episode as i32)
        .await
        .ok()?
        .stills
        .into_iter()
        .next();

    Some((metadata, image))
}
