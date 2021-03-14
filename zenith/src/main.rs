use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use env_logger::Env;

use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use zenith::config::Config;
use zenith::db::Db;
use zenith::library::MediaLibraryImpl;
use zenith::lifecycle::AppLifecycle;
use zenith::metadata::MetadataManager;
use zenith::sync::LibrarySync;
use zenith::tmdb::TmdbClient;
use zenith::watcher::FileWatcher;
use zenith::{middleware, AppState};
use zenith_http::headers::ContentType;
use zenith_http::{App, Body, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::new().default_filter_or("info,sqlx::query=warn"));

    let lifecycle = AppLifecycle::new();
    let config = Arc::new(Config::load("config.yml")?);
    let db = Db::init(&config.database.path).await?;
    let tmdb = TmdbClient::new(&config.tmdb.access_token);
    let metadata = MetadataManager::new(db.clone(), tmdb);
    let library = MediaLibraryImpl::new(db.clone(), metadata.clone());
    let mut sync = LibrarySync::new(library, config.clone(), &lifecycle);

    sync.start_full_sync();

    let mut watcher = FileWatcher::spawn({
        let mut sync = sync.clone();
        move |_| {
            // Run full sync anytime anything changes
            // TODO: Make this more clever
            sync.start_full_sync();
        }
    });

    watcher.watch(&config.libraries.movies);
    watcher.watch(&config.libraries.tv_shows);

    let mut app = App::new(AppState {
        config: config.clone(),
        db: db.clone(),
        sync,
        metadata,
    });

    let addr = SocketAddr::from_str(&format!("{}:{}", config.http.host, config.http.port))?;

    app.wrap(middleware::Logger);
    app.configure(zenith::api::configure);
    app.fallback_to(spa);
    app.run(addr).await?;

    lifecycle.signal_stopped()?;

    db.close().await;

    Ok(())
}

async fn spa(_: AppState, req: Request) -> Result<Response, Response> {
    let path = Path::new("zenith_web/dist").join(req.uri().path().trim_start_matches('/'));
    let path = if path.is_file() {
        path.as_path()
    } else {
        Path::new("zenith_web/dist/index.html")
    };

    let mime = mime_guess::from_path(path);
    let file = File::open(path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);

    Ok(Response::new()
        .with_header(ContentType::from(mime.first_or_text_plain()))
        .with_body(body))
}
