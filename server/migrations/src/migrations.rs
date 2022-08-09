#[path = "../migrations/20220809142403_initial/mod.rs"]
mod _20220809142403_initial;
use sqlx::SqliteConnection;

async fn _20220809142403_initial(conn: &mut SqliteConnection) -> eyre::Result<()> {
    _20220809142403_initial::execute(conn).await
}

pub(super) fn collect(migrator: &mut super::Migrator) {
    migrator.push_migration(
        "20220809142403_initial",
        Box::new(|conn| Box::pin(_20220809142403_initial(conn))),
        [10, 1, 107, 203, 53, 43, 3, 92, 120, 1, 114, 206, 16, 254, 187, 139, 98, 126, 165, 176, 199, 148, 86, 161, 217, 178, 254, 124, 51, 101, 120, 218],
    );
}
