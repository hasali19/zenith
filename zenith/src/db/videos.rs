use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqliteConnection};

use crate::db::streams::{self, StreamType};
use crate::db::subtitles;

use super::streams::Stream;
use super::subtitles::Subtitle;

pub async fn get_all_ids(conn: &mut SqliteConnection) -> eyre::Result<Vec<i64>> {
    sqlx::query_scalar("SELECT item_id FROM video_files")
        .fetch_all(conn)
        .await
        .map_err(Into::into)
}

pub struct BasicVideoInfo {
    pub path: String,
    pub duration: f64,
}

pub async fn get_basic_info(
    conn: &mut SqliteConnection,
    id: i64,
) -> eyre::Result<Option<BasicVideoInfo>> {
    let sql = "
        SELECT path
        FROM video_files
        WHERE item_id = ?
    ";

    let info = sqlx::query(sql)
        .bind(id)
        .try_map(|row: SqliteRow| {
            Ok(BasicVideoInfo {
                path: row.try_get("path")?,
                duration: row.try_get("duration")?,
            })
        })
        .fetch_optional(conn)
        .await?;

    Ok(info)
}

#[derive(Serialize)]
pub struct VideoInfo {
    pub path: String,
    pub duration: f64,
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Option<Stream>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<Stream>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitles: Option<Vec<Subtitle>>,
}

pub async fn get_info(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<VideoInfo>> {
    let sql = "
        SELECT path, duration, format_name
        FROM video_files
        WHERE item_id = ?
    ";

    let info = sqlx::query(sql)
        .bind(id)
        .try_map(|row: SqliteRow| {
            Ok(VideoInfo {
                path: row.try_get("path")?,
                duration: row.try_get("duration")?,
                format: row.try_get("format_name")?,
                video: None,
                audio: None,
                subtitles: None,
            })
        })
        .fetch_optional(&mut *conn)
        .await?;

    let mut info = match info {
        Some(info) => info,
        None => return Ok(None),
    };

    let streams = streams::get_for_video(&mut *conn, id).await?;

    let mut video = None;
    let mut audio = vec![];

    for stream in streams {
        match stream.stream_type() {
            StreamType::Video => video = Some(stream),
            StreamType::Audio => audio.push(stream),
        }
    }

    let subtitles = subtitles::get_for_video(&mut *conn, id).await?;

    info.video = Some(video);
    info.audio = Some(audio);
    info.subtitles = Some(subtitles);

    Ok(Some(info))
}

#[derive(Serialize)]
pub struct VideoUserData {
    pub is_watched: bool,
    pub position: Option<f64>,
}

pub struct UpdateVideo<'a> {
    pub path: Option<&'a str>,
    pub duration: Option<f64>,
    pub format_name: Option<Option<&'a str>>,
}

pub async fn update(
    conn: &mut SqliteConnection,
    id: i64,
    data: UpdateVideo<'_>,
) -> eyre::Result<()> {
    let sql = "
        UPDATE video_files
        SET path = COALESCE(?, path),
            duration = COALESCE(?, duration),
            format_name = COALESCE(?, format_name)
        WHERE item_id = ?
    ";

    sqlx::query(sql)
        .bind(data.path)
        .bind(data.duration)
        .bind(data.format_name)
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}

pub struct UpdateVideoUserData {
    pub is_watched: Option<bool>,
    pub position: Option<f64>,
}

pub async fn update_user_data(
    conn: &mut SqliteConnection,
    id: i64,
    data: UpdateVideoUserData,
) -> eyre::Result<VideoUserData> {
    let sql = "
        INSERT INTO user_item_data (item_id, position, is_watched)
        VALUES (
            ?1,
            MAX(0, MIN(COALESCE(?2, 0), (SELECT duration FROM video_files WHERE item_id = ?1))),
            COALESCE(?3, 0)
        )
        ON CONFLICT (item_id) DO UPDATE
        SET position = MAX(0, MIN(COALESCE(?2, position), (SELECT duration FROM video_files WHERE item_id = ?1))),
            is_watched = COALESCE(?3, is_watched)
        RETURNING CAST(position AS REAL), is_watched
    ";

    let (position, is_watched) = sqlx::query_as(sql)
        .bind(id)
        .bind(data.position)
        .bind(data.is_watched)
        .fetch_one(conn)
        .await?;

    let user_data = VideoUserData {
        position,
        is_watched,
    };

    Ok(user_data)
}
