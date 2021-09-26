use atium::respond::RespondRequestExt;
use atium::router::RouterRequestExt;
use atium::{endpoint, Request};

use crate::api::common::{self, Season};
use crate::api::ext::OptionExt;
use crate::db::Db;

#[endpoint]
pub(super) async fn get_seasons(req: &mut Request) -> eyre::Result<()> {
    let show_id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

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

    let seasons: Vec<Season> = sqlx::query_as(sql)
        .bind(show_id)
        .fetch_all(&mut conn)
        .await?;

    req.ok().json(&seasons)?;

    Ok(())
}

#[endpoint]
pub(super) async fn get_season(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let season = common::get_season_item(&mut conn, id)
        .await?
        .or_not_found("season not found")?;

    req.ok().json(&season)?;

    Ok(())
}
