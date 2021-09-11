use std::sync::Arc;

use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request};
use serde_json::{json, Value};

use crate::config::Config;
use crate::db::media::MediaItemType;
use crate::db::subtitles::{Subtitle, SubtitlePath};
use crate::db::{self, Db};
use crate::ffprobe::Ffprobe;

use super::ext::OptionExt;

pub fn routes(router: &mut Router) {
    router.route("/videos/:id").get(get_video_content);
    router.route("/videos/:id/info").get(get_video_info);
    router.route("/videos/:id/subtitles").get(get_subtitles);
}

#[endpoint]
async fn get_video_content(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT path
        FROM video_files
        WHERE item_id = ?
    ";

    let path: String = sqlx::query_scalar(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .or_not_found("video not found")?;

    req.respond_file(path).await?;

    Ok(())
}

#[endpoint]
async fn get_video_info(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;

    let config: &Arc<Config> = req.ext().unwrap();
    let db: &Db = req.ext().unwrap();

    let mut conn = db.acquire().await?;

    let sql = "
        SELECT file.path, item.item_type, data.position
        FROM video_files AS file
        JOIN media_items AS item ON item.id = file.item_id
        LEFT JOIN user_item_data AS data ON file.item_id = data.item_id
        WHERE file.item_id = ?
    ";

    let (path, item_type, position): (String, MediaItemType, Option<f64>) = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .or_not_found("video not found")?;

    let info = Ffprobe::new(&config.transcoding.ffprobe_path)
        .probe(&path)
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

    req.ok().json(&json!({
        "path": path,
        "type": item_type,
        "format": info.format.format_name,
        "duration": info.format.duration.parse::<f64>().unwrap(),
        "position": position,
        "video": video,
        "audio": audio,
        "subtitles": subtitles.collect::<Vec<_>>(),
    }))?;

    Ok(())
}

#[endpoint]
async fn get_subtitles(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let subtitles: Vec<_> = db::subtitles::get_for_video(&mut conn, id)
        .await?
        .into_iter()
        .map(subtitle_to_json)
        .collect();

    req.ok().json(&subtitles)?;

    Ok(())
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
