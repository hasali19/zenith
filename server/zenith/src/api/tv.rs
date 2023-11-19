use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum::Json;
use db::items::SortField;
use db::Db;
use speq::axum::get;

use crate::api::ApiResult;
use crate::MediaItemType;

use super::auth;
use super::dto::MediaItem;
use super::items::{query_items, query_items_by_id};

#[get("/shows")]
#[response(model = Vec<MediaItem>)]
pub async fn get_shows(user: auth::User, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let query = db::items::Query {
        item_type: Some(MediaItemType::Show),
        sort_by: &[SortField::Name],
        ..Default::default()
    };

    Ok(Json(query_items(&mut conn, user.id, query).await?))
}

#[get("/shows/recent")]
#[response(model = Vec<MediaItem>)]
pub async fn get_recent_shows(user: auth::User, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let ids = db::items::get_recently_updated_shows(&mut conn, user.id).await?;
    Ok(Json(query_items_by_id(&mut conn, user.id, &ids).await?))
}

#[get("/shows/:id/seasons")]
#[path(i64)]
#[response(model = Vec<MediaItem>)]
pub async fn get_seasons(
    show_id: Path<i64>,
    user: auth::User,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let query = db::items::Query {
        item_type: Some(MediaItemType::Season),
        parent_id: Some(*show_id),
        sort_by: &[SortField::ParentIndex],
        ..Default::default()
    };

    Ok(Json(query_items(&mut conn, user.id, query).await?))
}

#[get("/shows/:id/episodes")]
#[path(i64)]
#[response(model = Vec<MediaItem>)]
pub async fn get_show_episodes(
    show_id: Path<i64>,
    user: auth::User,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let query = db::items::Query {
        item_type: Some(MediaItemType::Episode),
        grandparent_id: Some(*show_id),
        sort_by: &[SortField::GrandparentIndex, SortField::ParentIndex],
        ..Default::default()
    };

    Ok(Json(query_items(&mut conn, user.id, query).await?))
}

#[get("/seasons/:id/episodes")]
#[path(i64)]
#[response(model = Vec<MediaItem>)]
pub async fn get_episodes(
    season_id: Path<i64>,
    user: auth::User,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let query = db::items::Query {
        item_type: Some(MediaItemType::Episode),
        parent_id: Some(*season_id),
        sort_by: &[SortField::ParentIndex],
        ..Default::default()
    };

    Ok(Json(query_items(&mut conn, user.id, query).await?))
}
