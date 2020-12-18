use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqliteConnection};
use std::collections::HashSet;

use crate::db;
use crate::db::movies::NewMovie;

pub async fn sync_movies(db: &mut SqliteConnection, path: &str) -> eyre::Result<()> {
    // Find all existing movies
    let mut movies = sqlx::query("SELECT id FROM movies")
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

        match db::movies::get_id_for_path(&mut *db, path).await? {
            // Remove from the set of ids if the movie exists
            Some(id) => {
                movies.remove(&id);
            }
            // Otherwise create a new movie
            None => {
                log::info!("found movie: {}", file_name);

                let movie = NewMovie {
                    path,
                    title: file_name,
                    video_path: &video_file,
                };

                db::movies::create(&mut *db, &movie).await?;
            }
        }
    }

    // Any remaining ids no longer exist on the filesystem, so
    // may be deleted from the database
    for id in movies {
        log::info!("removing movie: {}", id);

        sqlx::query("DELETE FROM movies WHERE id = ?")
            .bind(id)
            .execute(&mut *db)
            .await?;
    }

    Ok(())
}
