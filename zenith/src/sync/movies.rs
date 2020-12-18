use std::collections::HashSet;

use regex::Regex;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqliteConnection};

use crate::db;
use crate::db::movies::NewMovie;
use crate::tmdb::{self, MovieSearchQuery, TmdbClient};

pub async fn sync_movies(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    path: &str,
) -> eyre::Result<()> {
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

        match db::movies::get_id_for_path(&mut *db, path).await? {
            // Remove from the set of ids if the movie exists
            Some(id) => {
                movies.remove(&id);
            }
            // Otherwise create a new movie
            None => {
                log::info!("found movie: {}", file_name);

                let mut movie = NewMovie {
                    path,
                    title: &title,
                    year,
                    overview: None,
                    poster_url: None,
                    backdrop_url: None,
                    video_path: &video_file,
                };

                let metadata = get_metadata(tmdb, &title).await;
                if let Some(metadata) = &metadata {
                    movie.title = &metadata.title;
                    movie.overview = metadata.overview.as_deref();
                    movie.poster_url = metadata.poster_path.as_deref();
                    movie.backdrop_url = metadata.backdrop_path.as_deref();
                }

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

fn parse_movie_dir_name(name: &str) -> Option<(String, Option<i32>)> {
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

async fn get_metadata(tmdb: &TmdbClient, title: &str) -> Option<tmdb::MovieSearchResult> {
    let query = MovieSearchQuery {
        title,
        page: None,
        primary_release_year: None,
    };

    let metadata = match tmdb.search_movies(&query).await {
        Ok(metadata) => metadata,
        Err(_) => return None,
    };

    metadata.results.into_iter().next()
}
