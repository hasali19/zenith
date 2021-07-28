use sqlx::SqliteConnection;

pub async fn get_path(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<String>> {
    let sql = "
        SELECT path
        FROM video_files
        WHERE item_id = ?
    ";

    let path = sqlx::query_scalar(sql)
        .bind(id)
        .fetch_optional(conn)
        .await?;

    Ok(path)
}
