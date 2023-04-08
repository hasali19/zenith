use eyre::eyre;
use serde_json::Value;
use sqlx::SqliteConnection;

use crate::db;
use crate::db::streams::{NewAudioStream, NewVideoStream};
use crate::db::subtitles::NewSubtitle;
use crate::db::videos::UpdateVideo;
use crate::video_prober::VideoInfo;

pub async fn update_video_info(
    conn: &mut SqliteConnection,
    id: i64,
    info: &VideoInfo,
) -> eyre::Result<()> {
    let data = UpdateVideo {
        path: None,
        duration: Some(info.format.duration.parse()?),
        format_name: Some(Some(info.format.format_name.as_str())),
    };

    db::videos::update(&mut *conn, id, data).await?;

    for stream in &info.streams {
        let tags = stream.properties.get("tags").and_then(|v| v.as_object());

        let Some(codec_name) = stream.codec_name.as_deref() else {
            tracing::debug!(stream.index, stream.codec_type, "stream is missing codec name");
            continue;
        };

        match stream.codec_type.as_str() {
            "video" => {
                let (width, height) = if let (Some(width), Some(height)) = (
                    stream.properties.get("width").and_then(Value::as_u64),
                    stream.properties.get("height").and_then(Value::as_u64),
                ) {
                    (width as u32, height as u32)
                } else {
                    return Err(eyre!("missing width and height for video stream"));
                };

                let stream = NewVideoStream {
                    video_id: id,
                    index: stream.index,
                    codec_name,
                    width,
                    height,
                };

                db::streams::insert_video_stream(&mut *conn, &stream).await?;
            }
            "audio" => {
                let language = tags
                    .and_then(|tags| tags.get("language"))
                    .and_then(|v| v.as_str());

                let stream = NewAudioStream {
                    video_id: id,
                    index: stream.index,
                    codec_name,
                    language,
                };

                db::streams::insert_audio_stream(&mut *conn, &stream).await?;
            }
            "subtitle" => {
                let title = tags
                    .and_then(|tags| tags.get("title"))
                    .and_then(|v| v.as_str());

                let language = tags
                    .and_then(|tags| tags.get("language"))
                    .and_then(|v| v.as_str());

                let get_disposition_bool = |name| {
                    stream
                        .properties
                        .get("disposition")
                        .and_then(|v| v.as_object())
                        .and_then(|v| v.get(name))
                        .and_then(|v| v.as_i64())
                        .map(|v| v == 1)
                        .unwrap_or(false)
                };

                let subtitle = NewSubtitle {
                    video_id: id,
                    stream_index: Some(stream.index),
                    path: None,
                    title,
                    language,
                    format: Some(codec_name),
                    sdh: get_disposition_bool("hearing_impaired"),
                    forced: get_disposition_bool("forced"),
                };

                db::subtitles::insert(&mut *conn, &subtitle).await?;
            }
            _ => {}
        }
    }

    // Remove streams which no longer exist in the file
    db::streams::remove_except(&mut *conn, id, info.streams.iter().map(|s| s.index)).await?;

    Ok(())
}
