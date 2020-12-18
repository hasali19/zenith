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
pub struct Movie {
    id: i64,
    title: String,
}

async fn get_movies(db: Db) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;

    let movies: Vec<(i64, String)> = sqlx::query_as("SELECT id, title FROM movies ORDER BY title")
        .fetch_all(&mut conn)
        .await?;

    let res: Vec<Movie> = movies
        .into_iter()
        .map(|(id, title)| Movie { id, title })
        .collect();

    Ok(HttpResponse::Ok().json(res))
}

async fn get_movie(path: web::Path<(i64,)>, db: Db) -> ApiResult<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = db.acquire().await?;

    let movie: Option<(i64, String)> = sqlx::query_as("SELECT id, title FROM movies WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await?;

    let res = match movie {
        Some((id, title)) => Movie { id, title },
        None => return Ok(HttpResponse::NotFound().finish()),
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
