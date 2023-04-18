use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use db::items::VideoUserData;
use db::videos::UpdateVideoUserData;
use db::Db;
use serde::Deserialize;
use serde_qs::axum::QsQuery;
use speq::axum::post;
use speq::Reflect;

use crate::api::ApiResult;

use super::error::bad_request;
use super::ext::OptionExt;

#[derive(Deserialize, Reflect)]
struct ProgressUpdate {
    position: f64,
}

#[post("/progress/:id")]
#[path(i64)]
#[response(status = 200)]
async fn update_progress(
    id: Path<i64>,
    #[query] query: QsQuery<ProgressUpdate>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let item = db::items::get(&mut conn, *id)
        .await?
        .or_not_found("item not found")?;

    if !item.kind.is_video() {
        return Err(bad_request("item id must refer to a video item"));
    }

    let user_data = db::items::get_user_data_for_video(&mut conn, *id).await?;
    let video_file = db::video_files::get_for_item(&mut conn, *id).await?;

    let Some(video_file) = video_file.get(0) else {
        return Err(bad_request("no associated video files found"));
    };

    let is_watched = matches!(
        user_data,
        Some(VideoUserData {
            is_watched: true,
            ..
        }),
    );

    let data = UpdateVideoUserData {
        position: Some(query.position),
        is_watched: if is_watched {
            None
        } else {
            Some((query.position / video_file.duration.unwrap()) >= 0.9)
        },
        set_watched_at: true,
    };

    db::videos::update_user_data(&mut conn, *id, data).await?;

    Ok(StatusCode::OK)
}
