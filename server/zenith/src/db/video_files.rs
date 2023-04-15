use camino::Utf8Path;
use sqlx::SqliteConnection;

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
