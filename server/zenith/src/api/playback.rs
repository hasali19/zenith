use std::sync::Arc;

use axum::extract::Path;
use axum::response::{IntoResponse, NoContent};
use axum::{Extension, Json};
use db::items::MediaItem;
use db::media::MediaItemType;
use db::{Db, WriteConnection};
use serde::Deserialize;
use speq::Reflect;
use speq::axum::post;

use crate::trakt::{TraktClient, TraktService, VideoType};

use super::ext::OptionExt;
use super::{ApiResult, auth};

#[derive(Clone, Copy, Deserialize, Reflect)]
#[serde(rename_all = "snake_case")]
enum PlaybackAction {
    Start,
    Pause,
    Resume,
    Stop,
    Progress,
}

#[derive(Deserialize, Reflect)]
struct PlaybackUpdate {
    action: PlaybackAction,
    position: f64,
}

#[post("/playback/{video_id}")]
async fn start(
    Path(video_id): Path<i64>,
    db: Extension<Db>,
    trakt: Extension<Arc<TraktClient>>,
    user: auth::User,
    body: Json<PlaybackUpdate>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.begin_write().await?;

    let video_file = db::video_files::get(conn.as_read(), video_id)
        .await?
        .or_not_found("video not found")?;

    let item = db::items::get(conn.as_read(), video_file.item_id)
        .await?
        .or_not_found("item not found")?;

    let Some(tmdb_id) = item.tmdb_id else {
        return Ok(NoContent);
    };

    let progress = body.position / video_file.duration.unwrap();

    if let Err(e) = post_action_to_trakt(
        &mut conn,
        &trakt,
        user.id,
        tmdb_id,
        body.action,
        &item,
        progress,
    )
    .await
    {
        tracing::error!("failed to send scrobble to trakt: {e:?}");
    };

    conn.commit().await?;

    Ok(NoContent)
}

async fn post_action_to_trakt(
    conn: &mut WriteConnection,
    trakt: &TraktClient,
    user_id: i64,
    tmdb_id: i32,
    action: PlaybackAction,
    item: &MediaItem,
    progress: f64,
) -> eyre::Result<()> {
    let mut trakt = TraktService::new(trakt, conn);

    let video_type = match item.kind {
        MediaItemType::Movie => VideoType::Movie,
        MediaItemType::Episode => VideoType::Episode,
        _ => unreachable!(),
    };

    match action {
        PlaybackAction::Start | PlaybackAction::Resume => {
            trakt
                .scrobble_start(user_id, tmdb_id, progress, video_type)
                .await?;
        }
        PlaybackAction::Pause => {
            trakt
                .scrobble_pause(user_id, tmdb_id, progress, video_type)
                .await?;
        }
        PlaybackAction::Stop => {
            trakt
                .scrobble_stop(user_id, tmdb_id, progress, video_type)
                .await?;
        }
        PlaybackAction::Progress => {}
    }

    Ok(())
}
