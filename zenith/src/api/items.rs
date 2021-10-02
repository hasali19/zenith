use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::SqliteConnection;

use crate::api::error::bad_request;
use crate::db::media::{MediaItemType, VideoFileStreamType};
use crate::db::subtitles::{Subtitle, SubtitlePath};
use crate::db::{self, Db};

use super::common::{self, Episode, Movie, Season, Show};
use super::ext::OptionExt;

pub fn routes(router: &mut Router) {
    router.route("/items/:id").get(get_item);
    router.route("/items/:id/user_data").patch(update_user_data);
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
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let item_type = db::media::get_item_type(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    let mut item = get_media_item(&mut conn, id, item_type)
        .await?
        .or_not_found("media item not found")?;

    let item_info = match &mut item {
        MediaItem::Movie(movie) => &mut movie.video_info,
        MediaItem::Episode(episode) => &mut episode.video_info,
        _ => {
            req.ok().json(&item)?;
            return Ok(());
        }
    };

    let sql = "
        SELECT id, stream_index, stream_type, codec_name, v_width, v_height, a_language
        FROM video_file_streams WHERE video_id = ?
    ";

    #[derive(sqlx::FromRow)]
    struct StreamRow {
        id: i64,
        stream_index: u32,
        stream_type: VideoFileStreamType,
        codec_name: String,
        v_width: Option<u32>,
        v_height: Option<u32>,
        a_language: Option<String>,
    }

    let streams: Vec<StreamRow> = sqlx::query_as(sql).bind(id).fetch_all(&mut conn).await?;

    let video = streams
        .iter()
        .find(|stream| stream.stream_type == VideoFileStreamType::Video)
        .map(|stream| {
            json!({
                "id": stream.id,
                "index": stream.stream_index,
                "codec": stream.codec_name,
                "width": stream.v_width,
                "height": stream.v_height,
            })
        });

    let audio = streams
        .iter()
        .filter(|stream| stream.stream_type == VideoFileStreamType::Audio)
        .map(|stream| {
            json!({
                "id": stream.id,
                "index": stream.stream_index,
                "codec": stream.codec_name,
                "language": stream.a_language,
            })
        });

    let subtitles = db::subtitles::get_for_video(&mut conn, id)
        .await?
        .into_iter()
        .map(subtitle_to_json);

    item_info.extended = Some(json!({
        "video": video,
        "audio": audio.collect::<Vec<_>>(),
        "subtitles": subtitles.collect::<Vec<_>>(),
    }));

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
