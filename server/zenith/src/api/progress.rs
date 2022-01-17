use axum::extract::{Extension, Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_codegen::post;
use serde::Deserialize;

use crate::api::ApiResult;
use crate::db::videos::UpdateVideoUserData;
use crate::db::{self, Db};

use super::error::bad_request;
use super::ext::OptionExt;

#[derive(Deserialize)]
struct ProgressUpdate {
    position: f64,
}

#[post("/progress/:id")]
#[path(name = "id", model = i64)]
#[query(name = "position", model = f64)]
#[response(status = 200)]
async fn update_progress(
    id: Path<i64>,
    query: Query<ProgressUpdate>,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let item = db::items::get(&mut conn, *id)
        .await?
        .or_not_found("item not found")?;

    let (video_info, user_data) = match item {
        db::items::MediaItem::Movie(m) => (m.video_info, m.user_data),
        db::items::MediaItem::Episode(e) => (e.video_info, e.user_data),
        _ => return Err(bad_request("item id must refer to a movie or tv episode")),
    };

    let data = UpdateVideoUserData {
        position: Some(query.position),
        is_watched: if user_data.is_watched {
            None
        } else {
            Some((query.position / video_info.duration) >= 0.9)
        },
        set_watched_at: true,
    };

    db::videos::update_user_data(&mut conn, *id, data).await?;

    Ok(StatusCode::OK)
}
