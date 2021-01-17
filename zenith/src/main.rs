use std::path::Path;

use actix_files::NamedFile;
use actix_web::middleware::normalize::TrailingSlash;
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

use env_logger::Env;

use zenith::api;
use zenith::config::Config;
use zenith::db::Db;
use zenith::ffmpeg::Ffprobe;
use zenith::sync::movies::sync_movies;
use zenith::sync::tv_shows::sync_tv_shows;
use zenith::tmdb::TmdbClient;
use zenith::transcoder::Transcoder;

async fn sync_libraries(db: &Db, tmdb: &TmdbClient, config: &Config) -> eyre::Result<()> {
    let ffprobe = Ffprobe::new(config.ffprobe_path());
    let mut conn = db.acquire().await?;

    sync_movies(&mut conn, &tmdb, &ffprobe, &config.movie_path).await?;
    sync_tv_shows(&mut conn, &tmdb, &ffprobe, &config.tv_show_path).await?;

    Ok(())
}

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::new().default_filter_or("info,sqlx::query=warn"));

    let config = Config::load("config.yml")?;
    let db = Db::init(config.db_path.as_deref().unwrap_or("zenith.db")).await?;
    let tmdb = TmdbClient::new(&config.tmdb_access_token);

    sync_libraries(&db, &tmdb, &config).await?;

    let transcoder = Transcoder::new(db.clone(), &config);

    HttpServer::new({
        let db = db.clone();
        move || {
            App::new()
                .app_data(db.clone())
                .app_data(transcoder.clone())
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
