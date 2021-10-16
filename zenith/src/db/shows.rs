use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

use crate::utils;

use super::collections::CollectionUserData;
use super::items::ExternalIds;

#[derive(Serialize)]
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
    let sql = "
        SELECT
            item_id AS id,
            name,
            start_date,
            end_date,
            overview,
            poster,
            backdrop,
            tmdb_id,
            (
                SELECT COUNT(*)
                FROM tv_episodes AS episode
                JOIN tv_seasons AS season ON season.item_id = episode.season_id
                LEFT JOIN user_item_data AS u ON u.item_id = episode.item_id
                WHERE season.show_id = show.item_id AND COALESCE(u.is_watched, 0) = 0
            ) AS unwatched
        FROM tv_shows AS show
        WHERE item_id = ?
    ";

    Ok(sqlx::query_as(sql).bind(id).fetch_optional(conn).await?)
}

pub async fn get_path(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<String>> {
    let path = sqlx::query_scalar("SELECT path from tv_shows WHERE item_id = ?")
        .bind(id)
        .fetch_optional(conn)
        .await?;

    Ok(path)
}

pub async fn get_all(conn: &mut SqliteConnection) -> eyre::Result<Vec<Show>> {
    let sql = "
        SELECT
            item_id AS id,
            name,
            start_date,
            end_date,
            overview,
            poster,
            backdrop,
            tmdb_id,
            (
                SELECT COUNT(*)
                FROM tv_episodes AS episode
                JOIN tv_seasons AS season ON season.item_id = episode.season_id
                LEFT JOIN user_item_data AS u ON u.item_id = episode.item_id
                WHERE season.show_id = show.item_id AND COALESCE(u.is_watched, 0) = 0
            ) AS unwatched
        FROM tv_shows AS show
        ORDER BY name
    ";

    Ok(sqlx::query_as(sql).fetch_all(conn).await?)
}

pub async fn get_recently_updated(conn: &mut SqliteConnection) -> eyre::Result<Vec<Show>> {
    // Get shows sorted by the added_at of their most recently added episode
    // (i.e. shows that have had an episode added recently will appear higher up)
    let sql = "
        SELECT
            show.item_id AS id,
            show.name,
            start_date,
            end_date,
            show.overview,
            show.poster,
            show.backdrop,
            show.tmdb_id,
            (
                SELECT COUNT(*)
                FROM tv_episodes AS episode
                JOIN tv_seasons AS season ON season.item_id = episode.season_id
                LEFT JOIN user_item_data AS u ON u.item_id = episode.item_id
                WHERE season.show_id = show.item_id AND COALESCE(u.is_watched, 0) = 0
            ) AS unwatched,
            MAX(item.added_at) AS latest_episode_added_at
        FROM tv_shows AS show
        JOIN tv_seasons AS season ON season.show_id = show.item_id
        JOIN tv_episodes AS episode ON episode.season_id = season.item_id
        JOIN media_items AS item ON item.id = episode.item_id
        WHERE unwatched > 0
        GROUP BY show.item_id
        ORDER BY latest_episode_added_at DESC, show.name
        LIMIT 30
    ";

    Ok(sqlx::query_as(sql).fetch_all(conn).await?)
}
