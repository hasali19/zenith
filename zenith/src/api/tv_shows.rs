use actix_files::NamedFile;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Responder};

use crate::db::Db;

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
    poster_url: String,
}

async fn get_tv_shows(db: Db) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;

    let shows: Vec<(i64, String, String)> =
        sqlx::query_as("SELECT id, name, poster_url FROM tv_shows ORDER BY name")
            .fetch_all(&mut conn)
            .await?;

    let res: Vec<TvShow> = shows
        .into_iter()
        .map(|(id, name, poster_url)| TvShow {
            id,
            name,
            poster_url,
        })
        .collect();

    Ok(HttpResponse::Ok().json(res))
}

#[derive(serde::Serialize)]
struct TvShowFull {
    id: i64,
    title: String,
    episodes: Vec<TvEpisode>,
}

#[derive(serde::Serialize)]
pub struct TvEpisode {
    id: i64,
    season: u32,
    episode: u32,
}

async fn get_tv_show(path: web::Path<(i64,)>, db: Db) -> ApiResult<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = db.acquire().await?;

    let movie: Option<(i64, String)> = sqlx::query_as("SELECT id, name FROM tv_shows WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await?;

    let res = match movie {
        None => return Ok(HttpResponse::NotFound().finish()),
        Some((id, title)) => {
            let sql = "SELECT id, season, episode FROM tv_episodes WHERE show_id = ? ORDER BY season, episode";
            let episodes: Vec<(i64, i64, i64)> =
                sqlx::query_as(sql).bind(id).fetch_all(&mut conn).await?;

            let episodes = episodes
                .into_iter()
                .map(|(id, season, episode)| TvEpisode {
                    id,
                    season: season as u32,
                    episode: episode as u32,
                })
                .collect();

            TvShowFull {
                id,
                title,
                episodes,
            }
        }
    };

    Ok(HttpResponse::Ok().json(res))
}

async fn get_stream(path: web::Path<(i64, i64)>, db: Db) -> ApiResult<impl Responder> {
    let (show_id, episode_id) = path.into_inner();
    let mut conn = db.acquire().await?;

    let path: Option<(String,)> =
        sqlx::query_as("SELECT video_path FROM tv_episodes WHERE id = ? AND show_id = ?")
            .bind(episode_id)
            .bind(show_id)
            .fetch_optional(&mut conn)
            .await?;

    let path = match path {
        Some((path,)) => path,
        None => return Err(ApiError::NotFound),
    };

    Ok(NamedFile::open(path))
}
