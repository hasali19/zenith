use atium::respond::RespondRequestExt;
use atium::router::RouterRequestExt;
use atium::{endpoint, Request};

use crate::api::common::{self, Episode};
use crate::api::ext::OptionExt;
use crate::db::Db;

#[endpoint]
pub(super) async fn get_episodes(req: &mut Request) -> eyre::Result<()> {
    let season_id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

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

    let episode = common::get_episode_item(&mut conn, id)
        .await?
        .or_not_found("episode not found")?;

    req.ok().json(&episode)?;

    Ok(())
}
