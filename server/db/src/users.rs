use sqlx::{FromRow, SqliteConnection};

use crate::sql;

pub struct NewUser<'a> {
    pub username: &'a str,
}

pub async fn create(conn: &mut SqliteConnection, user: NewUser<'_>) -> eyre::Result<i64> {
    let sql = sql::insert("users")
        .columns(&["username"])
        .values(&["?"])
        .returning(&["id"])
        .to_sql();

    let id = sqlx::query_scalar(&sql)
        .bind(user.username)
        .fetch_one(conn)
        .await?;

    Ok(id)
}

#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
}

const SELECT_COLS: &[&str] = &["id", "username"];

pub async fn get_all(conn: &mut SqliteConnection) -> eyre::Result<Vec<User>> {
    let sql = sql::select("users").columns(SELECT_COLS).to_sql();
    sqlx::query_as(&sql)
        .fetch_all(conn)
        .await
        .map_err(Into::into)
}

pub async fn get_by_id(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<User>> {
    let sql = sql::select("users")
        .columns(SELECT_COLS)
        .condition("id = ?")
        .to_sql();

    sqlx::query_as(&sql)
        .bind(id)
        .fetch_optional(conn)
        .await
        .map_err(Into::into)
}

pub async fn get_by_username(
    conn: &mut SqliteConnection,
    username: &str,
) -> eyre::Result<Option<User>> {
    let sql = sql::select("users")
        .columns(SELECT_COLS)
        .condition("username = ?")
        .to_sql();

    let user = sqlx::query_as(&sql)
        .bind(username)
        .fetch_optional(conn)
        .await?;

    Ok(user)
}
