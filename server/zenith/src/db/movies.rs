use serde::Serialize;
use speq::Reflect;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

use crate::sql::Join;
use crate::{sql, utils};

use super::items::ExternalIds;
use super::videos::{self, VideoInfo, VideoUserData};

#[derive(Serialize, Reflect)]
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

const MOVIE_COLUMNS: &[&str] = &[
    "m.id",
    "name",
    "start_date",
    "overview",
    "poster",
    "backdrop",
    "tmdb_id",
    "path",
    "duration",
    "COALESCE(is_watched, 0) AS is_watched",
    "last_watched_at",
    "position",
    "format_name",
];

const MOVIE_JOINS: &[Join] = &[
    Join::inner("video_files AS v").on("m.id = v.item_id"),
    Join::left("user_item_data AS u").on("m.id = u.item_id"),
];

impl<'r> FromRow<'r, SqliteRow> for Movie {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let poster: Option<&str> = row.try_get("poster")?;
        let backdrop: Option<&str> = row.try_get("backdrop")?;

        Ok(Movie {
            id: row.try_get("id")?,
            title: row.try_get("name")?,
            release_date: row.try_get("start_date")?,
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
    let sql = sql::select("movies AS m")
        .columns(MOVIE_COLUMNS)
        .joins(MOVIE_JOINS)
        .condition("m.id = ?1")
        .to_sql();

    let mut movie: Movie = match sqlx::query_as(&sql)
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
