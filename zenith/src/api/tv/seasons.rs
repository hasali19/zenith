use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};
use zenith_http::{Request, Response};

use crate::api::{ApiError, ApiResult};
use crate::{utils, AppState};

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

pub(super) async fn get_seasons(state: AppState, req: Request) -> ApiResult {
    let show_id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

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
        .await?;

    Ok(Response::new().json(&seasons)?)
}

pub(super) async fn get_season(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

    let sql = "
        SELECT season.item_id, season.show_id, season_number, season.name, season.overview, season.poster, show.backdrop
        FROM tv_seasons AS season
        JOIN tv_shows AS show ON show.item_id = season.show_id
        WHERE season.item_id = ?
    ";

    let season: Season = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .ok_or_else(ApiError::not_found)?;

    Ok(Response::new().json(&season)?)
}
