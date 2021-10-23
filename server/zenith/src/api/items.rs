use actix_web::web::{Json, Path, Query};
use actix_web::{get, patch, HttpResponse, Responder};
use serde::Deserialize;

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

    let mut ids = vec![];

    match item_type {
        MediaItemType::Movie | MediaItemType::TvEpisode => ids.push(*id),
        MediaItemType::TvShow => ids.extend(
            db::episodes::get_for_show(&mut conn, *id)
                .await?
                .iter()
                .map(|e| e.id),
        ),
        MediaItemType::TvSeason => ids.extend(
            db::episodes::get_for_season(&mut conn, *id)
                .await?
                .iter()
                .map(|e| e.id),
        ),
    };

    for id in ids {
        let data = UpdateVideoUserData {
            is_watched: data.is_watched,
            position: data.position,
        };

        db::videos::update_user_data(&mut conn, id, data).await?;
    }

    Ok(HttpResponse::Ok())
}
