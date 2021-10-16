use atium::query::QueryRequestExt;
use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request};
use serde::Deserialize;

use crate::api::error::bad_request;
use crate::db::media::MediaItemType;
use crate::db::videos::UpdateVideoUserData;
use crate::db::{self, Db};

use super::ext::OptionExt;

pub fn routes(router: &mut Router) {
    router.route("/items").get(get_items);
    router.route("/items/:id").get(get_item);
    router.route("/items/:id/user_data").patch(update_user_data);
}

#[derive(Deserialize)]
struct GetItemsQuery {
    #[serde(default)]
    ids: Vec<i64>,
}

#[endpoint]
async fn get_items(req: &mut Request) -> eyre::Result<()> {
    let query: GetItemsQuery = req.query()?;
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let items = db::items::get_multiple(&mut conn, query.ids).await?;

    req.ok().json(&items)?;

    Ok(())
}

#[endpoint]
async fn get_item(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let item = db::items::get(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    req.ok().json(&item)?;

    Ok(())
}

#[derive(Deserialize)]
struct VideoUserDataPatch {
    #[serde(default)]
    is_watched: Option<bool>,
    #[serde(default)]
    position: Option<f64>,
}

#[endpoint]
async fn update_user_data(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let item_type = db::media::get_item_type(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    if !matches!(item_type, MediaItemType::Movie | MediaItemType::TvEpisode) {
        return bad_request("updating user data is only allowed for movies and episodes").into();
    }

    let data: VideoUserDataPatch = req.body_json().await.map_err(bad_request)?;

    if data.is_watched.is_none() && data.position.is_none() {
        req.ok();
        return Ok(());
    }

    let data = UpdateVideoUserData {
        is_watched: data.is_watched,
        position: data.position,
    };

    let data = db::videos::update_user_data(&mut conn, id, data).await?;

    req.ok().json(&data)?;

    Ok(())
}
