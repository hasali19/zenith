use std::sync::Arc;

use axum::extract::Path;
use axum::response::{IntoResponse, NoContent};
use axum::{Extension, Json};
use db::items::{MediaItem, VideoUserData};
use db::media::MediaItemType;
use db::videos::UpdateVideoUserData;
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
async fn playback_action(
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

    let progress = body.position / video_file.duration.unwrap();

    update_saved_position(&mut conn, user.id, &item, body.position, progress).await?;

    if let Err(e) =
        post_action_to_trakt(&mut conn, &trakt, user.id, body.action, &item, progress).await
    {
        tracing::error!("failed to send scrobble to trakt: {e:?}");
    };

    conn.commit().await?;

    Ok(NoContent)
}

async fn update_saved_position(
    conn: &mut WriteConnection,
    user_id: i64,
    item: &MediaItem,
    position: f64,
    progress: f64,
) -> eyre::Result<()> {
    let user_data =
        db::items::get_video_user_data_for_item(conn.as_read(), user_id, item.id).await?;

    let is_watched = matches!(
        user_data,
        Some(VideoUserData {
            is_watched: true,
            ..
        }),
    );

    let data = UpdateVideoUserData {
        position: Some(position),
        is_watched: if is_watched {
            None
        } else {
            Some(progress >= 0.9)
        },
        set_position_updated: true,
    };

    db::videos::update_user_data(conn, item.id, user_id, data).await?;

    Ok(())
}

async fn post_action_to_trakt(
    conn: &mut WriteConnection,
    trakt: &TraktClient,
    user_id: i64,
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
                .scrobble_start(user_id, item, progress, video_type)
                .await?;
        }
        PlaybackAction::Pause => {
            trakt
                .scrobble_pause(user_id, item, progress, video_type)
                .await?;
        }
        PlaybackAction::Stop => {
            trakt
                .scrobble_stop(user_id, item, progress, video_type)
                .await?;
        }
        PlaybackAction::Progress => {}
    }

    Ok(())
}
