use sqlx::FromRow;

use crate::{sql, ReadConnection, WriteConnection};

pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}

pub async fn create(conn: &mut WriteConnection, user: NewUser<'_>) -> eyre::Result<i64> {
    let sql = sql::insert("users")
        .columns(&["username", "password_hash"])
        .values(&["?", "?"])
        .returning(&["id"])
        .to_sql();

    let id = sqlx::query_scalar(&sql)
        .bind(user.username)
        .bind(user.password_hash)
        .fetch_one(conn)
        .await?;

    Ok(id)
}

#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

const SELECT_COLS: &[&str] = &["id", "username", "password_hash"];

pub async fn get_all(conn: &mut ReadConnection) -> eyre::Result<Vec<User>> {
    let sql = sql::select("users").columns(SELECT_COLS).to_sql();
    sqlx::query_as(&sql)
        .fetch_all(conn)
        .await
        .map_err(Into::into)
}

pub async fn get_by_id(conn: &mut ReadConnection, id: i64) -> eyre::Result<Option<User>> {
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
    conn: &mut ReadConnection,
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
