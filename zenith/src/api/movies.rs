use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Responder};

use crate::db::Db;
use crate::utils;

use super::ApiResult;

pub fn service(path: &str) -> impl HttpServiceFactory {
    web::scope(path)
        .route("", web::get().to(get_movies))
        .route("/{id}", web::get().to(get_movie))
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

    let sql = "
        SELECT id, name, CAST(strftime('%Y', datetime(release_date, 'unixepoch')) as INTEGER), primary_image
        FROM media_items WHERE item_type = 1
        ORDER BY name
    ";

    let movies: Vec<(i64, String, Option<i32>, Option<String>)> =
        sqlx::query_as(sql).fetch_all(&mut conn).await?;

    let res: Vec<MovieListItem> = movies
        .into_iter()
        .map(|(id, title, year, primary)| MovieListItem {
            id,
            title,
            year,
            poster_url: primary.as_deref().map(utils::get_image_url),
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
    stream: String,
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
        i64,
    );

    let sql = "
        SELECT movie.id, name, CAST(strftime('%Y', datetime(release_date, 'unixepoch')) as INTEGER),
               overview, primary_image, backdrop_image, file.id
        FROM media_items AS movie
        JOIN media_files AS file ON movie.id = file.item_id
        WHERE movie.id = ? AND item_type = 1 AND file_type = 1
    ";

    let movie: Option<Row> = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?;

    let res = match movie {
        None => return Ok(HttpResponse::NotFound().finish()),
        Some((id, title, year, overview, poster, backdrop, file_id)) => MovieDetails {
            id,
            title,
            year,
            overview,
            poster_url: poster.as_deref().map(utils::get_image_url),
            backdrop_url: backdrop.as_deref().map(utils::get_image_url),
            stream: format!("/api/stream/{}", file_id),
        },
    };

    Ok(HttpResponse::Ok().json(res))
}
