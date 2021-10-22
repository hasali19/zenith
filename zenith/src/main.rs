use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use atium::logger::Logger;
use atium::responder::File;
use atium::router::Router;
use atium::state::State;
use atium::Request;
use zenith::broadcaster::Broadcaster;
use zenith::config::Config;
use zenith::db::Db;
use zenith::ffprobe::Ffprobe;
use zenith::library::scanner::{LibraryScanner, ScanOptions};
use zenith::library::MediaLibrary;
use zenith::metadata::MetadataManager;
use zenith::tmdb::TmdbClient;
use zenith::transcoder::Transcoder;

#[tokio::main]
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

    let router = Router::new().with(|r| {
        r.route("/api/*").any(zenith::api::handler());
    });

    let state = atium::compose!(
        State(config),
        State(db.clone()),
        State(metadata),
        State(transcoder),
        State(broadcaster),
        State(scanner)
    );

    let app = atium::compose!(Logger::default(), state, router, spa);

    atium::run(addr, app).await?;

    db.close().await;

    Ok(())
}

#[atium::endpoint]
async fn spa(req: &mut Request) -> eyre::Result<File> {
    let path = Path::new("client/web/dist").join(req.uri().path().trim_start_matches('/'));
    let path = if path.is_file() {
        path.as_path()
    } else {
        Path::new("client/web/dist/index.html")
    };

    Ok(File::open(path).await?)
}
