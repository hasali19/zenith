use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

use crate::utils;

use super::items::ExternalIds;
use super::videos::{self, VideoInfo, VideoUserData};

#[derive(Serialize)]
pub struct Episode {
    pub id: i64,
    pub show_id: i64,
    pub season_id: i64,
    pub season_number: u32,
    pub episode_number: u32,
    pub name: Option<String>,
    pub air_date: Option<i64>,
    pub overview: Option<String>,
    pub thumbnail: Option<String>,
    pub external_ids: ExternalIds,
    pub video_info: VideoInfo,
    pub user_data: VideoUserData,
}

impl<'r> FromRow<'r, SqliteRow> for Episode {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let thumbnail: Option<&str> = row.try_get("thumbnail")?;

        Ok(Episode {
            id: row.try_get("id")?,
            show_id: row.try_get("show_id")?,
            season_id: row.try_get("season_id")?,
            season_number: row.try_get("season_number")?,
            episode_number: row.try_get("episode_number")?,
            name: row.try_get("name")?,
            air_date: row.try_get("air_date")?,
            overview: row.try_get("overview")?,
            thumbnail: thumbnail.as_deref().map(utils::get_image_url),
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
            },
        })
    }
}

pub async fn get(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Episode>> {
    let sql = "
        SELECT
            episode.item_id AS id,
            show_id,
            season_id,
            season_number,
            episode_number,
            episode.name,
            episode.air_date,
            episode.overview,
            episode.thumbnail,
            episode.tmdb_id,
            video.path,
            duration,
            COALESCE(is_watched, 0) AS is_watched,
            position,
            format_name
        FROM tv_episodes AS episode
        JOIN tv_seasons AS season ON season.item_id = episode.season_id
        JOIN tv_shows AS show ON show.item_id = season.show_id
        JOIN video_files AS video ON video.item_id = episode.item_id
        LEFT JOIN user_item_data AS user_data ON user_data.item_id = episode.item_id
        WHERE episode.item_id = ?
    ";

    let mut episode: Episode = match sqlx::query_as(sql)
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
    let sql = "
        SELECT
            episode.item_id AS id,
            show_id,
            season_id,
            season_number,
            episode_number,
            episode.name,
            episode.air_date,
            episode.overview,
            episode.thumbnail,
            episode.tmdb_id,
            video.path,
            duration,
            COALESCE(is_watched, 0) AS is_watched,
            position,
            format_name
        FROM tv_episodes AS episode
        JOIN tv_seasons AS season ON season.item_id = episode.season_id
        JOIN tv_shows AS show ON show.item_id = season.show_id
        JOIN video_files AS video ON video.item_id = episode.item_id
        LEFT JOIN user_item_data AS user ON user.item_id = episode.item_id
        WHERE episode.season_id = ?
        ORDER BY episode_number
    ";

    Ok(sqlx::query_as(sql).bind(season_id).fetch_all(conn).await?)
}
