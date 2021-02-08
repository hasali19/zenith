use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};
use zenith_http::{Request, Response};

use crate::api::{ApiError, ApiResult};
use crate::{utils, AppState};

#[derive(Serialize)]
struct Episode {
    id: i64,
    show_id: i64,
    season_id: i64,
    episode_number: i32,
    name: Option<String>,
    air_date: Option<i64>,
    overview: Option<String>,
    thumbnail: Option<String>,
    duration: f64,
}

impl<'r> FromRow<'r, SqliteRow> for Episode {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let thumbnail: Option<String> = row.try_get(7)?;

        Ok(Episode {
            id: row.try_get(0)?,
            show_id: row.try_get(1)?,
            season_id: row.try_get(2)?,
            episode_number: row.try_get(3)?,
            name: row.try_get(4)?,
            air_date: row.try_get(5)?,
            overview: row.try_get(6)?,
            thumbnail: thumbnail.as_deref().map(utils::get_image_url),
            duration: row.try_get(8)?,
        })
    }
}

pub(super) async fn get_episodes(state: AppState, req: Request) -> ApiResult {
    let season_id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

    let sql = "
        SELECT
            episode.item_id, show.item_id, season.item_id, episode_number,
            episode.name, episode.air_date, episode.overview, episode.thumbnail,
            video.duration
        FROM tv_episodes AS episode
        JOIN tv_seasons AS season ON season.item_id = episode.season_id
        JOIN tv_shows AS show ON show.item_id = season.show_id
        JOIN video_files AS video ON video.item_id = episode.item_id
        WHERE episode.season_id = ?
        ORDER BY episode_number
    ";

    let episodes: Vec<Episode> = sqlx::query_as(sql)
        .bind(season_id)
        .fetch_all(&mut conn)
        .await?;

    Ok(Response::new().json(&episodes)?)
}

pub(super) async fn get_episode(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

    let sql = "
        SELECT
            episode.item_id, show.item_id, season.item_id, episode_number,
            episode.name, episode.air_date, episode.overview, episode.thumbnail,
            video.duration
        FROM tv_episodes AS episode
        JOIN tv_seasons AS season ON season.item_id = episode.season_id
        JOIN tv_shows AS show ON show.item_id = season.show_id
        JOIN video_files AS video ON video.item_id = episode.item_id
        WHERE episode.item_id = ?
    ";

    let episode: Episode = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .ok_or_else(ApiError::not_found)?;

    Ok(Response::new().json(&episode)?)
}
