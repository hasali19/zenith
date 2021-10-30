use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

pub struct NewSubtitle<'a> {
    pub video_id: i64,
    pub stream_index: Option<u32>,
    pub path: Option<&'a str>,
    pub title: Option<&'a str>,
    pub language: Option<&'a str>,
}

#[derive(Serialize)]
pub struct Subtitle {
    pub id: i64,
    pub video_id: i64,
    pub stream_index: Option<u32>,
    pub path: Option<String>,
    pub title: Option<String>,
    pub language: Option<String>,
}

impl<'r> FromRow<'r, SqliteRow> for Subtitle {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Subtitle {
            id: row.try_get("id")?,
            video_id: row.try_get("video_id")?,
            stream_index: row.try_get("stream_index")?,
            path: row.try_get("path")?,
            title: row.try_get("title")?,
            language: row.try_get("language")?,
        })
    }
}

pub async fn insert(conn: &mut SqliteConnection, subtitle: &NewSubtitle<'_>) -> eyre::Result<i64> {
    let sql = "
        INSERT INTO subtitles
            (video_id, stream_index, path, title, language)
        VALUES
            (?, ?, ?, ?, ?)
        ON CONFLICT
        DO UPDATE SET
            title = excluded.title,
            language = excluded.language
    ";

    let res = sqlx::query(sql)
        .bind(subtitle.video_id)
        .bind(subtitle.stream_index)
        .bind(subtitle.path)
        .bind(subtitle.title)
        .bind(subtitle.language)
        .execute(conn)
        .await?;

    Ok(res.last_insert_rowid())
}

pub async fn get_by_id(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Subtitle>> {
    let sql = "
        SELECT id, video_id, stream_index, path, title, language
        FROM subtitles
        WHERE id = ?
    ";

    sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn get_for_video(
    conn: &mut SqliteConnection,
    video_id: i64,
) -> eyre::Result<Vec<Subtitle>> {
    let sql = "
        SELECT id, video_id, stream_index, path, title, language
        FROM subtitles
        WHERE video_id = ?
    ";

    sqlx::query_as(sql)
        .bind(video_id)
        .fetch_all(conn)
        .await
        .map_err(|e| e.into())
}

pub struct UpdateSubtitle<'a> {
    pub path: Option<&'a str>,
    pub title: Option<&'a str>,
    pub language: Option<&'a str>,
}

pub async fn update_embedded(
    conn: &mut SqliteConnection,
    video_id: i64,
    stream_index: u32,
    data: UpdateSubtitle<'_>,
) -> eyre::Result<()> {
    let sql = "
        UPDATE subtitles
        SET path = COALESCE(?, path),
            title = COALESCE(?, title),
            language = COALESCE(?, language)
        WHERE video_id = ? AND stream_index = ?
    ";

    sqlx::query(sql)
        .bind(data.path)
        .bind(data.title)
        .bind(data.language)
        .bind(video_id)
        .bind(stream_index)
        .execute(conn)
        .await?;

    Ok(())
}

pub async fn delete(conn: &mut SqliteConnection, id: i64) -> eyre::Result<()> {
    sqlx::query("DELETE FROM subtitles WHERE id = ?")
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}
