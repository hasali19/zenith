use env_logger::Env;
use sqlx::SqlitePool;
use zenith::sync::movies::sync_movies;
use zenith::sync::tv_shows::sync_tv_shows;

async fn sync_libraries(db: &SqlitePool) -> eyre::Result<()> {
    let mut conn = db.acquire().await?;

    sync_movies(&mut conn, "/mnt/nyx/sda/media/Movies").await?;
    sync_tv_shows(&mut conn, "/mnt/nyx/sda/media/TV").await?;

    Ok(())
}

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init_from_env(Env::new().default_filter_or("info,sqlx::query=warn"));

    let db = zenith::db::init_db().await?;

    sync_libraries(&db).await?;

    db.close().await;

    Ok(())
}
