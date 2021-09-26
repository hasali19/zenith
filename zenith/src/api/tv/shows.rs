use atium::respond::RespondRequestExt;
use atium::router::RouterRequestExt;
use atium::{endpoint, Request};

use crate::api::common::{self, Show};
use crate::api::ext::OptionExt;
use crate::db::Db;

#[endpoint]
pub(super) async fn get_shows(req: &mut Request) -> eyre::Result<()> {
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

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
        ORDER BY name
    ";

    let shows: Vec<Show> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

    req.ok().json(&shows)?;

    Ok(())
}

#[endpoint]
pub(super) async fn get_show(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let show = common::get_show_item(&mut conn, id)
        .await?
        .or_not_found("show not found")?;

    req.ok().json(&show)?;

    Ok(())
}

#[endpoint]
pub(super) async fn get_recently_updated_shows(req: &mut Request) -> eyre::Result<()> {
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    // Get shows sorted by the added_at of their most recently added episode
    // (i.e. shows that have had an episode added recently will appear higher up)
    let sql = "
        SELECT
            show.item_id AS id,
            show.name,
            start_date,
            end_date,
            show.overview,
            show.poster,
            show.backdrop,
            show.tmdb_id,
            (
                SELECT COUNT(*)
                FROM tv_episodes AS episode
                JOIN tv_seasons AS season ON season.item_id = episode.season_id
                LEFT JOIN user_item_data AS u ON u.item_id = episode.item_id
                WHERE season.show_id = show.item_id AND COALESCE(u.is_watched, 0) = 0
            ) AS unwatched,
            MAX(item.added_at) AS latest_episode_added_at
        FROM tv_shows AS show
        JOIN tv_seasons AS season ON season.show_id = show.item_id
        JOIN tv_episodes AS episode ON episode.season_id = season.item_id
        JOIN media_items AS item ON item.id = episode.item_id
        WHERE unwatched > 0
        GROUP BY show.item_id
        ORDER BY latest_episode_added_at DESC, show.name
        LIMIT 30
    ";

    let shows: Vec<Show> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

    req.ok().json(&shows)?;

    Ok(())
}
