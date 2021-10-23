use actix_web::web::{Json, Path, Query};
use actix_web::{get, patch, Responder};
use serde::Deserialize;

use crate::api::error::bad_request;
use crate::api::ApiResult;
use crate::db::media::MediaItemType;
use crate::db::videos::UpdateVideoUserData;
use crate::db::{self, Db};

use super::ext::OptionExt;

#[derive(Deserialize)]
struct GetItemsQuery {
    #[serde(default)]
    ids: Vec<i64>,
}

#[get("/items")]
async fn get_items(Query(query): Query<GetItemsQuery>, db: Db) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;
    let items = db::items::get_multiple(&mut conn, query.ids).await?;
    Ok(Json(items))
}

#[get("/items/{id}")]
pub async fn get_item(id: Path<i64>, db: Db) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;

    let item = db::items::get(&mut conn, *id)
        .await?
        .or_not_found("media item not found")?;

    Ok(Json(item))
}

#[derive(Deserialize)]
struct VideoUserDataPatch {
    #[serde(default)]
    is_watched: Option<bool>,
    #[serde(default)]
    position: Option<f64>,
}

#[patch("/items/{id}/user_data")]
async fn update_user_data(
    id: Path<i64>,
    data: Json<VideoUserDataPatch>,
    db: Db,
) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;

    let item_type = db::media::get_item_type(&mut conn, *id)
        .await?
        .or_not_found("media item not found")?;

    if !matches!(item_type, MediaItemType::Movie | MediaItemType::TvEpisode) {
        return Err(bad_request(
            "updating user data is only allowed for movies and episodes",
        ));
    }

    let data = UpdateVideoUserData {
        is_watched: data.is_watched,
        position: data.position,
    };

    let data = db::videos::update_user_data(&mut conn, *id, data).await?;

    Ok(Json(data))
}
