use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

use crate::utils;

use super::items::ExternalIds;
use super::media::MediaImage;
use super::videos::{self, VideoInfo, VideoUserData};

#[derive(Serialize)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub release_date: Option<i64>,
    pub overview: Option<String>,
    pub poster: Option<String>,
    pub backdrop: Option<String>,
    pub external_ids: ExternalIds,
    pub video_info: VideoInfo,
    pub user_data: VideoUserData,
}

impl<'r> FromRow<'r, SqliteRow> for Movie {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let poster: Option<&str> = row.try_get("poster")?;
        let backdrop: Option<&str> = row.try_get("backdrop")?;

        Ok(Movie {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            release_date: row.try_get("release_date")?,
            overview: row.try_get("overview")?,
            poster: poster.map(utils::get_image_url),
            backdrop: backdrop.map(utils::get_image_url),
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

pub async fn get(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Movie>> {
    let sql = "
        SELECT
            movie.item_id AS id,
            title,
            release_date,
            overview,
            poster,
            backdrop,
            tmdb_id,
            path,
            duration,
            COALESCE(is_watched, 0) AS is_watched,
            last_watched_at,
            position,
            format_name
        FROM movies AS movie
        JOIN video_files AS video ON movie.item_id = video.item_id
        LEFT JOIN user_item_data AS user_data ON movie.item_id = user_data.item_id
        WHERE movie.item_id = ?
    ";

    let mut movie: Movie = match sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?
    {
        Some(movie) => movie,
        None => return Ok(None),
    };

    movie.video_info = match videos::get_info(&mut *conn, id).await? {
        Some(info) => info,
        None => return Ok(None),
    };

    Ok(Some(movie))
}

pub async fn get_all(conn: &mut SqliteConnection) -> eyre::Result<Vec<Movie>> {
    let sql = "
        SELECT
            movie.item_id AS id,
            title,
            release_date,
            overview,
            poster,
            backdrop,
            tmdb_id,
            path,
            duration,
            COALESCE(is_watched, 0) AS is_watched,
            last_watched_at,
            position,
            format_name
        FROM movies AS movie
        JOIN video_files AS video ON movie.item_id = video.item_id
        LEFT JOIN user_item_data AS user_data ON movie.item_id = user_data.item_id
        ORDER BY title
    ";

    Ok(sqlx::query_as(sql).fetch_all(conn).await?)
}

pub async fn get_recently_added(conn: &mut SqliteConnection) -> eyre::Result<Vec<Movie>> {
    let sql = "
        SELECT
            movie.item_id AS id,
            title,
            release_date,
            overview,
            poster,
            backdrop,
            tmdb_id,
            path,
            duration,
            COALESCE(is_watched, 0) AS is_watched,
            last_watched_at,
            position,
            format_name
        FROM movies AS movie
        JOIN media_items AS item ON item.id = movie.item_id
        JOIN video_files AS video ON video.item_id = movie.item_id
        LEFT JOIN user_item_data AS user_data ON user_data.item_id = movie.item_id
        WHERE COALESCE(user_data.is_watched, 0) = 0
        ORDER BY added_at DESC, title
        LIMIT 30
    ";

    Ok(sqlx::query_as(sql).fetch_all(conn).await?)
}

pub struct UpdateMetadata<'a> {
    pub title: &'a str,
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
        UPDATE movies
        SET title    = ?,
            overview = ?,
            poster   = ?,
            backdrop = ?,
            tmdb_id  = ?
        WHERE item_id = ?
    ";

    sqlx::query(sql)
        .bind(data.title)
        .bind(data.overview)
        .bind(data.poster.map(|v| v.to_string()))
        .bind(data.backdrop.map(|v| v.to_string()))
        .bind(data.tmdb_id)
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}
