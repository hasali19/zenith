pub mod media;
pub mod subtitles;
pub mod utils;
pub mod videos;

use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Sqlite, SqlitePool, Transaction};

#[derive(Clone, Debug)]
pub struct Db(SqlitePool);

impl Db {
    pub async fn init(path: &str) -> sqlx::Result<Self> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        // Migrate database to latest version
        tracing::info!("running migrations");
        migrations::migrate(&pool).await?;

        Ok(Db(pool))
    }

    pub async fn acquire(&self) -> sqlx::Result<PoolConnection<Sqlite>> {
        self.0.acquire().await
    }

    pub async fn begin(&self) -> sqlx::Result<Transaction<'_, Sqlite>> {
        self.0.begin().await
    }

    pub async fn close(&self) {
        self.0.close().await
    }
}
