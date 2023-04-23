pub mod collections;
pub mod items;
pub mod media;
pub mod sql;
pub mod streams;
pub mod subtitles;
pub mod users;
pub mod video_files;
pub mod videos;

use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, Sqlite, SqlitePool, Transaction};
use tempfile::NamedTempFile;

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

    pub async fn backup(&self) -> sqlx::Result<Vec<u8>> {
        let file = tokio::task::spawn_blocking(NamedTempFile::new)
            .await
            .unwrap()?;

        let path = file.path();

        sqlx::query(&format!("VACUUM INTO {path:?}"))
            .execute(&mut self.acquire().await?)
            .await?;

        Ok(tokio::fs::read(path).await?)
    }

    pub async fn close(&self) {
        self.0.close().await
    }
}
