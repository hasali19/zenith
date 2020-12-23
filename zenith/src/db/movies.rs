use sqlx::sqlite::{SqliteDone, SqliteRow};
use sqlx::{Row, SqliteConnection};

/// Finds the id of a movie with the specified path.
pub async fn get_id_for_path(conn: &mut SqliteConnection, path: &str) -> sqlx::Result<Option<i64>> {
    sqlx::query("SELECT id FROM movies WHERE path = ?")
        .bind(path)
        .try_map(|row: SqliteRow| row.try_get(0))
        .fetch_optional(conn)
        .await
}

pub struct NewMovie<'a> {
    pub path: &'a str,
    pub title: &'a str,
    pub year: Option<i32>,
    pub video_path: &'a str,
}

/// Creates a new movie record.
pub async fn create(conn: &mut SqliteConnection, data: &NewMovie<'_>) -> sqlx::Result<i64> {
    let sql = "
        INSERT INTO movies (path, title, year, video_path)
        VALUES (?, ?, ?, ?)
    ";

    let res: SqliteDone = sqlx::query(sql)
        .bind(data.path)
        .bind(data.title)
        .bind(data.year)
        .bind(data.video_path)
        .execute(conn)
        .await?;

    Ok(res.last_insert_rowid())
}
