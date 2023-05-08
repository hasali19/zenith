use axum::extract::{Extension, Path};
use axum::Json;
use db::items::SortField;
use db::Db;

use crate::MediaItemType;

use super::dto::MediaItem;
use super::ext::OptionExt;
use super::items::{query_items, query_items_by_id};
use super::{auth, ApiResult};

/// GET /movies
pub async fn get_movies(user: auth::User, db: Extension<Db>) -> ApiResult<Json<Vec<MediaItem>>> {
    let mut conn = db.acquire().await?;

    let query = db::items::Query {
        item_type: Some(MediaItemType::Movie),
        sort_by: &[SortField::Name],
        ..Default::default()
    };

    Ok(Json(query_items(&mut conn, user.id, query).await?))
}

/// GET /movies/:id
pub async fn get_movie(
    id: Path<i64>,
    user: auth::User,
    db: Extension<Db>,
) -> ApiResult<Json<MediaItem>> {
    let mut conn = db.acquire().await?;

    let movie = query_items_by_id(&mut conn, user.id, &[*id])
        .await?
        .into_iter()
        .next()
        .or_not_found("movie not found")?;

    Ok(Json(movie))
}

/// GET /movies/recent
pub async fn get_recent_movies(
    user: auth::User,
    db: Extension<Db>,
) -> ApiResult<Json<Vec<MediaItem>>> {
    let mut conn = db.acquire().await?;
    let ids = db::items::get_recently_added_movies(&mut conn, user.id).await?;
    Ok(Json(query_items_by_id(&mut conn, user.id, &ids).await?))
}
