use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

use crate::utils;

use super::collections::CollectionUserData;
use super::items::ExternalIds;
use super::media::MediaImage;

#[derive(Serialize)]
pub struct Season {
    pub id: i64,
    pub show_id: i64,
    pub season_number: u32,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub poster: Option<String>,
    pub backdrop: Option<String>,
    pub external_ids: ExternalIds,
    pub user_data: CollectionUserData,
}

impl<'r> FromRow<'r, SqliteRow> for Season {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let poster: Option<&str> = row.try_get("poster")?;
        let backdrop: Option<&str> = row.try_get("backdrop")?;

        Ok(Season {
            id: row.try_get("id")?,
            show_id: row.try_get("show_id")?,
            season_number: row.try_get("season_number")?,
            name: row.try_get("name")?,
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
    let sql = "
        SELECT
            season.item_id AS id,
            show_id,
            season_number,
            season.name,
            season.overview,
            season.poster,
            show.backdrop,
            season.tmdb_id,
            (
                SELECT COUNT(*)
                FROM tv_episodes AS episode
                LEFT JOIN user_item_data AS u ON u.item_id = episode.item_id
                WHERE episode.season_id = season.item_id AND COALESCE(u.is_watched, 0) = 0
            ) AS unwatched
        FROM tv_seasons AS season
        JOIN tv_shows AS show ON show.item_id = season.show_id
        WHERE season.item_id = ?
    ";

    Ok(sqlx::query_as(sql).bind(id).fetch_optional(conn).await?)
}

pub async fn get_for_show(conn: &mut SqliteConnection, show_id: i64) -> eyre::Result<Vec<Season>> {
    let sql = "
        SELECT
            season.item_id AS id,
            show_id,
            season_number,
            season.name,
            season.overview,
            season.poster,
            show.backdrop,
            season.tmdb_id,
            (
                SELECT COUNT(*)
                FROM tv_episodes AS episode
                LEFT JOIN user_item_data AS u ON u.item_id = episode.item_id
                WHERE episode.season_id = season.item_id AND COALESCE(u.is_watched, 0) = 0
            ) AS unwatched
        FROM tv_seasons AS season
        JOIN tv_shows AS show ON show.item_id = season.show_id
        WHERE season.show_id = ?
        ORDER BY season_number
    ";

    Ok(sqlx::query_as(sql).bind(show_id).fetch_all(conn).await?)
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
