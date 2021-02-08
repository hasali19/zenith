use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};
use zenith_http::{App, Request, Response};

use crate::{utils, AppState};

use super::{ApiError, ApiResult};

pub fn configure(app: &mut App<AppState>) {
    app.get("/api/movies", get_movies);
    app.get("/api/movies/:id", get_movie);
    app.get("/api/movies/recent", get_recent_movies);
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
        })
    }
}

async fn get_movies(state: AppState, _: Request) -> ApiResult {
    let mut conn = state.db.acquire().await?;

    let sql = "
        SELECT
            movie.item_id, title, release_date, overview,
            poster, backdrop, video.duration
        FROM movies AS movie
        JOIN video_files AS video ON video.item_id = movie.item_id
        ORDER BY title
    ";

    let movies: Vec<Movie> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

    Ok(Response::new().json(&movies)?)
}

async fn get_movie(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

    let sql = "
        SELECT
            movie.item_id, title, release_date, overview,
            poster, backdrop, video.duration
        FROM movies AS movie
        JOIN video_files AS video ON video.item_id = movie.item_id
        WHERE movie.item_id = ?
        ORDER BY title
    ";

    let movie: Movie = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .ok_or_else(ApiError::not_found)?;

    Ok(Response::new().json(&movie)?)
}

async fn get_recent_movies(state: AppState, _: Request) -> ApiResult {
    let mut conn = state.db.acquire().await?;

    let sql = "
        SELECT
            movie.item_id, title, release_date, overview,
            poster, backdrop, video.duration
        FROM movies AS movie
        JOIN media_items AS item ON item.id = movie.item_id
        JOIN video_files AS video ON video.item_id = movie.item_id
        ORDER BY added_at DESC, title
        LIMIT 10
    ";

    let movies: Vec<Movie> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

    Ok(Response::new().json(&movies)?)
}
