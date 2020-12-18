use env_logger::Env;

use zenith::sync::movies::sync_movies;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init_from_env(Env::new().default_filter_or("info,sqlx::query=warn"));

    let db = zenith::db::init_db().await?;

    sync_movies(&mut *db.acquire().await?, "/mnt/nyx/sda/media/Movies").await?;

    db.close().await;

    Ok(())
}
