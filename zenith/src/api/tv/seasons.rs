use atium::respond::RespondRequestExt;
use atium::router::RouterRequestExt;
use atium::{endpoint, Request};
use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};

use crate::api::common::ExternalIds;
use crate::api::ext::OptionExt;
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
    external_ids: ExternalIds,
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
            external_ids: ExternalIds {
                tmdb: row.try_get(7)?,
            },
        })
    }
}

#[endpoint]
pub(super) async fn get_seasons(req: &mut Request) -> eyre::Result<()> {
    let show_id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT
            season.item_id,
            season.show_id,
            season_number,
            season.name,
            season.overview,
            season.poster,
            show.backdrop,
            season.tmdb_id
        FROM tv_seasons AS season
        JOIN tv_shows AS show ON show.item_id = season.show_id
        WHERE season.show_id = ?
        ORDER BY season_number
    ";

    let seasons: Vec<Season> = sqlx::query_as(sql)
        .bind(show_id)
        .fetch_all(&mut conn)
        .await?;

    req.ok().json(&seasons)?;

    Ok(())
}

#[endpoint]
pub(super) async fn get_season(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT
            season.item_id,
            season.show_id,
            season_number,
            season.name,
            season.overview,
            season.poster,
            show.backdrop,
            season.tmdb_id
        FROM tv_seasons AS season
        JOIN tv_shows AS show ON show.item_id = season.show_id
        WHERE season.item_id = ?
    ";

    let season: Season = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .or_not_found("season not found")?;

    req.ok().json(&season)?;

    Ok(())
}
