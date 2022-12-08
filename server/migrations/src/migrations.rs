#[path = "../migrations/20220809142403_initial/mod.rs"]
mod _20220809142403_initial;
#[path = "../migrations/20220911141855_flatten_media_items/mod.rs"]
mod _20220911141855_flatten_media_items;
use sqlx::SqliteConnection;

async fn _20220809142403_initial(conn: &mut SqliteConnection) -> eyre::Result<()> {
    _20220809142403_initial::execute(conn).await
}

async fn _20220911141855_flatten_media_items(conn: &mut SqliteConnection) -> eyre::Result<()> {
    _20220911141855_flatten_media_items::execute(conn).await
}

async fn _20221207120648_collections(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20221207120648_collections.sql"))
        .execute(conn)
        .await?;
    Ok(())
}

pub(super) fn collect(migrator: &mut super::Migrator) {
    migrator.push_migration(
        "20220809142403_initial",
        Box::new(|conn| Box::pin(_20220809142403_initial(conn))),
        [10, 1, 107, 203, 53, 43, 3, 92, 120, 1, 114, 206, 16, 254, 187, 139, 98, 126, 165, 176, 199, 148, 86, 161, 217, 178, 254, 124, 51, 101, 120, 218],
    );
    migrator.push_migration(
        "20220911141855_flatten_media_items",
        Box::new(|conn| Box::pin(_20220911141855_flatten_media_items(conn))),
        [59, 203, 61, 12, 6, 78, 97, 147, 247, 200, 212, 27, 62, 162, 74, 81, 64, 3, 87, 173, 210, 30, 230, 208, 30, 9, 234, 228, 99, 155, 177, 79],
    );
    migrator.push_migration(
        "20221207120648_collections",
        Box::new(|conn| Box::pin(_20221207120648_collections(conn))),
        [194, 58, 200, 181, 94, 145, 118, 34, 29, 39, 152, 79, 196, 97, 37, 176, 2, 205, 19, 70, 92, 56, 24, 17, 169, 50, 41, 148, 46, 205, 14, 173],
    );
}
