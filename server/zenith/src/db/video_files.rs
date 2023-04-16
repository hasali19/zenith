use camino::{Utf8Path, Utf8PathBuf};
use sqlx::{FromRow, SqliteConnection};

#[derive(FromRow)]
pub struct VideoFile {
    pub id: i64,
    pub item_id: i64,
    pub path: Utf8PathBuf,
    pub path_stem: Utf8PathBuf,
    pub duration: Option<f64>,
    pub format_name: Option<String>,
    pub scanned_at: Option<i64>,
}

pub async fn remove_by_path(conn: &mut SqliteConnection, path: &Utf8Path) -> eyre::Result<()> {
    let sql = "
        DELETE FROM video_file_streams
        WHERE video_id = (SELECT id FROM video_files WHERE path = ?)
    ";

    sqlx::query(sql).bind(path).execute(&mut *conn).await?;

    let sql = "
        DELETE FROM subtitles
        WHERE video_id = (SELECT id FROM video_files WHERE path = ?)
    ";

    sqlx::query(sql).bind(path).execute(&mut *conn).await?;

    sqlx::query("DELETE FROM video_files WHERE path = ?")
        .bind(path)
        .execute(&mut *conn)
        .await?;

    Ok(())
}
