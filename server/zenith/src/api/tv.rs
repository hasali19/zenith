use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum::Json;
use axum_codegen::get;

use crate::api::ApiResult;
use crate::db::{self, Db};

use super::ext::OptionExt;

#[get("/tv/shows")]
pub async fn get_shows(db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let shows = db::shows::get_all(&mut conn).await?;
    Ok(Json(shows))
}

#[get("/tv/shows/:id")]
pub async fn get_show(id: Path<i64>, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let show = db::shows::get(&mut conn, *id)
        .await?
        .or_not_found("show not found")?;

    Ok(Json(show))
}

#[get("/tv/shows/recent")]
pub async fn get_recently_updated_shows(db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let shows = db::shows::get_recently_updated(&mut conn).await?;
    Ok(Json(shows))
}

#[get("/tv/shows/:id/seasons")]
pub async fn get_seasons(show_id: Path<i64>, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let seasons = db::seasons::get_for_show(&mut conn, *show_id).await?;
    Ok(Json(seasons))
}

#[get("/tv/shows/:id/episodes")]
pub async fn get_show_episodes(
    show_id: Path<i64>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let episodes = db::episodes::get_for_show(&mut conn, *show_id).await?;
    Ok(Json(episodes))
}

#[get("/tv/seasons/:id")]
pub async fn get_season(id: Path<i64>, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let season = db::seasons::get(&mut conn, *id)
        .await?
        .or_not_found("season not found")?;

    Ok(Json(season))
}

#[get("/tv/seasons/:id/episodes")]
pub async fn get_episodes(season_id: Path<i64>, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let episodes = db::episodes::get_for_season(&mut conn, *season_id).await?;
    Ok(Json(episodes))
}

#[get("/tv/episodes/:id")]
pub async fn get_episode(id: Path<i64>, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let episode = db::episodes::get(&mut conn, *id)
        .await?
        .or_not_found("episode not found")?;

    Ok(Json(episode))
}
