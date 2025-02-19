use axum::Json;
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use db::Db;
use db::items::SortField;
use speq::axum::get;

use crate::MediaItemType;
use crate::api::ApiResult;

use super::auth;
use super::dto::MediaItem;
use super::items::{query_items, query_items_by_id};

#[get("/shows")]
#[response(model = Vec<MediaItem>)]
pub async fn get_shows(user: auth::User, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let query = db::items::Query {
        item_types: &[MediaItemType::Show],
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

#[get("/shows/{id}/seasons")]
#[response(model = Vec<MediaItem>)]
pub async fn get_seasons(
    show_id: Path<i64>,
    user: auth::User,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let query = db::items::Query {
        item_types: &[MediaItemType::Season],
        parent_id: Some(*show_id),
        sort_by: &[SortField::ParentIndex],
        ..Default::default()
    };

    Ok(Json(query_items(&mut conn, user.id, query).await?))
}

#[get("/shows/{id}/episodes")]
#[response(model = Vec<MediaItem>)]
pub async fn get_show_episodes(
    show_id: Path<i64>,
    user: auth::User,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let query = db::items::Query {
        item_types: &[MediaItemType::Episode],
        grandparent_id: Some(*show_id),
        sort_by: &[SortField::GrandparentIndex, SortField::ParentIndex],
        ..Default::default()
    };

    Ok(Json(query_items(&mut conn, user.id, query).await?))
}

#[get("/seasons/{id}/episodes")]
#[response(model = Vec<MediaItem>)]
pub async fn get_episodes(
    season_id: Path<i64>,
    user: auth::User,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let query = db::items::Query {
        item_types: &[MediaItemType::Episode],
        parent_id: Some(*season_id),
        sort_by: &[SortField::ParentIndex],
        ..Default::default()
    };

    Ok(Json(query_items(&mut conn, user.id, query).await?))
}
