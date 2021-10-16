use std::borrow::Cow;

use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqliteConnection};

pub struct NewSubtitle<'a> {
    pub video_id: i64,
    pub path: SubtitlePath<'a>,
    pub title: Option<&'a str>,
    pub language: Option<&'a str>,
}

#[derive(Serialize)]
pub struct Subtitle {
    pub id: i64,
    pub video_id: i64,
    #[serde(flatten)]
    pub path: SubtitlePath<'static>,
    pub title: Option<String>,
    pub language: Option<String>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SubtitlePath<'a> {
    External { path: Cow<'a, str> },
    Embedded { index: u32 },
}

impl<'r> FromRow<'r, SqliteRow> for Subtitle {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let path: &str = row.try_get(2)?;
        let path = if let Some(index) = path.strip_prefix("embedded://") {
            SubtitlePath::Embedded {
                index: index.parse().expect("invalid index for embedded subtitle"),
            }
        } else {
            SubtitlePath::External {
                path: Cow::Owned(path.to_owned()),
            }
        };

        Ok(Subtitle {
            id: row.try_get(0)?,
            video_id: row.try_get(1)?,
            path,
            title: row.try_get(3)?,
            language: row.try_get(4)?,
        })
    }
}

pub async fn insert(conn: &mut SqliteConnection, subtitle: &NewSubtitle<'_>) -> eyre::Result<i64> {
    let sql = "
        INSERT INTO subtitles
            (video_id, path, title, language)
        VALUES
            (?, ?, ?, ?)
        ON CONFLICT (video_id, path)
        DO UPDATE SET
            title = excluded.title,
            language = excluded.language
    ";

    let query = sqlx::query(sql).bind(subtitle.video_id);
    let query = match &subtitle.path {
        SubtitlePath::External { path } => query.bind(path.as_ref()),
        SubtitlePath::Embedded { index } => query.bind(format!("embedded://{}", index)),
    };

    let res = query
        .bind(subtitle.title)
        .bind(subtitle.language)
        .execute(conn)
        .await?;

    Ok(res.last_insert_rowid())
}

pub async fn get_by_id(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Subtitle>> {
    let sql = "
        SELECT id, video_id, path, title, language
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
        SELECT id, video_id, path, title, language
        FROM subtitles
        WHERE video_id = ?
    ";

    sqlx::query_as(sql)
        .bind(video_id)
        .fetch_all(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn delete(conn: &mut SqliteConnection, id: i64) -> eyre::Result<()> {
    sqlx::query("DELETE FROM subtitles WHERE id = ?")
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}
