use std::sync::Arc;

use atium::query::{QueryError, QueryRequestExt};
use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::SqliteConnection;

use crate::api::error::bad_request;
use crate::config::Config;
use crate::db::media::MediaItemType;
use crate::db::subtitles::{Subtitle, SubtitlePath};
use crate::db::{self, Db};
use crate::ffprobe::Ffprobe;

use super::common::{self, Episode, Movie, Season, Show};
use super::ext::OptionExt;

pub fn routes(router: &mut Router) {
    router.route("/items/:id").get(get_item);
    router.route("/items/:id/user_data").patch(update_user_data);
}

#[derive(Default, Deserialize)]
struct GetItemQuery {
    #[serde(default)]
    extended_video_info: bool,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum MediaItem {
    Movie(Movie),
    Show(Show),
    Season(Season),
    Episode(Episode),
}

#[endpoint]
async fn get_item(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;
    let query: GetItemQuery = match req.query() {
        Ok(query) => query,
        Err(QueryError::NotFound) => GetItemQuery::default(),
        Err(e) => return Err(e.into()),
    };

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let item_type = db::media::get_item_type(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    let mut item = get_media_item(&mut conn, id, item_type)
        .await?
        .or_not_found("media item not found")?;

    if query.extended_video_info {
        let item_info = match &mut item {
            MediaItem::Movie(movie) => &mut movie.video_info,
            MediaItem::Episode(episode) => &mut episode.video_info,
            _ => {
                req.ok().json(&item)?;
                return Ok(());
            }
        };

        let config: &Arc<Config> = req.ext().unwrap();
        let info = Ffprobe::new(&config.transcoding.ffprobe_path)
            .probe(&item_info.path)
            .await?;

        let video = info
            .streams
            .iter()
            .find(|stream| stream.codec_type == "video")
            .map(|stream| {
                json!({
                    "codec": stream.codec_name,
                    "profile": stream.properties.get("profile").unwrap().as_str().unwrap(),
                    "width": stream.properties.get("width").and_then(|v| v.as_u64()).unwrap(),
                    "height": stream.properties.get("height").and_then(|v| v.as_u64()).unwrap(),
                })
            });

        let audio = info
            .streams
            .iter()
            .find(|stream| stream.codec_type == "audio")
            .map(|stream| {
                json!({
                    "codec": stream.codec_name,
                })
            });

        let subtitles = db::subtitles::get_for_video(&mut conn, id)
            .await?
            .into_iter()
            .map(subtitle_to_json);

        item_info.extended = Some(json!({
            "format": info.format.format_name,
            "video": video,
            "audio": audio,
            "subtitles": subtitles.collect::<Vec<_>>(),
        }))
    }

    req.ok().json(&item)?;

    Ok(())
}

async fn get_media_item(
    conn: &mut SqliteConnection,
    id: i64,
    item_type: MediaItemType,
) -> eyre::Result<Option<MediaItem>> {
    let item = match item_type {
        MediaItemType::Movie => common::get_movie_item(conn, id)
            .await?
            .map(MediaItem::Movie),
        MediaItemType::TvShow => common::get_show_item(conn, id).await?.map(MediaItem::Show),
        MediaItemType::TvSeason => common::get_season_item(conn, id)
            .await?
            .map(MediaItem::Season),
        MediaItemType::TvEpisode => common::get_episode_item(conn, id)
            .await?
            .map(MediaItem::Episode),
    };

    Ok(item)
}

fn subtitle_to_json(subtitle: Subtitle) -> Value {
    let (subtitle_type, path_key, path_val) = match &subtitle.path {
        SubtitlePath::External(path) => ("external", "path", json!(path.as_ref())),
        SubtitlePath::Embedded(index) => ("embedded", "index", json!(index)),
    };

    json!({
        "id": subtitle.id,
        "title": subtitle.title,
        "language": subtitle.language,
        "type": subtitle_type,
        path_key: path_val,
    })
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

    let duration: f64 = sqlx::query_scalar("SELECT duration FROM video_files WHERE item_id = ?")
        .bind(id)
        .fetch_one(&mut conn)
        .await?;

    let sql = "
        INSERT INTO user_item_data (item_id, position, is_watched)
        VALUES (
            ?1,
            MAX(0, MIN(COALESCE(?2, 0), ?4)),
            COALESCE(?3, 0)
        )
        ON CONFLICT (item_id) DO UPDATE
        SET position = MAX(0, MIN(COALESCE(?2, position), ?4)),
            is_watched = COALESCE(?3, is_watched)
        RETURNING CAST(position AS REAL), is_watched
    ";

    let (position, is_watched): (f64, bool) = sqlx::query_as(sql)
        .bind(id)
        .bind(data.position)
        .bind(data.is_watched)
        .bind(duration)
        .fetch_one(&mut conn)
        .await?;

    req.ok().json(&json!({
        "is_watched": is_watched,
        "position": position,
    }))?;

    Ok(())
}
