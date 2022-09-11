use serde::Serialize;
use speq::Reflect;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

use crate::sql::Join;
use crate::{sql, utils};

use super::collections::CollectionUserData;
use super::items::ExternalIds;
use super::media::MediaImageType;

#[derive(Serialize, Reflect)]
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

impl Season {
    pub fn image(&self, img_type: MediaImageType) -> Option<&str> {
        match img_type {
            MediaImageType::Poster => self.poster.as_deref(),
            MediaImageType::Backdrop => self.backdrop.as_deref(),
            MediaImageType::Thumbnail => self.backdrop.as_deref(),
        }
    }
}

const SEASON_COLUMNS: &[&str] = &[
    "se.id",
    "se.show_id",
    "se.season_no",
    "se.name",
    "sh.name AS show_name",
    "se.overview",
    "COALESCE(se.poster, sh.poster) AS poster",
    "sh.backdrop",
    "se.tmdb_id",
    "(
        SELECT COUNT(*)
        FROM episodes AS episode
        LEFT JOIN user_item_data AS u ON u.item_id = episode.id
        WHERE episode.season_id = se.id AND COALESCE(u.is_watched, 0) = 0
    ) AS unwatched",
];

impl<'r> FromRow<'r, SqliteRow> for Season {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let poster: Option<&str> = row.try_get("poster")?;
        let backdrop: Option<&str> = row.try_get("backdrop")?;

        Ok(Season {
            id: row.try_get("id")?,
            show_id: row.try_get("show_id")?,
            season_number: row.try_get("season_no")?,
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
    let sql = sql::select("seasons AS se")
        .columns(SEASON_COLUMNS)
        .joins(&[Join::inner("shows AS sh").on("sh.id = se.show_id")])
        .condition("se.id = ?1")
        .to_sql();

    Ok(sqlx::query_as(&sql).bind(id).fetch_optional(conn).await?)
}

pub async fn get_for_show(conn: &mut SqliteConnection, show_id: i64) -> eyre::Result<Vec<Season>> {
    let sql = sql::select("seasons AS se")
        .columns(SEASON_COLUMNS)
        .joins(&[Join::inner("shows AS sh").on("sh.id = se.show_id")])
        .condition("se.show_id = ?1")
        .order_by(&["season_no"])
        .to_sql();

    Ok(sqlx::query_as(&sql).bind(show_id).fetch_all(conn).await?)
}
