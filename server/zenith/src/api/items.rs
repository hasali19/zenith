use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use db::videos::UpdateVideoUserData;
use db::Db;
use itertools::Itertools;
use serde::Deserialize;
use serde_qs::axum::QsQuery;
use speq::axum::{delete, get, patch};
use speq::Reflect;
use sqlx::SqliteConnection;

use crate::api::ApiResult;
use crate::library::MediaLibrary;

use super::dto::{MediaItem, MediaItemType};
use super::error::bad_request;
use super::ext::OptionExt;

#[derive(Debug, Deserialize, Reflect)]
struct ItemsQuery {
    #[serde(default)]
    ids: Vec<i64>,
    item_type: Option<MediaItemType>,
    parent_id: Option<i64>,
    grandparent_id: Option<i64>,
    collection_id: Option<i64>,
    is_watched: Option<bool>,
    #[serde(default)]
    sort_by: Vec<ItemSortField>,
    limit: Option<u32>,
    offset: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Reflect)]
#[serde(rename_all = "snake_case")]
enum ItemSortField {
    Name,
    ParentIndex,
    GrandparentIndex,
    CollectionIndex,
}

#[get("/items")]
#[response(model = Vec<MediaItem>)]
async fn get_items(
    #[query] QsQuery(query): QsQuery<ItemsQuery>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    if query.sort_by.contains(&ItemSortField::CollectionIndex) && query.collection_id.is_none() {
        return Err(bad_request(
            "sorting by collection_index requires a collection id",
        ));
    }

    let query = db::items::Query {
        ids: if query.ids.is_empty() {
            None
        } else {
            Some(&query.ids)
        },
        item_type: query.item_type.map(Into::into),
        parent_id: query.parent_id,
        grandparent_id: query.grandparent_id,
        collection_id: query.collection_id,
        sort_by: &query
            .sort_by
            .iter()
            .map(|f| match f {
                ItemSortField::Name => db::items::SortField::Name,
                ItemSortField::ParentIndex => db::items::SortField::ParentIndex,
                ItemSortField::GrandparentIndex => db::items::SortField::GrandparentIndex,
                ItemSortField::CollectionIndex => db::items::SortField::CollectionIndex,
            })
            .collect_vec(),
        is_watched: query.is_watched,
        limit: query.limit,
        offset: query.offset,
    };

    Ok(Json(query_items(&mut conn, query).await?))
}

#[derive(Deserialize, Reflect)]
struct ContinueWatchingQuery {
    limit: Option<u32>,
}

#[get("/items/continue_watching")]
#[response(model = Vec<MediaItem>)]
async fn get_continue_watching(
    #[query] QsQuery(query): QsQuery<ContinueWatchingQuery>,
    Extension(db): Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let limit = query.limit.unwrap_or(10);
    let ids = db::items::get_continue_watching(&mut conn, Some(limit)).await?;
    Ok(Json(query_items_by_id(&mut conn, &ids).await?))
}

#[get("/items/:id")]
#[path(i64)]
#[response(model = MediaItem)]
pub async fn get_item(id: Path<i64>, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    Ok(Json(
        query_items_by_id(&mut conn, &[*id])
            .await?
            .into_iter()
            .next()
            .or_not_found("item not found")?,
    ))
}

pub(super) async fn query_items_by_id(
    conn: &mut SqliteConnection,
    ids: &[i64],
) -> eyre::Result<Vec<MediaItem>> {
    let query = db::items::Query {
        ids: Some(ids),
        ..Default::default()
    };

    let mut items = query_items(conn, query)
        .await?
        .into_iter()
        .map(|item| (item.id, item))
        .collect::<HashMap<_, _>>();

    Ok(ids.iter().filter_map(|id| items.remove(id)).collect_vec())
}

pub(super) async fn query_items(
    conn: &mut SqliteConnection,
    query: db::items::Query<'_>,
) -> eyre::Result<Vec<MediaItem>> {
    let items = db::items::query(&mut *conn, query).await?;
    let ids = items.iter().map(|item| item.id).collect_vec();

    let mut video_user_data = db::items::get_user_data_for_videos(&mut *conn, &ids)
        .await?
        .into_iter()
        .map(|user_data| (user_data.item_id, user_data))
        .collect::<HashMap<_, _>>();

    let mut collection_user_data = db::items::get_user_data_for_collections(&mut *conn, &ids)
        .await?
        .into_iter()
        .map(|user_data| (user_data.item_id, user_data))
        .collect::<HashMap<_, _>>();

    let mut res = vec![];
    for item in items {
        let user_data = match item.kind {
            db::media::MediaItemType::Movie | db::media::MediaItemType::Episode => video_user_data
                .remove(&item.id)
                .unwrap_or(db::items::VideoUserData {
                    item_id: item.id,
                    is_watched: false,
                    position: 0.0,
                    last_watched_at: None,
                })
                .into(),
            db::media::MediaItemType::Show | db::media::MediaItemType::Season => {
                collection_user_data
                    .remove(&item.id)
                    .unwrap_or(db::items::CollectionUserData {
                        item_id: item.id,
                        unwatched: 0,
                    })
                    .into()
            }
        };

        let item = MediaItem {
            user_data: Some(user_data),
            ..MediaItem::from(item)
        };

        res.push(item);
    }

    Ok(res)
}

#[delete("/items/:id")]
#[path(i64)]
#[response(status = 200)]
async fn delete_item(
    Path(id): Path<i64>,
    Extension(db): Extension<Db>,
    Extension(library): Extension<Arc<MediaLibrary>>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let item = db::items::get(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    let files = db::video_files::get_recursive_for_item(&mut conn, item.id).await?;

    for file in files {
        tracing::info!("removing file: {}", file.path);
        tokio::fs::remove_file(&file.path).await?;
        library.remove_video(&file.path).await?;
    }

    library.validate().await?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Reflect)]
struct VideoUserDataPatch {
    #[serde(default)]
    is_watched: Option<bool>,
    #[serde(default)]
    position: Option<f64>,
}

#[patch("/items/:id/user_data")]
#[path(i64)]
#[request(model = VideoUserDataPatch)]
#[response(status = 200)]
async fn update_user_data(
    id: Path<i64>,
    db: Extension<Db>,
    data: Json<VideoUserDataPatch>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let item_type = db::media::get_item_type(&mut conn, *id)
        .await?
        .or_not_found("media item not found")?;

    let mut ids = vec![];

    match item_type {
        db::media::MediaItemType::Movie | db::media::MediaItemType::Episode => ids.push(*id),
        db::media::MediaItemType::Show => {
            let query = db::items::Query {
                grandparent_id: Some(*id),
                ..Default::default()
            };

            ids.extend(
                db::items::query(&mut conn, query)
                    .await?
                    .iter()
                    .map(|e| e.id),
            )
        }
        db::media::MediaItemType::Season => {
            let query = db::items::Query {
                parent_id: Some(*id),
                ..Default::default()
            };

            ids.extend(
                db::items::query(&mut conn, query)
                    .await?
                    .iter()
                    .map(|e| e.id),
            )
        }
    };

    for id in ids {
        let data = UpdateVideoUserData {
            is_watched: data.is_watched,
            position: data.position,
            set_watched_at: false,
        };

        db::videos::update_user_data(&mut conn, id, data).await?;
    }

    Ok(StatusCode::OK)
}
