use sqlx::Type;

use crate::utils::arguments::QueryArguments;
use crate::{WriteConnection, sql};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Type)]
pub enum StreamType {
    Video,
    Audio,
}

pub struct NewVideoStream<'a> {
    pub video_id: i64,
    pub index: u32,
    pub codec_name: &'a str,
    pub width: u32,
    pub height: u32,
}

pub async fn insert_video_stream(
    conn: &mut WriteConnection,
    stream: &NewVideoStream<'_>,
) -> eyre::Result<()> {
    let sql = "
        INSERT INTO video_file_streams
            (
                video_id,
                stream_index,
                stream_type,
                codec_name,
                v_width,
                v_height
            )
        VALUES
            (?, ?, ?, ?, ?, ?)
        ON CONFLICT (video_id, stream_index)
        DO UPDATE SET
            stream_type = excluded.stream_type,
            codec_name = excluded.codec_name,
            v_width = excluded.v_width,
            v_height = excluded.v_height
    ";

    sqlx::query(sql)
        .bind(stream.video_id)
        .bind(stream.index)
        .bind(StreamType::Video)
        .bind(stream.codec_name)
        .bind(stream.width)
        .bind(stream.height)
        .execute(conn)
        .await?;

    Ok(())
}

pub struct UpdateVideoStream {
    pub crop_x1: u32,
    pub crop_x2: u32,
    pub crop_y1: u32,
    pub crop_y2: u32,
}

pub async fn update_video_stream_by_index(
    conn: &mut WriteConnection,
    video_id: i64,
    stream_index: u32,
    stream: &UpdateVideoStream,
) -> eyre::Result<()> {
    let sql = sql::update("video_file_streams")
        .columns(&["v_crop_x1", "v_crop_x2", "v_crop_y1", "v_crop_y2"])
        .values(&["?", "?", "?", "?"])
        .condition("video_id = ? AND stream_index = ?")
        .to_sql();

    sqlx::query(&sql)
        .bind(stream.crop_x1)
        .bind(stream.crop_x2)
        .bind(stream.crop_y1)
        .bind(stream.crop_y2)
        .bind(video_id)
        .bind(stream_index)
        .execute(conn)
        .await?;

    Ok(())
}

pub struct NewAudioStream<'a> {
    pub video_id: i64,
    pub index: u32,
    pub codec_name: &'a str,
    pub language: Option<&'a str>,
    pub channels: Option<u32>,
    pub channel_layout: Option<&'a str>,
}

pub async fn insert_audio_stream(
    conn: &mut WriteConnection,
    stream: &NewAudioStream<'_>,
) -> eyre::Result<()> {
    let sql = "
        INSERT INTO video_file_streams
            (
                video_id,
                stream_index,
                stream_type,
                codec_name,
                a_language,
                a_channels,
                a_channel_layout
            )
        VALUES
            (?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT (video_id, stream_index)
        DO UPDATE SET
            stream_type = excluded.stream_type,
            codec_name = excluded.codec_name,
            a_language = excluded.a_language,
            a_channels = excluded.a_channels,
            a_channel_layout = excluded.a_channel_layout
    ";

    sqlx::query(sql)
        .bind(stream.video_id)
        .bind(stream.index)
        .bind(StreamType::Audio)
        .bind(stream.codec_name)
        .bind(stream.language)
        .bind(stream.channels)
        .bind(stream.channel_layout)
        .execute(conn)
        .await?;

    Ok(())
}

pub async fn remove_except(
    conn: &mut WriteConnection,
    video_id: i64,
    except: impl Iterator<Item = u32>,
) -> eyre::Result<()> {
    let mut count = 0;
    let mut args = QueryArguments::default();

    args.add(video_id)?;

    for index in except {
        args.add(index)?;
        count += 1;
    }

    let placeholders = sql::Placeholders(count);
    let sql = format!(
        "DELETE FROM video_file_streams
        WHERE video_id = ? AND stream_index NOT IN ({placeholders})"
    );

    sqlx::query_with(&sql, args.into_inner())
        .execute(conn)
        .await?;

    Ok(())
}
