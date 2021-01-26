use std::collections::HashSet;

use metadata::{MetadataManager, RefreshRequest};
use regex::Regex;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqliteConnection};

use crate::ffmpeg::Ffprobe;
use crate::metadata;

pub async fn sync_movies(
    db: &mut SqliteConnection,
    metadata: &MetadataManager,
    ffprobe: &Ffprobe,
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

        for entry in std::fs::read_dir(entry.path())? {
            let entry = entry?;

            if !entry.file_type().unwrap().is_file() {
                continue;
            }

            let file_path = entry.path();
            let file_name = match file_path.file_name().and_then(|v| v.to_str()) {
                Some(name) => name,
                None => continue,
            };

            let (title, year) = match parse_movie_file_name(file_name) {
                Some(v) => v,
                None => continue,
            };

            let path = entry.path();
            let path = match path.to_str() {
                Some(path) => path,
                None => continue,
            };

            match get_id_for_path(&mut *db, &path).await? {
                // Remove from the set of ids if the movie exists
                Some(id) => {
                    movies.remove(&id);
                }
                // Otherwise create a new movie
                None => {
                    log::info!("adding movie: {}", file_name);

                    let info = match ffprobe.get_video_info(&path).await {
                        Ok(ingo) => ingo,
                        Err(e) => {
                            log::warn!("{}", e);
                            continue;
                        }
                    };

                    let release_date = year
                        .and_then(|year| time::Date::try_from_yo(year, 1).ok())
                        .and_then(|date| date.try_with_hms(0, 0, 0).ok())
                        .map(|dt| dt.assume_utc().unix_timestamp());

                    let sql = "
                        INSERT INTO media_items (item_type, path, name, release_date, duration)
                        VALUES (1, ?, ?, ?, ?)
                    ";

                    let id: i64 = sqlx::query(sql)
                        .bind(&path)
                        .bind(&title)
                        .bind(release_date)
                        .bind(info.duration)
                        .execute(&mut *db)
                        .await?
                        .last_insert_rowid();

                    metadata.enqueue(RefreshRequest::Movie(id));
                }
            }
        }
    }

    // Any remaining ids no longer exist on the filesystem, so
    // may be deleted from the database
    for id in movies {
        log::info!("removing movie: {}", id);

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

pub fn parse_movie_file_name(name: &str) -> Option<(String, Option<i32>)> {
    lazy_static::lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^(\S.*?)(?: \((\d\d\d\d)\))?\.(mkv|mp4)$").unwrap();
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
