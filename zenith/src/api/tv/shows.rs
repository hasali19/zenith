use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};

use crate::db::Db;
use crate::utils;

#[derive(Serialize)]
struct Show {
    id: i64,
    name: String,
    start_date: Option<i64>,
    end_date: Option<i64>,
    overview: Option<String>,
    poster: Option<String>,
    backdrop: Option<String>,
    unwatched_episodes: u32,
}

impl<'r> FromRow<'r, SqliteRow> for Show {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let poster: Option<String> = row.try_get(5)?;
        let backdrop: Option<String> = row.try_get(6)?;

        Ok(Show {
            id: row.try_get(0)?,
            name: row.try_get(1)?,
            start_date: row.try_get(2)?,
            end_date: row.try_get(3)?,
            overview: row.try_get(4)?,
            poster: poster.as_deref().map(utils::get_image_url),
            backdrop: backdrop.as_deref().map(utils::get_image_url),
            unwatched_episodes: row.try_get(7)?,
        })
    }
}

/// GET /api/tv/shows
pub async fn get_shows(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let sql = "
        SELECT
            show.item_id, name, start_date, end_date,
            overview, poster, backdrop, (
                SELECT COUNT(*)
                FROM tv_episodes AS episode
                JOIN tv_seasons AS season ON season.item_id = episode.season_id
                LEFT JOIN user_item_data AS u ON u.item_id = episode.item_id
                WHERE season.show_id = show.item_id AND COALESCE(u.is_watched, 0) = 0
            )
        FROM tv_shows AS show
        ORDER BY name
    ";

    let shows: Vec<Show> = sqlx::query_as(sql)
        .fetch_all(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&shows))
}

/// GET /api/tv/shows/recent
pub async fn get_recently_updated_shows(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    // Get shows sorted by the added_at of their most recently added episode
    // (i.e. shows that have had an episode added recently will appear higher up)
    let sql = "
        SELECT
            show.item_id, show.name, show.start_date, show.end_date,
            show.overview, show.poster, show.backdrop, (
                SELECT COUNT(*)
                FROM tv_episodes AS episode
                JOIN tv_seasons AS season ON season.item_id = episode.season_id
                LEFT JOIN user_item_data AS u ON u.item_id = episode.item_id
                WHERE season.show_id = show.item_id AND COALESCE(u.is_watched, 0) = 0
            ) AS unwatched_episodes,
            MAX(item.added_at) AS latest_episode_added_at
        FROM tv_shows AS show
        JOIN tv_seasons AS season ON season.show_id = show.item_id
        JOIN tv_episodes AS episode ON episode.season_id = season.item_id
        JOIN media_items AS item ON item.id = episode.item_id
        WHERE unwatched_episodes > 0
        GROUP BY show.item_id
        ORDER BY latest_episode_added_at DESC, show.name
        LIMIT 30
    ";

    let shows: Vec<Show> = sqlx::query_as(sql)
        .fetch_all(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&shows))
}

/// GET /api/tv/shows/{id}
pub async fn get_show(
    req: HttpRequest,
    path: web::Path<(i64,)>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();

    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let sql = "
        SELECT
            show.item_id, name, start_date, end_date,
            overview, poster, backdrop, (
                SELECT COUNT(*)
                FROM tv_episodes AS episode
                JOIN tv_seasons AS season ON season.item_id = episode.season_id
                LEFT JOIN user_item_data AS u ON u.item_id = episode.item_id
                WHERE season.show_id = show.item_id AND COALESCE(u.is_watched, 0) = 0
            )
        FROM tv_shows AS show
        WHERE show.item_id = ?
    ";

    let show: Show = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(""))?;

    Ok(HttpResponse::Ok().json(&show))
}
