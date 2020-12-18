use actix_files::NamedFile;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Responder};

use crate::db::Db;

use super::{ApiError, ApiResult};

pub fn service(path: &str) -> impl HttpServiceFactory {
    web::scope(path)
        .route("", web::get().to(get_movies))
        .route("/{id}", web::get().to(get_movie))
        .route("/{id}/stream", web::get().to(get_stream))
}

#[derive(serde::Serialize)]
pub struct MovieListItem {
    id: i64,
    title: String,
    year: Option<i32>,
    poster_url: Option<String>,
}

async fn get_movies(db: Db) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;

    let movies: Vec<(i64, String, Option<i32>, Option<String>)> =
        sqlx::query_as("SELECT id, title, year, poster_url FROM movies ORDER BY title")
            .fetch_all(&mut conn)
            .await?;

    let res: Vec<MovieListItem> = movies
        .into_iter()
        .map(|(id, title, year, poster_url)| MovieListItem {
            id,
            title,
            year,
            poster_url,
        })
        .collect();

    Ok(HttpResponse::Ok().json(res))
}

#[derive(serde::Serialize)]
pub struct MovieDetails {
    id: i64,
    title: String,
    year: Option<i32>,
    overview: Option<String>,
    poster_url: Option<String>,
    backdrop_url: Option<String>,
}

async fn get_movie(path: web::Path<(i64,)>, db: Db) -> ApiResult<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = db.acquire().await?;

    type Row = (
        i64,
        String,
        Option<i32>,
        Option<String>,
        Option<String>,
        Option<String>,
    );

    let sql = "
        SELECT id, title, year, overview, poster_url, backdrop_url
        FROM movies WHERE id = ?
    ";

    let movie: Option<Row> = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?;

    let res = match movie {
        None => return Ok(HttpResponse::NotFound().finish()),
        Some((id, title, year, overview, poster_url, backdrop_url)) => MovieDetails {
            id,
            title,
            year,
            overview,
            poster_url,
            backdrop_url,
        },
    };

    Ok(HttpResponse::Ok().json(res))
}

async fn get_stream(path: web::Path<(i64,)>, db: Db) -> ApiResult<impl Responder> {
    let (movie_id,) = path.into_inner();
    let mut conn = db.acquire().await?;

    let path: Option<(String,)> = sqlx::query_as("SELECT video_path FROM movies WHERE id = ?")
        .bind(movie_id)
        .fetch_optional(&mut conn)
        .await?;

    let path = match path {
        Some((path,)) => path,
        None => return Err(ApiError::NotFound),
    };

    Ok(NamedFile::open(path))
}
