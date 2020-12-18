pub mod movies;

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Executor, SqlitePool};

pub async fn init_db() -> sqlx::Result<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename("zenith.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    pool.execute(include_str!("schema.sql")).await?;

    Ok(pool)
}
