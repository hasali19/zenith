use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use db::items::{MediaItem, VideoUserData};
use db::videos::UpdateVideoUserData;
use db::{Db, WriteConnection};
use serde::Deserialize;
use serde_qs::axum::QsQuery;
use speq::Reflect;
use speq::axum::post;

use crate::api::ApiResult;
use crate::trakt::{TraktClient, TraktService};

use super::auth;
use super::error::bad_request;
use super::ext::OptionExt;

#[derive(Deserialize, Reflect)]
struct ProgressUpdate {
    position: f64,
}

#[post("/progress/{id}")]
#[response(status = 200)]
async fn update_progress(
    id: Path<i64>,
    query: QsQuery<ProgressUpdate>,
    user: auth::User,
    db: Extension<Db>,
    trakt: Extension<Arc<TraktClient>>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.begin_write().await?;

    let item = db::items::get(conn.as_read(), *id)
        .await?
        .or_not_found("item not found")?;

    if !item.kind.is_video() {
        return Err(bad_request("item id must refer to a video item"));
    }

    let user_data = db::items::get_video_user_data_for_item(conn.as_read(), user.id, *id).await?;
    let video_files = db::video_files::get_for_item(conn.as_read(), *id).await?;

    let Some(video_file) = video_files.first() else {
        return Err(bad_request("no associated video files found"));
    };

    let prev_is_watched = matches!(
        user_data,
        Some(VideoUserData {
            is_watched: true,
            ..
        }),
    );

    let progress = query.position / video_file.duration.unwrap();
    let is_watched = progress >= 0.9;

    let data = UpdateVideoUserData {
        position: Some(query.position),
        is_watched: if prev_is_watched {
            None
        } else {
            Some(is_watched)
        },
        set_position_updated: true,
    };

    db::videos::update_user_data(&mut conn, *id, user.id, data).await?;

    if !prev_is_watched
        && is_watched
        && let Err(e) = sync_watch_to_trakt(&mut conn, &trakt, user.id, &item).await
    {
        tracing::error!("Failed to sync watched item to trakt: {e:?}");
    }

    conn.commit().await?;

    Ok(StatusCode::OK)
}

async fn sync_watch_to_trakt(
    conn: &mut WriteConnection,
    trakt: &TraktClient,
    user_id: i64,
    item: &MediaItem,
) -> eyre::Result<()> {
    tracing::debug!(item.id, "syncing item to trakt watch history");

    let synced = TraktService::new(trakt, conn)
        .add_to_watch_history(user_id, item)
        .await?;

    if synced {
        tracing::trace!(item.id, "synced item to trakt watch history");
    }

    Ok(())
}
