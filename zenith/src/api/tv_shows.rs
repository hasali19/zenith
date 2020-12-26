use actix_files::NamedFile;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Responder};

use crate::db::media::{MediaFileType, MediaItemType};
use crate::db::Db;
use crate::utils;

use super::{ApiError, ApiResult};

pub fn service(path: &str) -> impl HttpServiceFactory {
    web::scope(path)
        .route("", web::get().to(get_tv_shows))
        .route("/{id}", web::get().to(get_tv_show))
        .route(
            "/{movie_id}/episodes/{episode_id}/stream",
            web::get().to(get_stream),
        )
}

#[derive(serde::Serialize)]
pub struct TvShow {
    id: i64,
    name: String,
    poster_url: Option<String>,
}

async fn get_tv_shows(db: Db) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT id, name, primary_image
        FROM media_items WHERE item_type = ?
        ORDER BY name
    ";

    let shows: Vec<(i64, String, Option<String>)> = sqlx::query_as(sql)
        .bind(MediaItemType::TvShow)
        .fetch_all(&mut conn)
        .await?;

    let res: Vec<TvShow> = shows
        .into_iter()
        .map(|(id, name, primary)| TvShow {
            id,
            name,
            poster_url: primary.as_deref().map(utils::get_image_url),
        })
        .collect();

    Ok(HttpResponse::Ok().json(res))
}

#[derive(serde::Serialize)]
struct TvShowFull {
    id: i64,
    name: String,
    overview: Option<String>,
    poster_url: Option<String>,
    backdrop_url: Option<String>,
    episodes: Vec<TvEpisode>,
}

#[derive(serde::Serialize)]
pub struct TvEpisode {
    id: i64,
    season: u32,
    episode: u32,
    overview: Option<String>,
    thumbnail_url: Option<String>,
}

async fn get_tv_show(path: web::Path<(i64,)>, db: Db) -> ApiResult<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = db.acquire().await?;

    type Row = (i64, String, Option<String>, Option<String>, Option<String>);

    let sql = "
        SELECT id, name, overview, primary_image, backdrop_image
        FROM media_items WHERE id = ? AND item_type = ?
    ";

    let movie: Option<Row> = sqlx::query_as(sql)
        .bind(id)
        .bind(MediaItemType::TvShow)
        .fetch_optional(&mut conn)
        .await?;

    let res = match movie {
        None => return Ok(HttpResponse::NotFound().finish()),
        Some((id, name, overview, poster, backdrop)) => {
            let sql = "
                SELECT episode.id, season.index_number, episode.index_number, episode.overview, episode.primary_image
                FROM media_items AS episode
                JOIN media_items AS season ON season.id = episode.parent_id
                WHERE season.parent_id = ?
                ORDER BY season.index_number, episode.index_number
            ";

            type Row = (i64, i64, i64, Option<String>, Option<String>);

            let episodes: Vec<Row> = sqlx::query_as(sql).bind(id).fetch_all(&mut conn).await?;
            let episodes = episodes
                .into_iter()
                .map(|(id, season, episode, overview, primary)| TvEpisode {
                    id,
                    season: season as u32,
                    episode: episode as u32,
                    overview,
                    thumbnail_url: primary.as_deref().map(utils::get_image_url),
                })
                .collect();

            TvShowFull {
                id,
                name,
                overview,
                poster_url: poster.as_deref().map(utils::get_image_url),
                backdrop_url: backdrop.as_deref().map(utils::get_image_url),
                episodes,
            }
        }
    };

    Ok(HttpResponse::Ok().json(res))
}

async fn get_stream(path: web::Path<(i64, i64)>, db: Db) -> ApiResult<impl Responder> {
    let (show_id, episode_id) = path.into_inner();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT file.path FROM media_files AS file
        JOIN media_items AS episode ON episode.id = file.item_id
        JOIN media_items AS season ON season.id = episode.parent_id
        JOIN media_items AS show ON show.id = season.parent_id
        WHERE episode.id = ? AND show.id = ? AND file.file_type = ?
    ";

    let path: Option<(String,)> = sqlx::query_as(sql)
        .bind(episode_id)
        .bind(show_id)
        .bind(MediaFileType::Video)
        .fetch_optional(&mut conn)
        .await?;

    let path = match path {
        Some((path,)) => path,
        None => return Err(ApiError::NotFound),
    };

    Ok(NamedFile::open(path))
}
