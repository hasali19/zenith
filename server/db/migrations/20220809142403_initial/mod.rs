use sqlx::SqliteConnection;

pub async fn execute(conn: &mut SqliteConnection) -> eyre::Result<()> {
    let sql = "
        select name from sqlite_master
        where type='table' and name='_sqlx_migrations'";

    let has_sqlx_migrations = sqlx::query(sql).fetch_optional(&mut *conn).await?.is_some();

    if has_sqlx_migrations {
        // If the `_sqlx_migrations` table exists, assume the database is up to date with the old
        // sqlx migrations and carry on.
        return Ok(());
    }

    macro_rules! apply {
        ($conn:expr, $path:literal) => {
            sqlx::query(include_str!(concat!($path, ".sql")))
                .execute(&mut *$conn)
                .await?;
        };
    }

    apply!(conn, "20210703161358_initial");
    apply!(conn, "20210710170626_add_movie_tmdb_id");
    apply!(conn, "20210721195837_add_subtitles");
    apply!(conn, "20210729105737_add_unique_constraints");
    apply!(conn, "20211002154728_add_media_info");
    apply!(conn, "20211030165125_subtitles_v2");
    apply!(conn, "20220108133040_last_watched_time");

    Ok(())
}
