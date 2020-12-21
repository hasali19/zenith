use sqlx::sqlite::{SqliteDone, SqliteRow};
use sqlx::{Row, Sqlite, SqliteConnection, Transaction};

pub async fn get_all_ids(conn: &mut SqliteConnection) -> sqlx::Result<Vec<i64>> {
    sqlx::query("SELECT id FROM tv_shows")
        .try_map(|row: SqliteRow| row.try_get(0))
        .fetch_all(conn)
        .await
}

pub async fn get_id_for_path(conn: &mut SqliteConnection, path: &str) -> sqlx::Result<Option<i64>> {
    sqlx::query("SELECT id FROM tv_shows WHERE path = ?")
        .bind(path)
        .try_map(|row: SqliteRow| row.try_get(0))
        .fetch_optional(conn)
        .await
}

pub struct NewTvShow<'a> {
    pub path: &'a str,
    pub name: &'a str,
    pub overview: Option<&'a str>,
    pub poster_url: Option<&'a str>,
    pub backdrop_url: Option<&'a str>,
    pub tmdb_id: Option<i32>,
}

pub async fn create(conn: &mut SqliteConnection, data: &NewTvShow<'_>) -> sqlx::Result<i64> {
    let sql = "
        INSERT INTO tv_shows (path, name, overview, poster_url, backdrop_url, tmdb_id)
        VALUES (?, ?, ?, ?, ?, ?)
    ";

    let res: SqliteDone = sqlx::query(sql)
        .bind(data.path)
        .bind(data.name)
        .bind(data.overview)
        .bind(data.poster_url)
        .bind(data.backdrop_url)
        .bind(data.tmdb_id)
        .execute(conn)
        .await?;

    Ok(res.last_insert_rowid())
}

pub async fn delete(transaction: &mut Transaction<'_, Sqlite>, id: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM tv_episodes WHERE show_id = ?")
        .bind(id)
        .execute(&mut *transaction)
        .await?;

    sqlx::query("DELETE FROM tv_shows WHERE id = ?")
        .bind(id)
        .execute(&mut *transaction)
        .await?;

    Ok(())
}

pub async fn get_episode_ids(conn: &mut SqliteConnection, show_id: i64) -> sqlx::Result<Vec<i64>> {
    sqlx::query("SELECT id FROM tv_episodes WHERE show_id = ?")
        .bind(show_id)
        .try_map(|row: SqliteRow| row.try_get(0))
        .fetch_all(conn)
        .await
}

pub async fn get_episode_id_for_number(
    conn: &mut SqliteConnection,
    show_id: i64,
    season: u32,
    episode: u32,
) -> sqlx::Result<Option<i64>> {
    sqlx::query("SELECT id FROM tv_episodes WHERE show_id = ? AND season = ? AND episode = ?")
        .bind(show_id)
        .bind(season as i64)
        .bind(episode as i64)
        .try_map(|row: SqliteRow| row.try_get(0))
        .fetch_optional(conn)
        .await
}

pub struct NewTvEpisode<'a> {
    pub season: u32,
    pub episode: u32,
    pub overview: Option<&'a str>,
    pub image_url: Option<&'a str>,
    pub tmdb_id: Option<i32>,
    pub video_path: &'a str,
}

pub async fn create_episode(
    conn: &mut SqliteConnection,
    show_id: i64,
    data: &NewTvEpisode<'_>,
) -> sqlx::Result<i64> {
    let sql = "
        INSERT INTO tv_episodes (show_id, season, episode, overview, image_url, tmdb_id, video_path)
        VALUES (?, ?, ?, ?, ?, ?, ?)
    ";

    let res: SqliteDone = sqlx::query(sql)
        .bind(show_id)
        .bind(data.season as i64)
        .bind(data.episode as i64)
        .bind(data.overview)
        .bind(data.image_url)
        .bind(data.tmdb_id)
        .bind(data.video_path)
        .execute(conn)
        .await?;

    Ok(res.last_insert_rowid())
}

pub async fn delete_episode(conn: &mut SqliteConnection, id: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM tv_episodes WHERE id = ?")
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}
