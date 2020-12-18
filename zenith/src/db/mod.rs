pub mod movies;
pub mod tv_shows;

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{self, Ready};
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Executor, Sqlite, SqlitePool, Transaction};

#[derive(Clone, Debug)]
pub struct Db(SqlitePool);

impl Db {
    pub async fn init() -> sqlx::Result<Self> {
        let options = SqliteConnectOptions::new()
            .filename("zenith.db")
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        pool.execute(include_str!("schema.sql")).await?;

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

impl FromRequest for Db {
    type Error = ();
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        future::ok(req.app_data::<Self>().unwrap().clone())
    }
}
