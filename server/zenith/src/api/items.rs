use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_codegen::{delete, get, patch, Reflect};
use serde::Deserialize;
use serde_qs::axum::QsQuery;

use crate::api::ApiResult;
use crate::db::items::MediaItem;
use crate::db::media::MediaItemType;
use crate::db::videos::UpdateVideoUserData;
use crate::db::{self, Db};
use crate::library::scanner::{ScanOptions, VideoFileType};
use crate::library::LibraryScanner;

use super::ext::OptionExt;

#[derive(Deserialize)]
struct GetItemsQuery {
    #[serde(default)]
    ids: Vec<i64>,
}

#[get("/items")]
#[query(name = "ids", model = Vec<i64>)]
#[response(model = Vec<MediaItem>)]
async fn get_items(
    QsQuery(query): QsQuery<GetItemsQuery>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let items = db::items::get_multiple(&mut conn, query.ids).await?;
    Ok(Json(items))
}

#[derive(Deserialize)]
struct ContinueWatchingQuery {
    limit: Option<u32>,
}

#[get("/items/continue_watching")]
#[query(name = "limit", model = Option<u32>)]
#[response(model = Vec<MediaItem>)]
async fn get_continue_watching(
    QsQuery(query): QsQuery<ContinueWatchingQuery>,
    Extension(db): Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;
    let limit = query.limit.unwrap_or(10);
    let items = db::items::get_continue_watching(&mut conn, Some(limit)).await?;
    Ok(Json(items))
}

#[get("/items/:id")]
#[path(name = "id", model = i64)]
#[response(model = MediaItem)]
pub async fn get_item(id: Path<i64>, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let item = db::items::get(&mut conn, *id)
        .await?
        .or_not_found("media item not found")?;

    Ok(Json(item))
}

#[delete("/items/:id")]
#[path(name = "id", model = i64)]
#[response(status = 200)]
async fn delete_item(
    Path(id): Path<i64>,
    Extension(db): Extension<Db>,
    Extension(scanner): Extension<Arc<LibraryScanner>>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let item = db::items::get(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    let mut files = vec![];
    match item {
        MediaItem::Movie(movie) => files.push((movie.video_info.path, VideoFileType::Movie)),
        MediaItem::Episode(episode) => {
            files.push((episode.video_info.path, VideoFileType::Episode))
        }
        MediaItem::Show(show) => {
            for episode in db::episodes::get_for_show(&mut conn, show.id).await? {
                files.push((episode.video_info.path, VideoFileType::Episode));
            }
        }
        MediaItem::Season(season) => {
            for episode in db::episodes::get_for_season(&mut conn, season.id).await? {
                files.push((episode.video_info.path, VideoFileType::Episode));
            }
        }
    }

    for (path, video_type) in files {
        tracing::info!("removing file: {path}");
        tokio::fs::remove_file(&path).await?;
        scanner
            .scan_file_path(video_type, &path, &ScanOptions::quick())
            .await?;
    }

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
#[path(name = "id", model = i64)]
#[request(model = VideoUserDataPatch)]
#[response(status = 200)]
async fn update_user_data(
    id: Path<i64>,
    data: Json<VideoUserDataPatch>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let item_type = db::media::get_item_type(&mut conn, *id)
        .await?
        .or_not_found("media item not found")?;

    let mut ids = vec![];

    match item_type {
        MediaItemType::Movie | MediaItemType::Episode => ids.push(*id),
        MediaItemType::Show => ids.extend(
            db::episodes::get_for_show(&mut conn, *id)
                .await?
                .iter()
                .map(|e| e.id),
        ),
        MediaItemType::Season => ids.extend(
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
            set_watched_at: false,
        };

        db::videos::update_user_data(&mut conn, id, data).await?;
    }

    Ok(StatusCode::OK)
}
