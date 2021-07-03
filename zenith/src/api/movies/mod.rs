use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::web::{get, post, ServiceConfig};
use actix_web::{web, Responder};
use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};

use crate::db::Db;
use crate::utils;

use super::import::import_movie;

pub fn configure(config: &mut ServiceConfig) {
    let movies = web::resource("/movies")
        .route(get().to(get_movies))
        .route(post().to(import_movie));

    config
        .service(movies)
        .route("/movies/recent", web::get().to(get_recent_movies))
        .route("/movies/{id}", web::get().to(get_movie));
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

async fn get_movies(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let sql = "
        SELECT
            movie.item_id, title, release_date, overview,
            poster, backdrop, video.duration
        FROM movies AS movie
        JOIN video_files AS video ON video.item_id = movie.item_id
        ORDER BY title
    ";

    let movies: Vec<Movie> = sqlx::query_as(sql)
        .fetch_all(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&movies))
}

async fn get_movie(req: HttpRequest, path: web::Path<(i64,)>) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();
    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

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
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(""))?;

    Ok(HttpResponse::Ok().json(&movie))
}

async fn get_recent_movies(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let sql = "
        SELECT
            movie.item_id, title, release_date, overview,
            poster, backdrop, video.duration
        FROM movies AS movie
        JOIN media_items AS item ON item.id = movie.item_id
        JOIN video_files AS video ON video.item_id = movie.item_id
        LEFT JOIN user_item_data AS u ON u.item_id = movie.item_id
        WHERE COALESCE(u.is_watched, 0) = 0
        ORDER BY added_at DESC, title
        LIMIT 30
    ";

    let movies: Vec<Movie> = sqlx::query_as(sql)
        .fetch_all(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&movies))
}
