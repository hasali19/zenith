use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use tracing_actix_web::TracingLogger;
use zenith::broadcaster::Broadcaster;
use zenith::config::Config;
use zenith::db::Db;
use zenith::ffprobe::Ffprobe;
use zenith::library::scanner::LibraryScanner;
use zenith::library::MediaLibrary;
use zenith::metadata::MetadataManager;
use zenith::tmdb::TmdbClient;
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
    let broadcaster = Broadcaster::new();
    let tmdb = TmdbClient::new(&config.tmdb.access_token);
    let metadata = MetadataManager::new(db.clone(), tmdb);
    let video_info_provider = Arc::new(Ffprobe::new(&config.transcoding.ffprobe_path));
    let library = Arc::new(MediaLibrary::new(db.clone(), video_info_provider));
    let transcoder = Transcoder::new(db.clone());
    let scanner = Arc::new(LibraryScanner::new(
        library.clone(),
        metadata.clone(),
        config.clone(),
    ));

    // Broadcast events from transcoder to SSE clients
    tokio::spawn({
        let broadcaster = broadcaster.clone();
        let mut events = transcoder.subscribe();
        async move {
            loop {
                let event = match events.recv().await {
                    Ok(event) => event,
                    Err(_) => continue,
                };

                let (event, id) = match event {
                    zenith::transcoder::Event::Queued(id) => ("transcoder.queued", id),
                    zenith::transcoder::Event::Started(id) => ("transcoder.started", id),
                    zenith::transcoder::Event::Success(id) => ("transcoder.success", id),
                    zenith::transcoder::Event::Error(id) => ("transcoder.error", id),
                };

                let message = format!("data: {{\"event\": \"{}\", \"id\": {}}}\n\n", event, id);

                broadcaster.send(message);
            }
        }
    });

    scanner.start_scan();
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
                .app_data(broadcaster.clone())
                .app_data(scanner.clone())
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .wrap(Logger::default())
                .wrap(TracingLogger::default())
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
    let path = Path::new("client/web/dist").join(req.uri().path().trim_start_matches('/'));
    let path = if path.is_file() {
        path.as_path()
    } else {
        Path::new("client/web/dist/index.html")
    };

    Ok(NamedFile::open(path)?)
}
