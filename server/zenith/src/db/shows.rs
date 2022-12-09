use serde::Serialize;
use speq::Reflect;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

use crate::{sql, utils};

use super::collections::CollectionUserData;
use super::items::ExternalIds;

#[derive(Serialize, Reflect)]
pub struct Show {
    pub id: i64,
    pub name: String,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub overview: Option<String>,
    pub poster: Option<String>,
    pub backdrop: Option<String>,
    pub external_ids: ExternalIds,
    pub user_data: CollectionUserData,
}

const SHOW_COLUMNS: &[&str] = &[
    "sh.id",
    "sh.name",
    "sh.start_date",
    "sh.end_date",
    "sh.overview",
    "sh.poster",
    "sh.backdrop",
    "sh.tmdb_id",
    "(
        SELECT COUNT(*)
        FROM episodes AS episode
        LEFT JOIN user_item_data AS u ON u.item_id = episode.id
        WHERE episode.show_id = sh.id AND COALESCE(u.is_watched, 0) = 0
    ) AS unwatched",
];

impl<'r> FromRow<'r, SqliteRow> for Show {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let poster: Option<&str> = row.try_get("poster")?;
        let backdrop: Option<&str> = row.try_get("backdrop")?;

        Ok(Show {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            start_date: row.try_get("start_date")?,
            end_date: row.try_get("end_date")?,
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

pub async fn get(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Show>> {
    let sql = sql::select("shows AS sh")
        .columns(SHOW_COLUMNS)
        .condition("id = ?1")
        .to_sql();

    Ok(sqlx::query_as(&sql).bind(id).fetch_optional(conn).await?)
}
