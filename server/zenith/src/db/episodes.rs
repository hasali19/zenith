use axum_codegen::Reflect;
use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

use crate::sql::Join;
use crate::{sql, utils};

use super::items::ExternalIds;
use super::media::MediaImage;
use super::videos::{self, VideoInfo, VideoUserData};

#[derive(Serialize, Reflect)]
pub struct Episode {
    pub id: i64,
    pub show_id: i64,
    pub season_id: i64,
    pub season_number: u32,
    pub episode_number: u32,
    pub name: Option<String>,
    pub show_name: String,
    pub air_date: Option<i64>,
    pub overview: Option<String>,
    pub poster: Option<String>,
    pub backdrop: Option<String>,
    pub thumbnail: Option<String>,
    pub external_ids: ExternalIds,
    pub video_info: VideoInfo,
    pub user_data: VideoUserData,
}

const EPISODE_COLUMNS: &[&str] = &[
    "e.item_id AS id",
    "show_id",
    "season_id",
    "season_number",
    "episode_number",
    "e.name",
    "sh.name AS show_name",
    "e.air_date",
    "e.overview",
    "COALESCE(se.poster, sh.poster) AS poster",
    "COALESCE(sh.backdrop, e.thumbnail) AS backdrop",
    "e.thumbnail",
    "e.tmdb_id",
    "v.path",
    "duration",
    "COALESCE(is_watched, 0) AS is_watched",
    "last_watched_at",
    "position",
    "format_name",
];

const EPISODE_JOINS: &[Join] = &[
    Join::inner("tv_seasons AS se").on("se.item_id = e.season_id"),
    Join::inner("tv_shows AS sh").on("sh.item_id = se.show_id"),
    Join::inner("video_files AS v").on("v.item_id = e.item_id"),
    Join::left("user_item_data AS u").on("u.item_id = e.item_id"),
];

impl<'r> FromRow<'r, SqliteRow> for Episode {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let poster: Option<&str> = row.try_get("poster")?;
        let backdrop: Option<&str> = row.try_get("backdrop")?;
        let thumbnail: Option<&str> = row.try_get("thumbnail")?;

        Ok(Episode {
            id: row.try_get("id")?,
            show_id: row.try_get("show_id")?,
            season_id: row.try_get("season_id")?,
            season_number: row.try_get("season_number")?,
            episode_number: row.try_get("episode_number")?,
            name: row.try_get("name")?,
            show_name: row.try_get("show_name")?,
            air_date: row.try_get("air_date")?,
            overview: row.try_get("overview")?,
            poster: poster.map(utils::get_image_url),
            backdrop: backdrop.map(utils::get_image_url),
            thumbnail: thumbnail.map(utils::get_image_url),
            external_ids: ExternalIds {
                tmdb: row.try_get("tmdb_id")?,
            },
            video_info: VideoInfo {
                path: row.try_get("path")?,
                duration: row.try_get("duration")?,
                format: row.try_get("format_name")?,
                audio: None,
                video: None,
                subtitles: None,
            },
            user_data: VideoUserData {
                is_watched: row.try_get("is_watched")?,
                position: row.try_get("position")?,
                last_watched_at: row.try_get("last_watched_at")?,
            },
        })
    }
}

pub async fn get(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Episode>> {
    let sql = sql::select("tv_episodes AS e")
        .columns(EPISODE_COLUMNS)
        .joins(EPISODE_JOINS)
        .condition("e.item_id = ?1")
        .to_sql();

    let mut episode: Episode = match sqlx::query_as(&sql)
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?
    {
        Some(episode) => episode,
        None => return Ok(None),
    };

    episode.video_info = match videos::get_info(&mut *conn, id).await? {
        Some(info) => info,
        None => return Ok(None),
    };

    Ok(Some(episode))
}

pub async fn get_for_season(
    conn: &mut SqliteConnection,
    season_id: i64,
) -> eyre::Result<Vec<Episode>> {
    let sql = sql::select("tv_episodes AS e")
        .columns(EPISODE_COLUMNS)
        .joins(EPISODE_JOINS)
        .condition("season_id = ?1")
        .order_by(&["episode_number"])
        .to_sql();

    Ok(sqlx::query_as(&sql).bind(season_id).fetch_all(conn).await?)
}

pub async fn get_for_show(conn: &mut SqliteConnection, show_id: i64) -> eyre::Result<Vec<Episode>> {
    let sql = sql::select("tv_episodes AS e")
        .columns(EPISODE_COLUMNS)
        .joins(EPISODE_JOINS)
        .condition("show_id = ?1")
        .order_by(&["season_number", "episode_number"])
        .to_sql();

    Ok(sqlx::query_as(&sql).bind(show_id).fetch_all(conn).await?)
}

pub struct UpdateMetadata<'a> {
    pub name: Option<&'a str>,
    pub overview: Option<&'a str>,
    pub thumbnail: Option<MediaImage<'a>>,
    pub tmdb_id: Option<i32>,
}

pub async fn update_metadata(
    conn: &mut SqliteConnection,
    id: i64,
    data: UpdateMetadata<'_>,
) -> eyre::Result<()> {
    let sql = "
        UPDATE tv_episodes
        SET name = ?,
            overview = ?,
            thumbnail = ?,
            tmdb_id = ?
        WHERE item_id = ?
    ";

    sqlx::query(sql)
        .bind(data.name)
        .bind(data.overview)
        .bind(data.thumbnail.map(|t| t.to_string()))
        .bind(data.tmdb_id)
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}
