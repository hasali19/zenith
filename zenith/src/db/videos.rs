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

pub struct UpdateVideo {
    pub duration: f64,
}

pub async fn update(conn: &mut SqliteConnection, id: i64, data: UpdateVideo) -> eyre::Result<()> {
    let sql = "
        UPDATE video_files
        SET duration = ?
        WHERE item_id = ?
    ";

    sqlx::query(sql)
        .bind(data.duration)
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}
