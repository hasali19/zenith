use serde::Serialize;
use speq::Reflect;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

use crate::sql::Join;
use crate::util::VecExt;
use crate::{sql, utils};

use super::collections::CollectionUserData;
use super::items::ExternalIds;
use super::media::{MediaImage, MediaImageType, MediaItemType};

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

impl Show {
    pub fn image(&self, img_type: MediaImageType) -> Option<&str> {
        match img_type {
            MediaImageType::Poster => self.poster.as_deref(),
            MediaImageType::Backdrop => self.backdrop.as_deref(),
            MediaImageType::Thumbnail => self.backdrop.as_deref(),
        }
    }
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

pub async fn get_path(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<String>> {
    let sql = "
        SELECT path from indexed_paths
        JOIN shows ON item_id = shows.id
        WHERE shows.id = ?";

    let path = sqlx::query_scalar(sql)
        .bind(id)
        .fetch_optional(conn)
        .await?;

    Ok(path)
}

pub async fn get_all(conn: &mut SqliteConnection) -> eyre::Result<Vec<Show>> {
    let sql = sql::select("shows AS sh")
        .columns(SHOW_COLUMNS)
        .order_by(&["name"])
        .to_sql();

    Ok(sqlx::query_as(&sql).fetch_all(conn).await?)
}

pub async fn get_recently_updated(conn: &mut SqliteConnection) -> eyre::Result<Vec<Show>> {
    // Get shows sorted by the added_at of their most recently added episode
    // (i.e. shows that have had an episode added recently will appear higher up)
    let sql = sql::select("shows AS sh")
        .columns(
            SHOW_COLUMNS
                .to_vec()
                .with_push("MAX(e.added_at) AS latest_episode_added_at"),
        )
        .joins(&[Join::inner("episodes AS e").on("e.show_id = sh.id")])
        .condition("unwatched > 0")
        .group_by("sh.id")
        .order_by(&["latest_episode_added_at DESC", "sh.name"])
        .limit(30)
        .to_sql();

    Ok(sqlx::query_as(&sql).fetch_all(conn).await?)
}

pub struct UpdateMetadata<'a> {
    pub name: &'a str,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub overview: Option<&'a str>,
    pub poster: Option<MediaImage<'a>>,
    pub backdrop: Option<MediaImage<'a>>,
    pub tmdb_id: Option<i32>,
}

pub async fn update_metadata(
    conn: &mut SqliteConnection,
    id: i64,
    data: UpdateMetadata<'_>,
) -> eyre::Result<()> {
    let sql = "
        UPDATE media_items
        SET name = ?,
            start_date = ?,
            end_date = ?,
            overview = ?,
            poster = ?,
            backdrop = ?,
            tmdb_id = ?
        WHERE item_type = ? AND id = ?
    ";

    sqlx::query(sql)
        .bind(data.name)
        .bind(data.start_date)
        .bind(data.end_date)
        .bind(data.overview)
        .bind(data.poster.map(|p| p.to_string()))
        .bind(data.backdrop.map(|b| b.to_string()))
        .bind(data.tmdb_id)
        .bind(MediaItemType::Show)
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}
