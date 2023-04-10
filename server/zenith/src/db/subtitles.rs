use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;
use speq::Reflect;
use sqlx::{FromRow, SqliteConnection};

pub struct NewSubtitle<'a> {
    pub video_id: i64,
    pub stream_index: Option<u32>,
    pub path: Option<&'a Utf8Path>,
    pub title: Option<&'a str>,
    pub language: Option<&'a str>,
    pub format: Option<&'a str>,
    pub sdh: bool,
    pub forced: bool,
}

#[derive(Serialize, Reflect, FromRow)]
pub struct Subtitle {
    pub id: i64,
    pub video_id: i64,
    pub stream_index: Option<u32>,
    pub path: Option<Utf8PathBuf>,
    pub title: Option<String>,
    pub language: Option<String>,
    pub format: Option<String>,
    pub sdh: bool,
    pub forced: bool,
}

pub async fn insert(conn: &mut SqliteConnection, subtitle: &NewSubtitle<'_>) -> eyre::Result<i64> {
    let sql = "
        INSERT INTO subtitles
            (video_id, stream_index, path, title, language, format, sdh, forced)
        VALUES
            (?, ?, ?, ?, ?, ?, ?, ?)
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
        .bind(subtitle.format)
        .bind(subtitle.sdh)
        .bind(subtitle.forced)
        .execute(conn)
        .await?;

    Ok(res.last_insert_rowid())
}

pub async fn get_by_id(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Subtitle>> {
    let sql = "
        SELECT id, video_id, stream_index, path, title, language, format, sdh, forced
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
        SELECT id, video_id, stream_index, path, title, language, format, sdh, forced
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
    pub path: Option<&'a Utf8Path>,
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
