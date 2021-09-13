use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request};
use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};

use crate::db::Db;
use crate::utils;

use super::common::ExternalIds;
use super::ext::OptionExt;
use super::import::import_movie;

pub fn routes(router: &mut Router) {
    router.route("/movies").get(get_movies).post(import_movie);
    router.route("/movies/:id").get(get_movie);
    router.route("/movies/recent").get(get_recent_movies);
}

#[derive(Serialize)]
struct Movie {
    id: i64,
    title: String,
    release_date: Option<i64>,
    overview: Option<String>,
    poster: Option<String>,
    backdrop: Option<String>,
    duration: f64,
    is_watched: bool,
    external_ids: ExternalIds,
}

impl<'r> FromRow<'r, SqliteRow> for Movie {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let poster: Option<String> = row.try_get(4)?;
        let backdrop: Option<String> = row.try_get(5)?;

        Ok(Movie {
            id: row.try_get(0)?,
            title: row.try_get(1)?,
            release_date: row.try_get(2)?,
            overview: row.try_get(3)?,
            poster: poster.as_deref().map(utils::get_image_url),
            backdrop: backdrop.as_deref().map(utils::get_image_url),
            duration: row.try_get(6)?,
            is_watched: row.try_get(8)?,
            external_ids: ExternalIds {
                tmdb: row.try_get(7)?,
            },
        })
    }
}

#[endpoint]
async fn get_movies(req: &mut Request) -> eyre::Result<()> {
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT
            movie.item_id, title, release_date, overview,
            poster, backdrop, video.duration, tmdb_id,
            COALESCE(u.is_watched, 0)
        FROM movies AS movie
        JOIN video_files AS video ON video.item_id = movie.item_id
        LEFT JOIN user_item_data AS u ON u.item_id = movie.item_id
        ORDER BY title
    ";

    let movies: Vec<Movie> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

    req.ok().json(&movies)?;

    Ok(())
}

#[endpoint]
async fn get_movie(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT
            movie.item_id, title, release_date, overview,
            poster, backdrop, video.duration, tmdb_id,
            COALESCE(u.is_watched, 0)
        FROM movies AS movie
        JOIN video_files AS video ON video.item_id = movie.item_id
        LEFT JOIN user_item_data AS u ON u.item_id = movie.item_id
        WHERE movie.item_id = ?
        ORDER BY title
    ";

    let movie: Movie = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .or_not_found("movie not found")?;

    req.ok().json(&movie)?;

    Ok(())
}

#[endpoint]
async fn get_recent_movies(req: &mut Request) -> eyre::Result<()> {
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT
            movie.item_id, title, release_date, overview,
            poster, backdrop, video.duration, tmdb_id,
            COALESCE(u.is_watched, 0)
        FROM movies AS movie
        JOIN media_items AS item ON item.id = movie.item_id
        JOIN video_files AS video ON video.item_id = movie.item_id
        LEFT JOIN user_item_data AS u ON u.item_id = movie.item_id
        WHERE COALESCE(u.is_watched, 0) = 0
        ORDER BY added_at DESC, title
        LIMIT 30
    ";

    let movies: Vec<Movie> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

    req.ok().json(&movies)?;

    Ok(())
}
