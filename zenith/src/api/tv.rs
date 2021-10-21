use atium::responder::Json;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request};

use crate::db::{self, Db};

use super::ext::OptionExt;
use super::import::{import_episode, import_show};

pub fn routes(router: &mut Router) {
    router.route("/tv/shows").get(get_shows).post(import_show);
    router.route("/tv/shows/:id").get(get_show);
    router
        .route("/tv/shows/recent")
        .get(get_recently_updated_shows);
    router.route("/tv/shows/:id/seasons").get(get_seasons);
    router
        .route("/tv/shows/:show_id/episodes")
        .post(import_episode);
    router.route("/tv/seasons/:id").get(get_season);
    router.route("/tv/seasons/:id/episodes").get(get_episodes);
    router.route("/tv/episodes/:id").get(get_episode);
}

#[endpoint]
pub async fn get_shows(req: &mut Request) -> eyre::Result<impl Responder> {
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;
    let shows = db::shows::get_all(&mut conn).await?;
    Ok(Json(shows))
}

#[endpoint]
pub async fn get_show(req: &mut Request) -> eyre::Result<impl Responder> {
    let id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let show = db::shows::get(&mut conn, id)
        .await?
        .or_not_found("show not found")?;

    Ok(Json(show))
}

#[endpoint]
pub async fn get_recently_updated_shows(req: &mut Request) -> eyre::Result<impl Responder> {
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let shows = db::shows::get_recently_updated(&mut conn).await?;

    Ok(Json(shows))
}

#[endpoint]
pub async fn get_seasons(req: &mut Request) -> eyre::Result<impl Responder> {
    let show_id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let seasons = db::seasons::get_for_show(&mut conn, show_id).await?;

    Ok(Json(seasons))
}

#[endpoint]
pub async fn get_season(req: &mut Request) -> eyre::Result<impl Responder> {
    let id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let season = db::seasons::get(&mut conn, id)
        .await?
        .or_not_found("season not found")?;

    Ok(Json(season))
}

#[endpoint]
pub(super) async fn get_episodes(req: &mut Request) -> eyre::Result<impl Responder> {
    let season_id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let episodes = db::episodes::get_for_season(&mut conn, season_id).await?;

    Ok(Json(episodes))
}

#[endpoint]
pub(super) async fn get_episode(req: &mut Request) -> eyre::Result<impl Responder> {
    let id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let episode = db::episodes::get(&mut conn, id)
        .await?
        .or_not_found("episode not found")?;

    Ok(Json(episode))
}
