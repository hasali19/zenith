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

async fn _20230121173622_trailers(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20230121173622_trailers.sql"))
        .execute(conn)
        .await?;
    Ok(())
}

async fn _20230216113153_subtitle_type(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20230216113153_subtitle_type.sql"))
        .execute(conn)
        .await?;
    Ok(())
}

async fn _20230408085807_multiple_video_files(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20230408085807_multiple_video_files.sql"))
        .execute(conn)
        .await?;
    Ok(())
}

async fn _20230410172401_video_path_stem(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20230410172401_video_path_stem.sql"))
        .execute(conn)
        .await?;
    Ok(())
}

async fn _20230414220002_video_scanned_at(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20230414220002_video_scanned_at.sql"))
        .execute(conn)
        .await?;
    Ok(())
}

async fn _20230414222557_audio_channels(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20230414222557_audio_channels.sql"))
        .execute(conn)
        .await?;
    Ok(())
}

async fn _20230417123322_users(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20230417123322_users.sql"))
        .execute(conn)
        .await?;
    Ok(())
}

async fn _20230618085148_cast(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20230618085148_cast.sql"))
        .execute(conn)
        .await?;
    Ok(())
}

async fn _20230618162615_crew(conn: &mut SqliteConnection) -> eyre::Result<()> {
    sqlx::query(include_str!("../migrations/20230618162615_crew.sql"))
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
    migrator.push_migration(
        "20230121173622_trailers",
        Box::new(|conn| Box::pin(_20230121173622_trailers(conn))),
        [242, 183, 94, 127, 233, 34, 168, 212, 229, 75, 246, 133, 123, 71, 22, 140, 141, 36, 97, 74, 141, 151, 255, 112, 183, 215, 16, 29, 247, 129, 221, 183],
    );
    migrator.push_migration(
        "20230216113153_subtitle_type",
        Box::new(|conn| Box::pin(_20230216113153_subtitle_type(conn))),
        [201, 128, 238, 33, 237, 106, 51, 226, 182, 229, 33, 43, 32, 247, 110, 146, 187, 100, 29, 83, 186, 73, 165, 208, 152, 88, 225, 188, 186, 159, 197, 11],
    );
    migrator.push_migration(
        "20230408085807_multiple_video_files",
        Box::new(|conn| Box::pin(_20230408085807_multiple_video_files(conn))),
        [156, 65, 67, 86, 42, 176, 187, 87, 57, 55, 172, 128, 157, 241, 173, 12, 132, 157, 169, 195, 51, 138, 215, 114, 203, 162, 24, 213, 92, 32, 229, 23],
    );
    migrator.push_migration(
        "20230410172401_video_path_stem",
        Box::new(|conn| Box::pin(_20230410172401_video_path_stem(conn))),
        [81, 68, 46, 190, 224, 28, 14, 42, 115, 59, 74, 114, 203, 151, 56, 65, 200, 106, 188, 240, 130, 179, 157, 81, 201, 52, 121, 207, 71, 242, 44, 106],
    );
    migrator.push_migration(
        "20230414220002_video_scanned_at",
        Box::new(|conn| Box::pin(_20230414220002_video_scanned_at(conn))),
        [119, 235, 37, 125, 239, 159, 6, 141, 114, 183, 71, 74, 185, 82, 63, 64, 195, 199, 189, 0, 188, 68, 251, 220, 180, 122, 228, 6, 173, 184, 152, 157],
    );
    migrator.push_migration(
        "20230414222557_audio_channels",
        Box::new(|conn| Box::pin(_20230414222557_audio_channels(conn))),
        [33, 132, 11, 54, 150, 106, 92, 128, 238, 133, 104, 226, 143, 66, 15, 174, 171, 253, 34, 225, 24, 201, 214, 133, 142, 247, 231, 2, 250, 131, 127, 230],
    );
    migrator.push_migration(
        "20230417123322_users",
        Box::new(|conn| Box::pin(_20230417123322_users(conn))),
        [83, 38, 120, 38, 194, 254, 222, 255, 239, 32, 150, 137, 222, 39, 64, 251, 45, 175, 28, 202, 162, 83, 142, 133, 223, 244, 109, 106, 227, 163, 109, 72],
    );
    migrator.push_migration(
        "20230618085148_cast",
        Box::new(|conn| Box::pin(_20230618085148_cast(conn))),
        [141, 86, 99, 131, 188, 150, 45, 46, 204, 176, 74, 32, 125, 199, 202, 253, 220, 65, 189, 171, 125, 17, 177, 116, 23, 212, 133, 200, 56, 21, 126, 143],
    );
    migrator.push_migration(
        "20230618162615_crew",
        Box::new(|conn| Box::pin(_20230618162615_crew(conn))),
        [39, 10, 244, 79, 192, 6, 187, 207, 59, 17, 24, 248, 186, 121, 202, 231, 212, 200, 139, 169, 186, 234, 174, 240, 62, 213, 80, 244, 40, 59, 11, 141],
    );
}
