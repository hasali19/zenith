use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

use crate::sql::Join;
use crate::{sql, utils};

use super::collections::CollectionUserData;
use super::items::ExternalIds;
use super::media::MediaImage;

#[derive(Serialize)]
pub struct Season {
    pub id: i64,
    pub show_id: i64,
    pub season_number: u32,
    pub name: Option<String>,
    pub show_name: String,
    pub overview: Option<String>,
    pub poster: Option<String>,
    pub backdrop: Option<String>,
    pub external_ids: ExternalIds,
    pub user_data: CollectionUserData,
}

const SEASON_COLUMNS: &[&str] = &[
    "se.item_id AS id",
    "show_id",
    "season_number",
    "se.name",
    "sh.name AS show_name",
    "se.overview",
    "se.poster",
    "sh.backdrop",
    "se.tmdb_id",
    "(
        SELECT COUNT(*)
        FROM tv_episodes AS episode
        LEFT JOIN user_item_data AS u ON u.item_id = episode.item_id
        WHERE episode.season_id = se.item_id AND COALESCE(u.is_watched, 0) = 0
    ) AS unwatched",
];

impl<'r> FromRow<'r, SqliteRow> for Season {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let poster: Option<&str> = row.try_get("poster")?;
        let backdrop: Option<&str> = row.try_get("backdrop")?;

        Ok(Season {
            id: row.try_get("id")?,
            show_id: row.try_get("show_id")?,
            season_number: row.try_get("season_number")?,
            name: row.try_get("name")?,
            show_name: row.try_get("show_name")?,
            overview: row.try_get("overview")?,
            poster: poster.map(utils::get_image_url),
            backdrop: backdrop.map(utils::get_image_url),
            external_ids: ExternalIds {
                tmdb: row.try_get("tmdb_id")?,
            },
            user_data: CollectionUserData {
                unwatched: row.try_get("unwatched")?,
            },
        })
    }
}

pub async fn get(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Season>> {
    let sql = sql::select("tv_seasons AS se")
        .columns(SEASON_COLUMNS)
        .joins(&[Join::inner("tv_shows AS sh").on("sh.item_id = se.show_id")])
        .condition("se.item_id = ?1")
        .to_sql();

    Ok(sqlx::query_as(&sql).bind(id).fetch_optional(conn).await?)
}

pub async fn get_for_show(conn: &mut SqliteConnection, show_id: i64) -> eyre::Result<Vec<Season>> {
    let sql = sql::select("tv_seasons AS se")
        .columns(SEASON_COLUMNS)
        .joins(&[Join::inner("tv_shows AS sh").on("sh.item_id = se.show_id")])
        .condition("se.show_id = ?1")
        .order_by(&["season_number"])
        .to_sql();

    Ok(sqlx::query_as(&sql).bind(show_id).fetch_all(conn).await?)
}

pub struct UpdateMetadata<'a> {
    pub name: Option<&'a str>,
    pub overview: Option<&'a str>,
    pub poster: Option<MediaImage<'a>>,
    pub tmdb_id: Option<i32>,
}

pub async fn update_metadata(
    conn: &mut SqliteConnection,
    id: i64,
    data: UpdateMetadata<'_>,
) -> eyre::Result<()> {
    let sql = "
        UPDATE tv_seasons
        SET name = ?,
            overview = ?,
            poster = ?,
            tmdb_id = ?
        WHERE item_id = ?
    ";

    sqlx::query(sql)
        .bind(data.name)
        .bind(data.overview)
        .bind(data.poster.map(|p| p.to_string()))
        .bind(data.tmdb_id)
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}
