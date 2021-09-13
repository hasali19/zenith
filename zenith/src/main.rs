use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use atium::logger::Logger;
use atium::respond::RespondRequestExt;
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
    let transcoder = Transcoder::new(db.clone());
    let scanner = Arc::new(LibraryScanner::new(
        db.clone(),
        library.clone(),
        metadata.clone(),
        config.clone(),
        video_info_provider,
    ));

    start_event_broadcaster(broadcaster.clone(), &transcoder);

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

fn start_event_broadcaster(broadcaster: Arc<Broadcaster>, transcoder: &Transcoder) {
    // Broadcast events from transcoder to SSE clients
    tokio::spawn({
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
}

#[atium::endpoint]
async fn spa(req: &mut Request) -> eyre::Result<()> {
    let path = Path::new("client/web/dist").join(req.uri().path().trim_start_matches('/'));
    let path = if path.is_file() {
        path.as_path()
    } else {
        Path::new("client/web/dist/index.html")
    };

    req.respond_file(path).await?;

    Ok(())
}
