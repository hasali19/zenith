use std::path::Path;
use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::middleware::normalize::TrailingSlash;
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

use env_logger::Env;

use zenith::api;
use zenith::config::Config;
use zenith::db::Db;
use zenith::sync::SyncService;
use zenith::tmdb::TmdbClient;
use zenith::transcoder::Transcoder;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::new().default_filter_or("info,sqlx::query=warn"));

    let config = Arc::new(Config::load("config.yml")?);
    let tmdb = TmdbClient::new(&config.tmdb_access_token);
    let db = Db::init(config.db_path()).await?;
    let transcoder = Transcoder::new(db.clone(), &config);
    let sync_service = SyncService::new(db.clone(), tmdb, config.clone());

    HttpServer::new({
        let db = db.clone();
        move || {
            App::new()
                .app_data(db.clone())
                .app_data(transcoder.clone())
                .app_data(sync_service.clone())
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .wrap(Logger::default())
                .service(api::service("/api"))
                .default_service(web::get().to(spa_fallback))
        }
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await?;

    db.close().await;

    Ok(())
}

async fn spa_fallback(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let path = Path::new("zenith_web/build").join(req.path().trim_start_matches('/'));
    if path.is_file() {
        Ok(NamedFile::open(path)?)
    } else {
        Ok(NamedFile::open("zenith_web/build/index.html")?)
    }
}
