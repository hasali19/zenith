use serde_json::Value;
use sqlx::SqliteConnection;

use crate::db;
use crate::db::streams::{NewAudioStream, NewVideoStream};
use crate::db::subtitles::{NewSubtitle, SubtitlePath};
use crate::db::videos::UpdateVideo;
use crate::ffprobe::VideoInfo;

pub async fn update_video_info(
    conn: &mut SqliteConnection,
    id: i64,
    info: &VideoInfo,
) -> eyre::Result<()> {
    let data = UpdateVideo {
        duration: info.format.duration.parse()?,
        format_name: Some(info.format.format_name.as_str()),
    };

    db::videos::update(&mut *conn, id, data).await?;

    for stream in &info.streams {
        let tags = stream.properties.get("tags").and_then(|v| v.as_object());
        match stream.codec_type.as_str() {
            "video" => {
                let (width, height) = if let (Some(width), Some(height)) = (
                    stream.properties.get("width").and_then(Value::as_u64),
                    stream.properties.get("height").and_then(Value::as_u64),
                ) {
                    (width as u32, height as u32)
                } else {
                    return Err(eyre::eyre!("missing width and height for video stream"));
                };

                let stream = NewVideoStream {
                    video_id: id,
                    index: stream.index,
                    codec_name: &stream.codec_name,
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
                    codec_name: &stream.codec_name,
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

                let subtitle = NewSubtitle {
                    video_id: id,
                    path: SubtitlePath::Embedded {
                        index: stream.index,
                    },
                    title,
                    language,
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
