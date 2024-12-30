use std::time::{Duration, SystemTime};

use sqlx::{FromRow, SqliteConnection};

pub struct NewUserRegistration<'a> {
    pub id: &'a str,
    pub duration: Duration,
}

pub async fn create(
    conn: &mut SqliteConnection,
    data: NewUserRegistration<'_>,
) -> eyre::Result<UserRegistration> {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let expires_at = now + data.duration;

    let sql = "
        INSERT INTO user_registrations (id, created_at, expires_at)
        VALUES (?, ?, ?)
        RETURNING id, created_at, expires_at
    ";

    let res = sqlx::query_as(sql)
        .bind(data.id)
        .bind(now.as_secs() as i64)
        .bind(expires_at.as_secs() as i64)
        .fetch_one(conn)
        .await?;

    Ok(res)
}

#[derive(FromRow)]
pub struct UserRegistration {
    pub id: String,
    pub created_at: i64,
    pub expires_at: i64,
}

pub async fn get(conn: &mut SqliteConnection, id: &str) -> eyre::Result<Option<UserRegistration>> {
    let registration = sqlx::query_as("SELECT * FROM user_registrations WHERE id = ?")
        .bind(id)
        .fetch_optional(conn)
        .await?;
    Ok(registration)
}

pub async fn get_all(conn: &mut SqliteConnection) -> eyre::Result<Vec<UserRegistration>> {
    let registrations = sqlx::query_as("SELECT * FROM user_registrations")
        .fetch_all(conn)
        .await?;
    Ok(registrations)
}

pub async fn delete(conn: &mut SqliteConnection, id: &str) -> eyre::Result<bool> {
    let res = sqlx::query("DELETE FROM user_registrations WHERE id = ?")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(res.rows_affected() > 0)
}
