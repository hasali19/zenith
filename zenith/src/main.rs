use actix_web::middleware::normalize::TrailingSlash;
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{App, HttpServer};

use env_logger::Env;

use zenith::api;
use zenith::db::Db;
use zenith::sync::movies::sync_movies;
use zenith::sync::tv_shows::sync_tv_shows;
use zenith::tmdb::TmdbClient;

async fn sync_libraries(db: &Db) -> eyre::Result<()> {
    let mut conn = db.acquire().await?;
    let tmdb = TmdbClient::new(&std::env::var("TMDB_ACCESS_TOKEN").unwrap());

    sync_movies(&mut conn, &tmdb, "/mnt/nyx/sda/media/Movies").await?;
    sync_tv_shows(&mut conn, &tmdb, "/mnt/nyx/sda/media/TV").await?;

    Ok(())
}

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::new().default_filter_or("info,sqlx::query=warn"));

    let db = Db::init().await?;

    sync_libraries(&db).await?;

    HttpServer::new({
        let db = db.clone();
        move || {
            App::new()
                .app_data(db.clone())
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .wrap(Logger::default())
                .service(api::service("/api"))
        }
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await?;

    db.close().await;

    Ok(())
}
