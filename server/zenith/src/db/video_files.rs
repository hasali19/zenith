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

pub async fn get(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<VideoFile>> {
    sqlx::query_as("SELECT * FROM video_files WHERE id = ?")
        .bind(id)
        .fetch_optional(conn)
        .await
        .map_err(Into::into)
}

pub async fn get_for_item(
    conn: &mut SqliteConnection,
    item_id: i64,
) -> eyre::Result<Vec<VideoFile>> {
    sqlx::query_as("SELECT * FROM video_files WHERE item_id = ?")
        .bind(item_id)
        .fetch_all(conn)
        .await
        .map_err(Into::into)
}

/// Retrieves all videos associated with the item or its children/grandchildren.
pub async fn get_recursive_for_item(
    conn: &mut SqliteConnection,
    item_id: i64,
) -> eyre::Result<Vec<VideoFile>> {
    let sql = "
        SELECT * FROM video_files
        WHERE item_id IN (
            SELECT id FROM media_items WHERE id = ?1
                OR parent_id = ?1
                OR grandparent_id = ?1
        )
    ";

    sqlx::query_as(sql)
        .bind(item_id)
        .fetch_all(conn)
        .await
        .map_err(Into::into)
}

pub struct UpdateVideoFile<'a> {
    pub path: Option<&'a Utf8Path>,
    pub duration: Option<f64>,
    pub format_name: Option<Option<&'a str>>,
    pub set_scanned_at: bool,
}

pub async fn update(
    conn: &mut SqliteConnection,
    id: i64,
    data: UpdateVideoFile<'_>,
) -> eyre::Result<()> {
    let sql = "
        UPDATE video_files
        SET path = COALESCE(?, path),
            duration = COALESCE(?, duration),
            format_name = COALESCE(?, format_name),
            scanned_at = IIF(?, strftime('%s'), scanned_at)
        WHERE id = ?
    ";

    sqlx::query(sql)
        .bind(data.path)
        .bind(data.duration)
        .bind(data.format_name)
        .bind(data.set_scanned_at)
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
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
