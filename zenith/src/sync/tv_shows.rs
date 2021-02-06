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
    let mut tv_shows = sqlx::query("SELECT item_id FROM tv_shows")
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
    let res = sqlx::query_scalar("SELECT item_id FROM tv_shows WHERE path = ?")
        .bind(path)
        .fetch_optional(&mut *db)
        .await?;

    let id = match res {
        Some(id) => id,
        None => {
            log::info!("adding tv show: {}", name);

            let sql = "
                INSERT INTO media_items (item_type)
                VALUES (?)
            ";

            let id: i64 = sqlx::query(sql)
                .bind(MediaItemType::TvShow)
                .execute(&mut *db)
                .await?
                .last_insert_rowid();

            let sql = "
                INSERT INTO tv_shows (item_id, path, name)
                VALUES (?, ?, ?)
            ";

            sqlx::query(sql)
                .bind(id)
                .bind(path)
                .bind(name)
                .execute(&mut *db)
                .await?;

            metadata.enqueue(RefreshRequest::TvShow(id));

            id
        }
    };

    let mut seasons = sqlx::query_scalar("SELECT item_id FROM tv_seasons WHERE show_id = ?")
        .bind(id)
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
        SELECT item_id FROM tv_seasons
        WHERE show_id = ? AND season_number = ?
    ";

    let res = sqlx::query_scalar(sql)
        .bind(show_id)
        .bind(season)
        .fetch_optional(&mut *db)
        .await?;

    let id = match res {
        Some(id) => id,
        None => {
            log::info!("adding tv season: {} (show_id: {})", season, show_id);

            let sql = "
                INSERT INTO media_items (item_type)
                VALUES (?)
            ";

            let id: i64 = sqlx::query(sql)
                .bind(MediaItemType::TvSeason)
                .execute(&mut *db)
                .await?
                .last_insert_rowid();

            let sql = "
                INSERT INTO tv_seasons (item_id, show_id, season_number)
                VALUES (?, ?, ?)
            ";

            sqlx::query(sql)
                .bind(id)
                .bind(show_id)
                .bind(season)
                .execute(&mut *db)
                .await?;

            metadata.enqueue(RefreshRequest::TvSeason(id));

            id
        }
    };

    let mut episode_ids = sqlx::query_scalar("SELECT item_id FROM tv_episodes WHERE season_id = ?")
        .bind(id)
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
        SELECT item_id FROM tv_episodes
        WHERE season_id = ? AND episode_number = ?
    ";

    let res = sqlx::query_scalar(sql)
        .bind(season_id)
        .bind(episode)
        .fetch_optional(&mut *db)
        .await?;

    let id = match res {
        Some(id) => id,
        None => {
            log::info!("adding tv episode: {} (season_id: {})", episode, season_id);

            let info = ffprobe.get_video_info(path).await?;
            let duration: f64 = info.format.duration.parse()?;

            let sql = "
                INSERT INTO media_items (item_type)
                VALUES (?)
            ";

            let id: i64 = sqlx::query(sql)
                .bind(MediaItemType::TvEpisode)
                .execute(&mut *db)
                .await?
                .last_insert_rowid();

            let sql = "
                INSERT INTO tv_episodes (item_id, season_id, episode_number)
                VALUES (?, ?, ?)
            ";

            sqlx::query(sql)
                .bind(id)
                .bind(season_id)
                .bind(episode)
                .execute(&mut *db)
                .await?;

            let sql = "
                INSERT INTO video_files (item_id, path, duration)
                VALUES (?, ?, ?)
            ";

            sqlx::query(sql)
                .bind(id)
                .bind(&path)
                .bind(duration)
                .execute(&mut *db)
                .await?;

            metadata.enqueue(RefreshRequest::TvEpisode(id));

            id
        }
    };

    Ok(id)
}

async fn remove_show(db: &mut SqliteConnection, id: i64) -> eyre::Result<()> {
    log::info!("removing tv show: {}", id);

    let seasons: Vec<i64> = sqlx::query_scalar("SELECT item_id FROM tv_seasons WHERE show_id = ?")
        .bind(id)
        .fetch_all(&mut *db)
        .await?;

    for season in seasons {
        remove_season(&mut *db, season).await?;
    }

    sqlx::query("DELETE FROM tv_shows WHERE item_id = ?")
        .bind(id)
        .execute(&mut *db)
        .await?;

    sqlx::query("DELETE FROM media_items WHERE id = ?")
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}

async fn remove_season(db: &mut SqliteConnection, id: i64) -> eyre::Result<()> {
    log::info!("removing tv season: {}", id);

    let episodes: Vec<i64> =
        sqlx::query_scalar("SELECT item_id FROM tv_episodes WHERE season_id = ?")
            .bind(id)
            .fetch_all(&mut *db)
            .await?;

    for episode in episodes {
        remove_episode(&mut *db, episode).await?;
    }

    sqlx::query("DELETE FROM tv_seasons WHERE item_id = ?")
        .bind(id)
        .execute(&mut *db)
        .await?;

    sqlx::query("DELETE FROM media_items WHERE id = ?")
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}

async fn remove_episode(db: &mut SqliteConnection, id: i64) -> eyre::Result<()> {
    log::info!("removing tv episode: {}", id);

    sqlx::query("DELETE FROM video_files WHERE item_id = ?")
        .bind(id)
        .execute(&mut *db)
        .await?;

    sqlx::query("DELETE FROM tv_episodes WHERE item_id = ?")
        .bind(id)
        .execute(&mut *db)
        .await?;

    sqlx::query("DELETE FROM media_items WHERE id = ?")
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}
