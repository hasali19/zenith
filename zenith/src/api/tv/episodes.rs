use atium::respond::RespondRequestExt;
use atium::router::RouterRequestExt;
use atium::{endpoint, Request};
use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};

use crate::api::common::ExternalIds;
use crate::api::ext::OptionExt;
use crate::db::Db;
use crate::utils;

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
    is_watched: bool,
    external_ids: ExternalIds,
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
            is_watched: row.try_get(9)?,
            external_ids: ExternalIds {
                tmdb: row.try_get(10)?,
            },
        })
    }
}

#[endpoint]
pub(super) async fn get_episodes(req: &mut Request) -> eyre::Result<()> {
    let season_id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT
            episode.item_id, show.item_id, season.item_id, episode_number,
            episode.name, episode.air_date, episode.overview, episode.thumbnail,
            video.duration, COALESCE(user.is_watched, 0), episode.tmdb_id
        FROM tv_episodes AS episode
        JOIN tv_seasons AS season ON season.item_id = episode.season_id
        JOIN tv_shows AS show ON show.item_id = season.show_id
        JOIN video_files AS video ON video.item_id = episode.item_id
        LEFT JOIN user_item_data AS user ON user.item_id = episode.item_id
        WHERE episode.season_id = ?
        ORDER BY episode_number
    ";

    let episodes: Vec<Episode> = sqlx::query_as(sql)
        .bind(season_id)
        .fetch_all(&mut conn)
        .await?;

    req.ok().json(&episodes)?;

    Ok(())
}

#[endpoint]
pub(super) async fn get_episode(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT
            episode.item_id, show.item_id, season.item_id, episode_number,
            episode.name, episode.air_date, episode.overview, episode.thumbnail,
            video.duration, COALESCE(user.is_watched, 0), episode.tmdb_id
        FROM tv_episodes AS episode
        JOIN tv_seasons AS season ON season.item_id = episode.season_id
        JOIN tv_shows AS show ON show.item_id = season.show_id
        JOIN video_files AS video ON video.item_id = episode.item_id
        LEFT JOIN user_item_data AS user ON user.item_id = episode.item_id
        WHERE episode.item_id = ?
    ";

    let episode: Episode = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .or_not_found("episode not found")?;

    req.ok().json(&episode)?;

    Ok(())
}
