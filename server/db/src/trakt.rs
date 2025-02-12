use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use time::format_description::well_known::Iso8601;

use crate::{ReadConnection, WriteConnection, sql};

#[derive(FromRow)]
pub struct TraktUser {
    pub user_id: i64,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<String>,
}

pub async fn get_user(conn: &mut ReadConnection, user_id: i64) -> eyre::Result<Option<TraktUser>> {
    let sql = sql::select("trakt_user_auth")
        .columns(&["*"])
        .condition("user_id = ?")
        .to_sql();

    let res = sqlx::query_as(&sql)
        .bind(user_id)
        .fetch_optional(conn)
        .await?;

    Ok(res)
}

pub async fn update_tokens(
    conn: &mut WriteConnection,
    user_id: i64,
    refresh_token: &str,
    access_token: &str,
    expires_at: &OffsetDateTime,
) -> eyre::Result<()> {
    let expires_at = expires_at.format(&Iso8601::DEFAULT)?;

    let sql = sql::update("trakt_user_auth")
        .columns(&["refresh_token", "access_token", "expires_at"])
        .values(&["?", "?", "?"])
        .condition("user_id = ?")
        .to_sql();

    sqlx::query(&sql)
        .bind(refresh_token)
        .bind(access_token)
        .bind(expires_at)
        .bind(user_id)
        .execute(conn)
        .await?;

    Ok(())
}

/// Sets trakt refresh token to null. This should be used to indicate that the
/// user needs to reauthenticate with trakt, such as if we failed to exchange
/// the refresh token for an access token.
pub async fn invalidate_tokens(conn: &mut WriteConnection, user_id: i64) -> eyre::Result<()> {
    let sql = sql::update("trakt_user_auth")
        .columns(&["refresh_token", "access_token", "expires_at"])
        .values(&["null", "null", "null"])
        .condition("user_id = ?")
        .to_sql();

    sqlx::query(&sql).bind(user_id).execute(conn).await?;

    Ok(())
}
