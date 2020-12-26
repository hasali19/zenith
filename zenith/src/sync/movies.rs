use std::collections::HashSet;

use regex::Regex;
use sqlx::sqlite::SqliteRow;
use sqlx::{Connection, Row, SqliteConnection};

use crate::tmdb::TmdbClient;
use crate::{ffmpeg, metadata};

pub async fn sync_movies(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    path: &str,
) -> eyre::Result<()> {
    // Find all existing movies
    let mut movies = sqlx::query("SELECT id FROM media_items WHERE item_type = 1")
        .try_map(|row: SqliteRow| row.try_get::<i64, _>(0))
        .fetch_all(&mut *db)
        .await?
        .into_iter()
        .collect::<HashSet<_>>();

    // Search for all movies on the filesystem
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

        let path = match file_path.to_str() {
            Some(path) => path,
            None => continue,
        };

        let (title, year) = match parse_movie_dir_name(file_name) {
            Some(v) => v,
            None => continue,
        };

        // Just get the first video file found for now
        let video_file = std::fs::read_dir(path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                let path = path.to_str()?;

                if !path.ends_with(".mkv") && !path.ends_with(".mp4") {
                    return None;
                }

                Some(path.to_owned())
            })
            .next();

        let video_file = match video_file {
            Some(video_file) => video_file,
            None => continue,
        };

        match get_id_for_path(&mut *db, path).await? {
            // Remove from the set of ids if the movie exists
            Some(id) => {
                movies.remove(&id);
            }
            // Otherwise create a new movie
            None => {
                log::info!("adding movie: {}", file_name);

                let info = ffmpeg::get_video_info(&video_file).await?;

                let release_date = year
                    .and_then(|year| time::Date::try_from_yo(year, 1).ok())
                    .and_then(|date| date.try_with_hms(0, 0, 0).ok())
                    .map(|dt| dt.assume_utc().unix_timestamp());

                let mut transaction = db.begin().await?;

                let sql = "
                    INSERT INTO media_items (item_type, path, name, release_date)
                    VALUES (1, ?, ?, ?)
                ";

                let id: i64 = sqlx::query(sql)
                    .bind(path)
                    .bind(&title)
                    .bind(release_date)
                    .execute(&mut transaction)
                    .await?
                    .last_insert_rowid();

                let sql = "
                    INSERT INTO video_files (item_id, path, duration)
                    VALUES (last_insert_rowid(), ?, ?)
                ";

                sqlx::query(sql)
                    .bind(video_file)
                    .bind(info.duration)
                    .execute(&mut transaction)
                    .await?;

                transaction.commit().await?;

                if let Err(e) = metadata::refresh_movie_metadata(&mut *db, tmdb, id).await {
                    log::error!("failed to update metadata: {}", e);
                }
            }
        }
    }

    // Any remaining ids no longer exist on the filesystem, so
    // may be deleted from the database
    for id in movies {
        log::info!("removing movie: {}", id);

        sqlx::query("DELETE FROM video_files WHERE item_id = ?")
            .bind(id)
            .execute(&mut *db)
            .await?;

        sqlx::query("DELETE FROM media_items WHERE id = ?")
            .bind(id)
            .execute(&mut *db)
            .await?;
    }

    Ok(())
}

async fn get_id_for_path(db: &mut SqliteConnection, path: &str) -> sqlx::Result<Option<i64>> {
    sqlx::query("SELECT id FROM media_items WHERE item_type = 1 AND path = ?")
        .bind(path)
        .try_map(|row: SqliteRow| row.try_get(0))
        .fetch_optional(db)
        .await
}

pub fn parse_movie_dir_name(name: &str) -> Option<(String, Option<i32>)> {
    lazy_static::lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^(\S.*?)(?: \((\d\d\d\d)\))?$").unwrap();
    }

    REGEX.captures(name).map(|captures| {
        let name = captures.get(1).unwrap().as_str().to_owned();
        let year = captures
            .get(2)
            .map(|m| m.as_str().parse::<i32>().ok())
            .flatten();

        (name, year)
    })
}
