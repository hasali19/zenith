use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer};
use tmdb::TmdbClient;
use tracing_actix_web::TracingLogger;
use zenith::config::Config;
use zenith::db::Db;
use zenith::ffprobe::Ffprobe;
use zenith::library::scanner::{LibraryScanner, ScanOptions};
use zenith::library::MediaLibrary;
use zenith::metadata::MetadataManager;
use zenith::transcoder::Transcoder;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("info,sqlx::query=warn")
        .init();

    let config = Arc::new(Config::load("config.yml")?);
    let db = Db::init(&config.database.path).await?;
    let tmdb = TmdbClient::new(&config.tmdb.access_token);
    let metadata = MetadataManager::new(db.clone(), tmdb);
    let video_info_provider = Arc::new(Ffprobe::new(&config.transcoding.ffprobe_path));
    let library = Arc::new(MediaLibrary::new(db.clone(), video_info_provider.clone()));
    let transcoder = Transcoder::new(db.clone(), config.clone());
    let scanner = Arc::new(LibraryScanner::new(
        db.clone(),
        library.clone(),
        metadata.clone(),
        config.clone(),
        video_info_provider,
    ));

    scanner.clone().start_scan(ScanOptions::default());
    transcoder.clone().start();

    let addr = SocketAddr::from_str(&format!("{}:{}", config.http.host, config.http.port))?;

    HttpServer::new({
        let db = db.clone();
        move || {
            App::new()
                .app_data(config.clone())
                .app_data(db.clone())
                .app_data(metadata.clone())
                .app_data(transcoder.clone())
                .app_data(scanner.clone())
                .wrap(TracingLogger::default())
                .service(zenith::api::service())
                .default_service(web::get().to(spa))
        }
    })
    .bind(addr)?
    .run()
    .await?;

    db.close().await;

    Ok(())
}

async fn spa(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path = Path::new("client/web/dist").join(req.uri().path().trim_start_matches('/'));
    let path = if path.is_file() {
        path.as_path()
    } else {
        Path::new("client/web/dist/index.html")
    };

    Ok(NamedFile::open(path)?)
}
