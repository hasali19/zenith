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

pub struct UpdateVideo<'a> {
    pub duration: f64,
    pub format_name: Option<&'a str>,
}

pub async fn update(
    conn: &mut SqliteConnection,
    id: i64,
    data: UpdateVideo<'_>,
) -> eyre::Result<()> {
    let sql = "
        UPDATE video_files
        SET duration = ?,
            format_name = ?
        WHERE item_id = ?
    ";

    sqlx::query(sql)
        .bind(data.duration)
        .bind(data.format_name)
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}
