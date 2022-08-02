use std::convert::TryFrom;

use axum_codegen::Reflect;
use serde::Serialize;
use sqlx::sqlite::{SqliteArguments, SqliteRow};
use sqlx::Type;
use sqlx::{Arguments, FromRow, Row, SqliteConnection};

use crate::sql;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Type, Serialize)]
pub enum StreamType {
    Video = 1,
    Audio = 2,
}

impl TryFrom<&'_ str> for StreamType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "video" => Ok(StreamType::Video),
            "audio" => Ok(StreamType::Audio),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Reflect)]
pub struct Stream {
    pub id: i64,
    pub index: u32,
    pub codec: String,
    #[serde(flatten)]
    pub props: StreamProps,
}

#[derive(Serialize, Reflect)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum StreamProps {
    Video { width: u32, height: u32 },
    Audio { language: Option<String> },
}

impl Stream {
    pub fn stream_type(&self) -> StreamType {
        match self.props {
            StreamProps::Video { .. } => StreamType::Video,
            StreamProps::Audio { .. } => StreamType::Audio,
        }
    }
}

impl<'r> FromRow<'r, SqliteRow> for Stream {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let index = row.try_get("stream_index")?;
        let codec = row.try_get("codec_name")?;

        let stream_type = row.try_get("stream_type")?;
        let props = match stream_type {
            StreamType::Audio => StreamProps::Audio {
                language: row.try_get("a_language")?,
            },
            StreamType::Video => StreamProps::Video {
                width: row.try_get("v_width")?,
                height: row.try_get("v_height")?,
            },
        };

        let stream = Stream {
            id,
            index,
            codec,
            props,
        };

        Ok(stream)
    }
}

pub async fn get_for_video(
    conn: &mut SqliteConnection,
    video_id: i64,
) -> eyre::Result<Vec<Stream>> {
    let sql = "
        SELECT id, stream_index, stream_type, codec_name, v_width, v_height, a_language
        FROM video_file_streams WHERE video_id = ?
    ";

    Ok(sqlx::query_as(sql).bind(video_id).fetch_all(conn).await?)
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
        .bind(&stream.codec_name)
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
        .bind(&stream.codec_name)
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
