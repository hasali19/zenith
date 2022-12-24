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

async fn _20221211142452_metadata(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20221211142452_metadata.sql"))
        .execute(conn)
        .await?;
    Ok(())
}

async fn _20221224150754_metadata_provider(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20221224150754_metadata_provider.sql"))
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
    migrator.push_migration(
        "20221211142452_metadata",
        Box::new(|conn| Box::pin(_20221211142452_metadata(conn))),
        [162, 136, 200, 125, 128, 41, 60, 76, 249, 95, 226, 131, 126, 42, 243, 165, 86, 250, 77, 183, 129, 0, 162, 113, 200, 158, 184, 232, 205, 105, 126, 114],
    );
    migrator.push_migration(
        "20221224150754_metadata_provider",
        Box::new(|conn| Box::pin(_20221224150754_metadata_provider(conn))),
        [223, 163, 114, 148, 82, 41, 187, 241, 92, 1, 47, 56, 50, 111, 180, 77, 32, 108, 43, 185, 233, 6, 193, 74, 167, 176, 41, 99, 57, 57, 110, 39],
    );
}
