use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use speq::axum::{delete, get, post, put};
use speq::Reflect;

use crate::{db, utils, Db};

use super::ApiResult;

#[derive(Serialize, Reflect)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub overview: Option<String>,
    pub poster: Option<String>,
}

impl From<db::collections::Collection> for Collection {
    fn from(collection: db::collections::Collection) -> Self {
        Collection {
            id: collection.id,
            name: collection.name,
            overview: collection.overview,
            poster: collection.poster.map(utils::get_image_url),
        }
    }
}

#[get("/collections")]
#[response(model = Vec<Collection>)]
async fn get_collections(db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let collections = db::collections::get_all(&mut conn)
        .await?
        .into_iter()
        .map(Collection::from)
        .collect_vec();
    Ok(Json(collections))
}

#[get("/collections/:id")]
#[path(i64)]
#[response(model = Collection)]
async fn get_collection(id: Path<i64>, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let collection = db::collections::get(&mut conn, *id)
        .await?
        .map(Collection::from);
    Ok(Json(collection))
}

#[derive(Deserialize)]
struct NewCollection {
    name: String,
}

/// xyz
#[post("/collections")]
#[response(model = Collection)]
async fn create_collection(
    db: Extension<Db>,
    data: Json<NewCollection>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let data = db::collections::NewCollection { name: &data.name };
    let collection = db::collections::create(&mut conn, data).await?;
    Ok(Json(Collection::from(collection)))
}

#[delete("/collections/:id")]
async fn delete_collection(id: Path<i64>, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    db::collections::remove(&mut conn, *id).await?;
    Ok(())
}

#[derive(Deserialize)]
struct UpdateCollection {
    meta: Option<UpdateCollectionMeta>,
    items: Option<Vec<i64>>,
}

#[derive(Deserialize)]
struct UpdateCollectionMeta {
    name: String,
    overview: Option<String>,
}

#[put("/collections/:id")]
#[response(model = Collection)]
async fn update_collection(
    id: Path<i64>,
    db: Extension<Db>,
    data: Json<UpdateCollection>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    if let Some(meta) = &data.meta {
        let data = db::collections::UpdateCollection {
            name: &meta.name,
            overview: meta.overview.as_deref(),
        };
        db::collections::update(&mut conn, *id, data).await?;
    }

    if let Some(items) = &data.items {
        db::collections::set_items(&mut conn, *id, items).await?;
    }

    Ok(())
}
