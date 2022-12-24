pub mod collections;
pub mod episodes;
pub mod items;
pub mod media;
pub mod streams;
pub mod subtitles;
pub mod videos;

use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, Sqlite, SqlitePool, Transaction};

#[derive(Clone, Debug)]
pub struct Db(SqlitePool);

impl Db {
    pub async fn init(path: &str) -> eyre::Result<Self> {
        let mut options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);

        options.disable_statement_logging();

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
