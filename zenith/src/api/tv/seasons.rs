use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};

use crate::db::Db;
use crate::utils;

#[derive(Serialize)]
struct Season {
    id: i64,
    show_id: i64,
    season_number: i32,
    name: Option<String>,
    overview: Option<String>,
    poster: Option<String>,
    backdrop: Option<String>,
}

impl<'r> FromRow<'r, SqliteRow> for Season {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let poster: Option<String> = row.try_get(5)?;
        let backdrop: Option<String> = row.try_get(6)?;

        Ok(Season {
            id: row.try_get(0)?,
            show_id: row.try_get(1)?,
            season_number: row.try_get(2)?,
            name: row.try_get(3)?,
            overview: row.try_get(4)?,
            poster: poster.as_deref().map(utils::get_image_url),
            backdrop: backdrop.as_deref().map(utils::get_image_url),
        })
    }
}

/// GET /api/tv/shows/{id}/seasons
pub async fn get_seasons(
    req: HttpRequest,
    path: web::Path<(i64,)>,
) -> actix_web::Result<impl Responder> {
    let (show_id,) = path.into_inner();

    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let sql = "
        SELECT season.item_id, season.show_id, season_number, season.name, season.overview, season.poster, show.backdrop
        FROM tv_seasons AS season
        JOIN tv_shows AS show ON show.item_id = season.show_id
        WHERE season.show_id = ?
        ORDER BY season_number
    ";

    let seasons: Vec<Season> = sqlx::query_as(sql)
        .bind(show_id)
        .fetch_all(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&seasons))
}

/// GET /api/tv/seasons/{id}
pub async fn get_season(
    req: HttpRequest,
    path: web::Path<(i64,)>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();

    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let sql = "
        SELECT season.item_id, season.show_id, season_number, season.name, season.overview, season.poster, show.backdrop
        FROM tv_seasons AS season
        JOIN tv_shows AS show ON show.item_id = season.show_id
        WHERE season.item_id = ?
    ";

    let season: Season = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(""))?;

    Ok(HttpResponse::Ok().json(&season))
}
