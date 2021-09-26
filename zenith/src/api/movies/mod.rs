use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request};

use crate::db::Db;

use super::common::{self, Movie};
use super::ext::OptionExt;
use super::import::import_movie;

pub fn routes(router: &mut Router) {
    router.route("/movies").get(get_movies).post(import_movie);
    router.route("/movies/:id").get(get_movie);
    router.route("/movies/recent").get(get_recent_movies);
}

#[endpoint]
async fn get_movies(req: &mut Request) -> eyre::Result<()> {
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT
            movie.item_id AS id,
            title,
            release_date,
            overview,
            poster,
            backdrop,
            tmdb_id,
            path,
            duration,
            COALESCE(is_watched, 0) AS is_watched,
            position
        FROM movies AS movie
        JOIN video_files AS video ON movie.item_id = video.item_id
        LEFT JOIN user_item_data AS user_data ON movie.item_id = user_data.item_id
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

    let movie = common::get_movie_item(&mut conn, id)
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
            movie.item_id AS id,
            title,
            release_date,
            overview,
            poster,
            backdrop,
            tmdb_id,
            path,
            duration,
            COALESCE(is_watched, 0) AS is_watched,
            position
        FROM movies AS movie
        JOIN media_items AS item ON item.id = movie.item_id
        JOIN video_files AS video ON video.item_id = movie.item_id
        LEFT JOIN user_item_data AS user_data ON user_data.item_id = movie.item_id
        WHERE COALESCE(user_data.is_watched, 0) = 0
        ORDER BY added_at DESC, title
        LIMIT 30
    ";

    let movies: Vec<Movie> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

    req.ok().json(&movies)?;

    Ok(())
}
