use std::collections::{HashMap, HashSet};
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqliteConnection};

use crate::db::media::MediaItemType;
use crate::ffmpeg::Ffprobe;
use crate::metadata::{MetadataManager, RefreshRequest};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^S(\d\d)E(\d\d)\.\S+$").unwrap();
}

pub async fn sync_tv_shows(
    db: &mut SqliteConnection,
    metadata: &MetadataManager,
    ffprobe: &Ffprobe,
    path: &str,
) -> eyre::Result<()> {
    let mut tv_shows = sqlx::query("SELECT id FROM media_items WHERE item_type = 2")
        .try_map(|row: SqliteRow| row.try_get::<i64, _>(0))
        .fetch_all(&mut *db)
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
        let id = sync_show(&mut *db, metadata, ffprobe, file_name, &path, &episodes).await?;

        tv_shows.remove(&id);
    }

    for id in tv_shows {
        remove_show(&mut *db, id).await?;
    }

    Ok(())
}

fn find_episodes(path: &Path) -> eyre::Result<HashMap<i32, Vec<(i32, String)>>> {
    let mut episodes = HashMap::new();

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
    db: &mut SqliteConnection,
    metadata: &MetadataManager,
    ffprobe: &Ffprobe,
    name: &str,
    path: &str,
    episodes: &HashMap<i32, Vec<(i32, String)>>,
) -> eyre::Result<i64> {
    let sql = "
        SELECT id FROM media_items
        WHERE path = ? AND item_type = ?
    ";

    let res = sqlx::query_as(sql)
        .bind(path)
        .bind(MediaItemType::TvShow)
        .fetch_optional(&mut *db)
        .await?;

    let id = match res {
        Some((id,)) => id,
        None => {
            log::info!("adding tv show: {}", name);

            let sql = "
                INSERT INTO media_items (item_type, path, name)
                VALUES (?, ?, ?)
            ";

            let res = sqlx::query(sql)
                .bind(MediaItemType::TvShow)
                .bind(path)
                .bind(name)
                .execute(&mut *db)
                .await?;

            let id = res.last_insert_rowid();

            metadata.enqueue(RefreshRequest::TvShow(id));

            id
        }
    };

    let mut seasons = sqlx::query("SELECT id FROM media_items WHERE parent_id = ?")
        .bind(id)
        .try_map(|row: SqliteRow| row.try_get::<i64, _>(0))
        .fetch_all(&mut *db)
        .await?
        .into_iter()
        .collect::<HashSet<_>>();

    for (season, episodes) in episodes {
        seasons.remove(&sync_season(db, metadata, ffprobe, id, *season, episodes).await?);
    }

    for season in seasons {
        remove_season(&mut *db, season).await?;
    }

    Ok(id)
}

async fn sync_season(
    db: &mut SqliteConnection,
    metadata: &MetadataManager,
    ffprobe: &Ffprobe,
    show_id: i64,
    season: i32,
    episodes: &[(i32, String)],
) -> eyre::Result<i64> {
    let sql = "
        SELECT id FROM media_items
        WHERE parent_id = ? AND index_number = ? AND item_type = ?
    ";

    let res = sqlx::query_as(sql)
        .bind(show_id)
        .bind(season)
        .bind(MediaItemType::TvSeason)
        .fetch_optional(&mut *db)
        .await?;

    let id = match res {
        Some((id,)) => id,
        None => {
            log::info!("adding tv season: {} (show_id: {})", season, show_id);

            let sql = "
                INSERT INTO media_items (parent_id, item_type, index_number)
                VALUES (?, ?, ?)
            ";

            let res = sqlx::query(sql)
                .bind(show_id)
                .bind(MediaItemType::TvSeason)
                .bind(season)
                .execute(&mut *db)
                .await?;

            let id = res.last_insert_rowid();

            metadata.enqueue(RefreshRequest::TvSeason(id));

            id
        }
    };

    let mut episode_ids = sqlx::query("SELECT id FROM media_items WHERE parent_id = ?")
        .bind(id)
        .try_map(|row: SqliteRow| row.try_get::<i64, _>(0))
        .fetch_all(&mut *db)
        .await?
        .into_iter()
        .collect::<HashSet<i64>>();

    for (episode, path) in episodes {
        match sync_episode(db, metadata, ffprobe, id, *episode, path).await {
            Ok(id) => {
                episode_ids.remove(&id);
            }
            Err(e) => {
                log::warn!("{}", e);
            }
        }
    }

    for episode in episode_ids {
        remove_episode(&mut *db, episode).await?;
    }

    Ok(id)
}

async fn sync_episode(
    db: &mut SqliteConnection,
    metadata: &MetadataManager,
    ffprobe: &Ffprobe,
    season_id: i64,
    episode: i32,
    path: &str,
) -> eyre::Result<i64> {
    let sql = "
        SELECT id FROM media_items
        WHERE parent_id = ? AND index_number = ? AND item_type = ?
    ";

    let res = sqlx::query_as(sql)
        .bind(season_id)
        .bind(episode)
        .bind(MediaItemType::TvEpisode)
        .fetch_optional(&mut *db)
        .await?;

    let id = match res {
        Some((id,)) => id,
        None => {
            log::info!("adding tv episode: {} (season_id: {})", episode, season_id);

            let info = ffprobe.get_video_info(path).await?;

            let sql = "
                INSERT INTO media_items (parent_id, item_type, path, index_number, duration)
                VALUES (?, ?, ?, ?, ?)
            ";

            let res = sqlx::query(sql)
                .bind(season_id)
                .bind(MediaItemType::TvEpisode)
                .bind(path)
                .bind(episode)
                .bind(info.duration)
                .execute(&mut *db)
                .await?;

            let id = res.last_insert_rowid();

            metadata.enqueue(RefreshRequest::TvEpisode(id));

            id
        }
    };

    Ok(id)
}

async fn remove_show(db: &mut SqliteConnection, id: i64) -> eyre::Result<()> {
    log::info!("removing tv show: {}", id);

    let seasons: Vec<(i64,)> = sqlx::query_as("SELECT id FROM media_items WHERE parent_id = ?")
        .bind(id)
        .fetch_all(&mut *db)
        .await?;

    for (season,) in seasons {
        remove_season(&mut *db, season).await?;
    }

    sqlx::query("DELETE FROM media_items WHERE id = ?")
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}

async fn remove_season(db: &mut SqliteConnection, id: i64) -> eyre::Result<()> {
    log::info!("removing tv season: {}", id);

    let episodes: Vec<(i64,)> = sqlx::query_as("SELECT id FROM media_items WHERE parent_id = ?")
        .bind(id)
        .fetch_all(&mut *db)
        .await?;

    for (episode,) in episodes {
        remove_episode(&mut *db, episode).await?;
    }

    sqlx::query("DELETE FROM media_items WHERE id = ?")
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}

async fn remove_episode(db: &mut SqliteConnection, id: i64) -> eyre::Result<()> {
    log::info!("removing tv episode: {}", id);

    sqlx::query("DELETE FROM media_items WHERE id = ?")
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}
