use serde::Serialize;
use sqlx::sqlite::SqliteArguments;
use sqlx::Type;
use sqlx::{Arguments, SqliteConnection};

use crate::sql;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Type, Serialize)]
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
    conn: &mut SqliteConnection,
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

pub struct NewAudioStream<'a> {
    pub video_id: i64,
    pub index: u32,
    pub codec_name: &'a str,
    pub language: Option<&'a str>,
}

pub async fn insert_audio_stream(
    conn: &mut SqliteConnection,
    stream: &NewAudioStream<'_>,
) -> eyre::Result<()> {
    let sql = "
        INSERT INTO video_file_streams
            (
                video_id,
                stream_index,
                stream_type,
                codec_name,
                a_language
            )
        VALUES
            (?, ?, ?, ?, ?)
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
        .bind(StreamType::Audio)
        .bind(stream.codec_name)
        .bind(stream.language)
        .execute(conn)
        .await?;

    Ok(())
}

pub async fn remove_except(
    conn: &mut SqliteConnection,
    video_id: i64,
    except: impl Iterator<Item = u32>,
) -> eyre::Result<()> {
    let mut count = 0;
    let mut args = SqliteArguments::default();

    args.add(video_id);

    for index in except {
        args.add(index);
        count += 1;
    }

    let placeholders = sql::Placeholders(count);
    let sql = format!(
        "DELETE FROM video_file_streams
        WHERE video_id = ? AND stream_index NOT IN ({placeholders})"
    );

    sqlx::query_with(&sql, args).execute(conn).await?;

    Ok(())
}
