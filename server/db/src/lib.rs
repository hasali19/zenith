mod migrations;
mod utils;

pub mod access_tokens;
pub mod collections;
pub mod images;
pub mod items;
pub mod media;
pub mod people;
pub mod sql;
pub mod streams;
pub mod subtitles;
pub mod trakt;
pub mod user_registrations;
pub mod users;
pub mod video_files;
pub mod videos;

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use futures::future::BoxFuture;
use futures::stream::BoxStream;
use sqlx::sqlite::{
    SqliteConnectOptions, SqlitePoolOptions, SqliteQueryResult, SqliteRow, SqliteStatement,
    SqliteTypeInfo,
};
use sqlx::{ConnectOptions, Executor, Sqlite, SqliteConnection, SqlitePool};
use tempfile::NamedTempFile;

#[derive(Clone, Debug)]
pub struct Db {
    read_pool: SqlitePool,
    write_pool: SqlitePool,
}

impl Db {
    pub async fn init(path: &str) -> eyre::Result<Self> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .disable_statement_logging();

        let read_pool = SqlitePool::connect_with(options.clone()).await?;
        let write_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;

        let db = Db {
            read_pool,
            write_pool,
        };

        // Migrate database to latest version
        tracing::info!("running migrations");
        migrations::migrate(&db).await?;

        Ok(db)
    }

    pub async fn acquire(&self) -> sqlx::Result<PoolConnection<Read>> {
        self.read_pool
            .acquire()
            .await
            .map(|c| PoolConnection(c, PhantomData))
    }

    pub async fn acquire_write(&self) -> sqlx::Result<PoolConnection<Write>> {
        self.write_pool
            .acquire()
            .await
            .map(|c| PoolConnection(c, PhantomData))
    }

    pub async fn begin(&self) -> sqlx::Result<Transaction<'_, Read>> {
        self.read_pool
            .begin()
            .await
            .map(|t| Transaction(t, PhantomData))
    }

    pub async fn begin_write(&self) -> sqlx::Result<Transaction<'_, Write>> {
        self.write_pool
            .begin()
            .await
            .map(|t| Transaction(t, PhantomData))
    }

    pub async fn backup(&self) -> sqlx::Result<Vec<u8>> {
        let file = tokio::task::spawn_blocking(NamedTempFile::new)
            .await
            .unwrap()?;

        let path = file.path();

        sqlx::query(&format!("VACUUM INTO {path:?}"))
            .execute(&mut *self.acquire().await?)
            .await?;

        Ok(tokio::fs::read(path).await?)
    }

    pub async fn close(&self) {
        self.read_pool.close().await;
        self.write_pool.close().await;
    }
}

#[repr(transparent)]
pub struct Connection<T>(SqliteConnection, PhantomData<T>);

pub struct Read;
pub struct Write;

pub type ReadConnection = Connection<Read>;
pub type WriteConnection = Connection<Write>;

impl<T> Connection<T> {
    pub async fn begin(&mut self) -> sqlx::Result<Transaction<'_, T>> {
        use sqlx::Connection;
        self.0.begin().await.map(|t| Transaction(t, PhantomData))
    }
}

impl Connection<Write> {
    pub fn as_read(&mut self) -> &mut Connection<Read> {
        let connection = &raw mut self.0;
        unsafe { &mut *connection.cast::<Connection<Read>>() }
    }
}

impl<T> std::fmt::Debug for Connection<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Connection")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

pub struct PoolConnection<T>(sqlx::pool::PoolConnection<Sqlite>, PhantomData<T>);

impl<T> Deref for PoolConnection<T> {
    type Target = Connection<T>;

    fn deref(&self) -> &Self::Target {
        let connection = &raw const *self.0;
        unsafe { &*connection.cast::<Connection<T>>() }
    }
}

impl<T> DerefMut for PoolConnection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let connection = &raw mut *self.0;
        unsafe { &mut *connection.cast::<Connection<T>>() }
    }
}

#[repr(transparent)]
pub struct Transaction<'c, T>(sqlx::Transaction<'c, Sqlite>, PhantomData<T>);

impl<T> Transaction<'_, T> {
    pub async fn commit(self) -> sqlx::Result<()> {
        self.0.commit().await
    }

    pub async fn rollback(self) -> sqlx::Result<()> {
        self.0.rollback().await
    }
}

impl Transaction<'_, Write> {
    pub fn as_read(&mut self) -> &mut Transaction<'_, Read> {
        let transaction = &raw mut self.0;
        unsafe { &mut *transaction.cast::<Transaction<Read>>() }
    }
}

impl<T> Deref for Transaction<'_, T> {
    type Target = Connection<T>;

    fn deref(&self) -> &Self::Target {
        let connection = &raw const *self.0;
        unsafe { &*connection.cast::<Connection<T>>() }
    }
}

impl<T> DerefMut for Transaction<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let connection = &raw mut *self.0;
        unsafe { &mut *connection.cast::<Connection<T>>() }
    }
}

impl<T> std::fmt::Debug for Transaction<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Transaction")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl<'c, T: Send> Executor<'c> for &'c mut Connection<T> {
    type Database = Sqlite;

    fn fetch_many<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxStream<'e, sqlx::Result<sqlx::Either<SqliteQueryResult, SqliteRow>>>
    where
        'c: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        self.0.fetch_many(query)
    }

    fn fetch_optional<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxFuture<'e, sqlx::Result<Option<SqliteRow>>>
    where
        'c: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        self.0.fetch_optional(query)
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [SqliteTypeInfo],
    ) -> BoxFuture<'e, sqlx::Result<SqliteStatement<'q>>>
    where
        'c: 'e,
    {
        self.0.prepare_with(sql, parameters)
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, sqlx::Result<sqlx::Describe<Self::Database>>>
    where
        'c: 'e,
    {
        self.0.describe(sql)
    }
}

impl<'c, T: Send> Executor<'c> for &'c mut Transaction<'c, T> {
    type Database = Sqlite;

    fn fetch_many<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxStream<'e, sqlx::Result<sqlx::Either<SqliteQueryResult, SqliteRow>>>
    where
        'c: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        self.0.fetch_many(query)
    }

    fn fetch_optional<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxFuture<'e, sqlx::Result<Option<SqliteRow>>>
    where
        'c: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        self.0.fetch_optional(query)
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [SqliteTypeInfo],
    ) -> BoxFuture<'e, sqlx::Result<SqliteStatement<'q>>>
    where
        'c: 'e,
    {
        self.0.prepare_with(sql, parameters)
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, sqlx::Result<sqlx::Describe<Self::Database>>>
    where
        'c: 'e,
    {
        self.0.describe(sql)
    }
}
