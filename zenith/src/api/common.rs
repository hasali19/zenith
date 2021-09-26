use serde::Serialize;
use serde_json::Value;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

use crate::utils;

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

#[derive(Serialize)]
pub struct ExternalIds {
    pub tmdb: Option<i32>,
}

#[derive(Serialize)]
pub struct VideoInfo {
    pub path: String,
    pub duration: f64,
    #[serde(flatten)]
    pub extended: Option<Value>,
}

#[derive(Serialize)]
pub struct VideoUserData {
    pub is_watched: bool,
    pub position: Option<f64>,
}

#[derive(Serialize)]
pub struct CollectionUserData {
    pub unwatched: u32,
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
                extended: None,
            },
            user_data: VideoUserData {
                is_watched: row.try_get("is_watched")?,
                position: row.try_get("position")?,
            },
        })
    }
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
                extended: None,
            },
            user_data: VideoUserData {
                is_watched: row.try_get("is_watched")?,
                position: row.try_get("position")?,
            },
        })
    }
}

pub async fn get_movie_item(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Movie>> {
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
            position
        FROM movies AS movie
        JOIN video_files AS video ON movie.item_id = video.item_id
        LEFT JOIN user_item_data AS user_data ON movie.item_id = user_data.item_id
        WHERE movie.item_id = ?
    ";

    Ok(sqlx::query_as(sql).bind(id).fetch_optional(conn).await?)
}

pub async fn get_show_item(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Show>> {
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

pub async fn get_season_item(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Season>> {
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

pub async fn get_episode_item(
    conn: &mut SqliteConnection,
    id: i64,
) -> eyre::Result<Option<Episode>> {
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
            position
        FROM tv_episodes AS episode
        JOIN tv_seasons AS season ON season.item_id = episode.season_id
        JOIN tv_shows AS show ON show.item_id = season.show_id
        JOIN video_files AS video ON video.item_id = episode.item_id
        LEFT JOIN user_item_data AS user_data ON user_data.item_id = episode.item_id
        WHERE episode.item_id = ?
    ";

    Ok(sqlx::query_as(sql).bind(id).fetch_optional(conn).await?)
}
