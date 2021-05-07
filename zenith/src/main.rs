use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use zenith::config::Config;
use zenith::db::Db;
use zenith::ffmpeg::Ffprobe;
use zenith::library::scanner::LibraryScanner;
use zenith::library::MediaLibrary;
use zenith::metadata::MetadataManager;
use zenith::tmdb::TmdbClient;
use zenith::transcoder::Transcoder;
use zenith::watcher::FileWatcher;

#[actix_rt::main]
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
    let library = Arc::new(MediaLibrary::new(db.clone(), video_info_provider));
    let transcoder = Transcoder::new(db.clone());
    let scanner = Arc::new(LibraryScanner::new(
        library,
        metadata.clone(),
        config.clone(),
    ));

    scanner.start_scan();
    transcoder.clone().start();

    let mut watcher = FileWatcher::spawn({
        let scanner = scanner.clone();
        move |_| {
            scanner.start_scan();
        }
    });

    watcher.watch(&config.libraries.movies);
    watcher.watch(&config.libraries.tv_shows);

    let addr = SocketAddr::from_str(&format!("{}:{}", config.http.host, config.http.port))?;

    HttpServer::new({
        let db = db.clone();
        move || {
            App::new()
                .app_data(config.clone())
                .app_data(db.clone())
                .app_data(metadata.clone())
                .app_data(transcoder.clone())
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .service(zenith::api::service("/api"))
                .default_service(web::to(spa))
        }
    })
    .bind(addr)?
    .run()
    .await?;

    db.close().await;

    Ok(())
}

async fn spa(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let path = Path::new("zenith_web/dist").join(req.uri().path().trim_start_matches('/'));
    let path = if path.is_file() {
        path.as_path()
    } else {
        Path::new("zenith_web/dist/index.html")
    };

    Ok(NamedFile::open(path)?)
}
