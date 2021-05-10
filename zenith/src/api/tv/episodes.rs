use std::path::{Path, PathBuf};

use actix_http::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};

use crate::db::Db;
use crate::utils;

#[derive(Serialize)]
struct Episode {
    id: i64,
    show_id: i64,
    season_id: i64,
    episode_number: i32,
    name: Option<String>,
    air_date: Option<i64>,
    overview: Option<String>,
    thumbnail: Option<String>,
    duration: f64,
    is_watched: bool,
}

impl<'r> FromRow<'r, SqliteRow> for Episode {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let thumbnail: Option<String> = row.try_get(7)?;

        Ok(Episode {
            id: row.try_get(0)?,
            show_id: row.try_get(1)?,
            season_id: row.try_get(2)?,
            episode_number: row.try_get(3)?,
            name: row.try_get(4)?,
            air_date: row.try_get(5)?,
            overview: row.try_get(6)?,
            thumbnail: thumbnail.as_deref().map(utils::get_image_url),
            duration: row.try_get(8)?,
            is_watched: row.try_get(9)?,
        })
    }
}

pub(super) async fn get_episodes(
    req: HttpRequest,
    path: web::Path<(i64,)>,
) -> actix_web::Result<impl Responder> {
    let (season_id,) = path.into_inner();

    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let sql = "
        SELECT
            episode.item_id, show.item_id, season.item_id, episode_number,
            episode.name, episode.air_date, episode.overview, episode.thumbnail,
            video.duration, COALESCE(user.is_watched, 0)
        FROM tv_episodes AS episode
        JOIN tv_seasons AS season ON season.item_id = episode.season_id
        JOIN tv_shows AS show ON show.item_id = season.show_id
        JOIN video_files AS video ON video.item_id = episode.item_id
        LEFT JOIN user_item_data AS user ON user.item_id = episode.item_id
        WHERE episode.season_id = ?
        ORDER BY episode_number
    ";

    let episodes: Vec<Episode> = sqlx::query_as(sql)
        .bind(season_id)
        .fetch_all(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&episodes))
}

#[derive(Deserialize)]
pub(super) struct ImportRequest {
    source: ImportSource,
    season_number: u32,
    episode_number: u32,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub(super) enum ImportSource {
    Local { path: String },
}

pub(super) async fn import_episode(
    req: HttpRequest,
    path: web::Path<(i64,)>,
    data: web::Json<ImportRequest>,
) -> actix_web::Result<impl Responder> {
    let (show_id,) = path.into_inner();
    let data = data.into_inner();
    let db: &Db = req.app_data().unwrap();

    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;
    let show_path: String = sqlx::query_scalar("SELECT path from tv_shows WHERE item_id = ?")
        .bind(show_id)
        .fetch_optional(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("show not found"))?;

    let src_path = match data.source {
        ImportSource::Local { path } => path,
    };

    let src_path = PathBuf::from(src_path);
    let src_ext = src_path
        .extension()
        .ok_or_else(|| ErrorBadRequest("source file has no extension"))?;

    let dst_name = format!("S{:02}E{:02}", data.season_number, data.episode_number);
    let dst_path = Path::new(&show_path).join(dst_name).with_extension(src_ext);

    // Just move the file into the library and let the fs watcher
    // take care of the rest
    tracing::info!("moving {:?} to {:?}", src_path, dst_path);
    std::fs::rename(src_path, dst_path).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok())
}

pub(super) async fn get_episode(
    req: HttpRequest,
    path: web::Path<(i64,)>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();

    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let sql = "
        SELECT
            episode.item_id, show.item_id, season.item_id, episode_number,
            episode.name, episode.air_date, episode.overview, episode.thumbnail,
            video.duration, COALESCE(user.is_watched, 0)
        FROM tv_episodes AS episode
        JOIN tv_seasons AS season ON season.item_id = episode.season_id
        JOIN tv_shows AS show ON show.item_id = season.show_id
        JOIN video_files AS video ON video.item_id = episode.item_id
        LEFT JOIN user_item_data AS user ON user.item_id = episode.item_id
        WHERE episode.item_id = ?
    ";

    let episode: Episode = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(""))?;

    Ok(HttpResponse::Ok().json(&episode))
}
