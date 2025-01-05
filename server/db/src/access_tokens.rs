use std::time::{Duration, SystemTime};

use sqlx::FromRow;

use crate::{sql, ReadConnection, WriteConnection};

#[derive(Clone, Copy, sqlx::Type)]
#[repr(i32)]
pub enum AccessTokenOwner {
    System = 1,
    User = 2,
}

pub struct NewAccessToken<'a> {
    pub owner: AccessTokenOwner,
    pub name: &'a str,
    pub user_id: i64,
    pub token: &'a str,
    pub duration: Option<Duration>,
}

pub async fn create(
    conn: &mut WriteConnection,
    data: NewAccessToken<'_>,
) -> eyre::Result<AccessToken> {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let expires_at = data.duration.map(|duration| now + duration);

    let sql = "
        INSERT INTO user_access_tokens (owner, name, user_id, token, created_at, expires_at)
        VALUES (?, ?, ?, ?, ?, ?)
    ";

    sqlx::query(sql)
        .bind(data.owner)
        .bind(data.name)
        .bind(data.user_id)
        .bind(data.token)
        .bind(now.as_secs() as i64)
        .bind(expires_at.map(|expires_at| expires_at.as_secs() as i64))
        .execute(conn)
        .await?;

    Ok(AccessToken {
        owner: data.owner,
        name: data.name.to_owned(),
        token: data.token.to_owned(),
        user_id: data.user_id,
        created_at: now.as_secs() as i64,
        expires_at: expires_at.map(|expires_at| expires_at.as_secs() as i64),
    })
}

#[derive(FromRow)]
pub struct AccessToken {
    pub owner: AccessTokenOwner,
    pub name: String,
    pub user_id: i64,
    pub token: String,
    pub created_at: i64,
    pub expires_at: Option<i64>,
}

pub async fn get_named(
    conn: &mut ReadConnection,
    user_id: i64,
    owner: AccessTokenOwner,
    name: &str,
) -> eyre::Result<Option<AccessToken>> {
    let sql = sql::select("user_access_tokens")
        .columns(&["*"])
        .condition("user_id = ? AND owner = ? AND name = ?")
        .to_sql();

    let token = sqlx::query_as(&sql)
        .bind(user_id)
        .bind(owner)
        .bind(name)
        .fetch_optional(conn)
        .await?;

    Ok(token)
}

pub async fn get(conn: &mut ReadConnection, token: &str) -> eyre::Result<Option<AccessToken>> {
    let token = sqlx::query_as("SELECT * FROM user_access_tokens WHERE token = ?")
        .bind(token)
        .fetch_optional(conn)
        .await?;
    Ok(token)
}

pub async fn delete(conn: &mut WriteConnection, token: &str) -> eyre::Result<()> {
    sqlx::query("DELETE FROM user_access_tokens WHERE token = ?")
        .bind(token)
        .execute(conn)
        .await?;
    Ok(())
}
