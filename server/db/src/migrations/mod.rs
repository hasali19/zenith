#[allow(clippy::module_inception)]
#[rustfmt::skip]
mod migrations;

use std::future::Future;
use std::pin::Pin;

use eyre::{Context, eyre};

use crate::{Db, WriteConnection};

type MigrationFn = Box<
    dyn for<'a> Fn(&'a mut WriteConnection) -> Pin<Box<dyn Future<Output = eyre::Result<()>> + 'a>>,
>;

struct Migration {
    name: String,
    hash: [u8; 32],
    runner: MigrationFn,
}

struct Migrator {
    migrations: Vec<Migration>,
}

impl Migrator {
    fn push_migration(&mut self, name: &str, f: MigrationFn, hash: [u8; 32]) {
        self.migrations.push(Migration {
            name: name.to_owned(),
            runner: f,
            hash,
        })
    }
}

#[tracing::instrument(skip(db))]
pub async fn migrate(db: &Db) -> eyre::Result<()> {
    let mut migrator = Migrator { migrations: vec![] };
    migrations::collect(&mut migrator);

    let mut conn = db.acquire_write().await?;

    init_migration_table(&mut conn).await?;

    let executed_migrations = get_current_migrations(&mut conn).await?;

    for (i, migration) in migrator.migrations.into_iter().enumerate() {
        if let Some(executed) = executed_migrations.get(i) {
            verify_migration(&migration, executed)
                .await
                .wrap_err_with(|| eyre!("verification failed for '{}'", migration.name))?;
        } else {
            apply_migration(&mut conn, &migration)
                .await
                .wrap_err_with(|| eyre!("failed to apply migration '{}'", migration.name))?;
        }
    }

    Ok(())
}

async fn init_migration_table(conn: &mut WriteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("schema.sql"))
        .execute(conn)
        .await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct ExecutedMigration {
    hash: Vec<u8>,
}

async fn get_current_migrations(
    conn: &mut WriteConnection,
) -> eyre::Result<Vec<ExecutedMigration>> {
    sqlx::query_as("select version, name, hash from _migrations order by version asc")
        .fetch_all(conn)
        .await
        .map_err(Into::into)
}

async fn verify_migration(migration: &Migration, executed: &ExecutedMigration) -> eyre::Result<()> {
    tracing::debug!("verifying existing migration: {}", migration.name);
    if migration.hash != executed.hash.as_slice() {
        return Err(eyre!("migration hash has changed"));
    }
    Ok(())
}

async fn apply_migration(conn: &mut WriteConnection, migration: &Migration) -> eyre::Result<()> {
    tracing::info!("applying migration '{}'", migration.name);
    let mut tx = conn.begin().await?;
    (migration.runner)(&mut tx).await?;
    record_migration(&mut tx, &migration.name, &migration.hash).await?;
    tx.commit().await?;
    Ok(())
}

async fn record_migration(conn: &mut WriteConnection, name: &str, hash: &[u8]) -> eyre::Result<()> {
    sqlx::query("insert into _migrations (name, hash) values (?, ?)")
        .bind(name)
        .bind(hash)
        .execute(conn)
        .await?;
    Ok(())
}
