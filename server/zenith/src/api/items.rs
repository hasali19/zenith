use std::collections::HashMap;
use std::sync::Arc;

use axum::Json;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use db::videos::UpdateVideoUserData;
use db::{Db, ReadConnection};
use itertools::Itertools;
use serde::Deserialize;
use serde_qs::axum::QsQuery;
use speq::Reflect;
use speq::axum::{delete, get, patch};

use crate::api::ApiResult;
use crate::library::MediaLibrary;

use super::auth;
use super::dto::{MediaItem, MediaItemType};
use super::error::bad_request;
use super::ext::OptionExt;

#[derive(Debug, Deserialize, Reflect)]
struct ItemsQuery {
    #[serde(default)]
    ids: Vec<i64>,
    #[serde(default)]
    item_type: Vec<MediaItemType>,
    parent_id: Option<i64>,
    grandparent_id: Option<i64>,
    collection_id: Option<i64>,
    name: Option<String>,
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
    QsQuery(query): QsQuery<ItemsQuery>,
    user: auth::User,
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
        item_types: &query.item_type.into_iter().map(Into::into).collect_vec(),
        parent_id: query.parent_id,
        grandparent_id: query.grandparent_id,
        collection_id: query.collection_id,
        name: query.name.as_deref(),
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
        limit: query.limit,
        offset: query.offset,
    };

    Ok(Json(query_items(&mut conn, user.id, query).await?))
}

#[derive(Deserialize, Reflect)]
struct ContinueWatchingQuery {
    limit: Option<u32>,
}

#[get("/items/continue_watching")]
#[response(model = Vec<MediaItem>)]
async fn get_continue_watching(
    QsQuery(query): QsQuery<ContinueWatchingQuery>,
    user: auth::User,
    Extension(db): Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let limit = query.limit.unwrap_or(10);
    let ids = db::items::get_continue_watching(&mut conn, user.id, Some(limit)).await?;
    Ok(Json(query_items_by_id(&mut conn, user.id, &ids).await?))
}

#[get("/items/{id}")]
#[response(model = MediaItem)]
pub async fn get_item(
    id: Path<i64>,
    user: auth::User,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    Ok(Json(
        query_items_by_id(&mut conn, user.id, &[*id])
            .await?
            .into_iter()
            .next()
            .or_not_found("item not found")?,
    ))
}

pub(super) async fn query_items_by_id(
    conn: &mut ReadConnection,
    user_id: i64,
    ids: &[i64],
) -> eyre::Result<Vec<MediaItem>> {
    let query = db::items::Query {
        ids: Some(ids),
        ..Default::default()
    };

    let mut items = query_items(conn, user_id, query)
        .await?
        .into_iter()
        .map(|item| (item.id, item))
        .collect::<HashMap<_, _>>();

    Ok(ids.iter().filter_map(|id| items.remove(id)).collect_vec())
}

pub(super) async fn query_items(
    conn: &mut ReadConnection,
    user_id: i64,
    query: db::items::Query<'_>,
) -> eyre::Result<Vec<MediaItem>> {
    let items = db::items::query(&mut *conn, query).await?;
    let ids = items.iter().map(|item| item.id).collect_vec();

    let mut video_user_data = db::items::get_video_user_data_for_items(&mut *conn, user_id, &ids)
        .await?
        .into_iter()
        .map(|user_data| (user_data.item_id, user_data))
        .collect::<HashMap<_, _>>();

    let mut collection_user_data =
        db::items::get_collection_user_data_for_items(&mut *conn, user_id, &ids)
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
                    position_updated_at: None,
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

#[derive(Deserialize, Reflect)]
pub struct DeleteItemQuery {
    #[serde(default)]
    remove_files: bool,
}

#[delete("/items/{id}")]
#[response(status = 200)]
async fn delete_item(
    Path(id): Path<i64>,
    QsQuery(query): QsQuery<DeleteItemQuery>,
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
        if query.remove_files {
            tokio::fs::remove_file(&file.path).await?;
        }
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

#[patch("/items/{id}/user_data")]
#[response(status = 200)]
async fn update_user_data(
    id: Path<i64>,
    user: auth::User,
    db: Extension<Db>,
    data: Json<VideoUserDataPatch>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire_write().await?;

    let item_type = db::media::get_item_type(conn.as_read(), *id)
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
                db::items::query(conn.as_read(), query)
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
                db::items::query(conn.as_read(), query)
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
            set_position_updated: false,
        };

        db::videos::update_user_data(&mut conn, id, user.id, data).await?;
    }

    Ok(StatusCode::OK)
}
