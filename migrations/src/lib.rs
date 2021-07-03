use sqlx::migrate::MigrateError;
use sqlx::migrate::Migrator;
use sqlx::SqlitePool;

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn migrate(pool: &SqlitePool) -> Result<(), MigrateError> {
    MIGRATOR.run(pool).await
}
