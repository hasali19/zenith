mod utils;

pub mod collections;
pub mod episodes;
pub mod items;
pub mod media;
pub mod movies;
pub mod seasons;
pub mod shows;
pub mod streams;
pub mod subtitles;
pub mod videos;

use std::convert::Infallible;
use std::future::{ready, Ready};

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

impl actix_web::FromRequest for Db {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(Ok(req.app_data().cloned().unwrap()))
    }
}
