use axum::extract::{Extension, Path};
use axum::Json;
use db::items::SortField;
use db::Db;
use speq::axum::get;

use crate::MediaItemType;

use super::dto::MediaItem;
use super::ext::OptionExt;
use super::items::{query_items, query_items_by_id};
use super::ApiResult;

#[get("/movies")]
#[response(model = Vec<MediaItem>)]
pub async fn get_movies(db: Extension<Db>) -> ApiResult<Json<Vec<MediaItem>>> {
    let mut conn = db.acquire().await?;

    let query = db::items::Query {
        item_type: Some(MediaItemType::Movie),
        sort_by: &[SortField::Name],
        ..Default::default()
    };

    Ok(Json(query_items(&mut conn, query).await?))
}

#[get("/movies/:id")]
#[path(i64)]
#[response(model = MediaItem)]
pub async fn get_movie(id: Path<i64>, db: Extension<Db>) -> ApiResult<Json<MediaItem>> {
    let mut conn = db.acquire().await?;

    let movie = query_items_by_id(&mut conn, &[*id])
        .await?
        .into_iter()
        .next()
        .or_not_found("movie not found")?;

    Ok(Json(movie))
}

#[get("/movies/recent")]
#[response(model = Vec<MediaItem>)]
pub async fn get_recent_movies(db: Extension<Db>) -> ApiResult<Json<Vec<MediaItem>>> {
    let mut conn = db.acquire().await?;
    let ids = db::items::get_recently_added_movies(&mut conn).await?;
    Ok(Json(query_items_by_id(&mut conn, &ids).await?))
}
