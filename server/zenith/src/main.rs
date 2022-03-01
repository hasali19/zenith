use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::OriginalUri;
use axum::http::StatusCode;
use axum::AddExtensionLayer;
use axum_files::{FileRequest, FileResponse};
use tmdb::TmdbClient;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use zenith::config::Config;
use zenith::db::Db;
use zenith::library::scanner::{LibraryScanner, ScanOptions};
use zenith::library::MediaLibrary;
use zenith::metadata::MetadataManager;
use zenith::transcoder::Transcoder;
use zenith::video_prober::Ffprobe;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(if std::env::var_os("RUST_LOG").is_some() {
            EnvFilter::from_default_env()
        } else {
            EnvFilter::from("info")
        })
        .with_target(true)
        .init();

    match std::env::args().nth(1).as_deref().unwrap_or("serve") {
        "openapi" => {
            let spec = zenith::api::openapi_spec();
            let json = serde_json::to_string_pretty(&spec)?;
            println!("{json}");
        }
        "serve" => {
            run_server().await?;
        }
        cmd => {
            eprintln!("unrecognised command: {cmd}");
        }
    }

    Ok(())
}

async fn run_server() -> eyre::Result<()> {
    let config = Arc::new(Config::load("config.yml")?);
    let db = Db::init(&config.database.path).await?;
    let tmdb = TmdbClient::new(&config.tmdb.api_key);
    let metadata = MetadataManager::new(db.clone(), tmdb.clone());
    let video_prober = Arc::new(Ffprobe::new(&config.transcoding.ffprobe_path));
    let library = Arc::new(MediaLibrary::new(db.clone(), video_prober.clone()));
    let transcoder = Transcoder::new(db.clone(), config.clone(), video_prober.clone());
    let scanner = Arc::new(LibraryScanner::new(
        db.clone(),
        library.clone(),
        metadata.clone(),
        config.clone(),
        video_prober,
        transcoder.clone(),
    ));

    scanner.clone().start_scan(ScanOptions::default());
    transcoder.clone().start();

    if config.watcher.enabled {
        zenith::library::watcher::start(config.clone(), scanner.clone());
    }

    let addr = SocketAddr::from_str(&format!("{}:{}", config.http.host, config.http.port))?;

    let app = axum::Router::new()
        .nest("/api", zenith::api::router())
        .fallback(axum::routing::get(spa))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(config))
        .layer(AddExtensionLayer::new(db.clone()))
        .layer(AddExtensionLayer::new(metadata))
        .layer(AddExtensionLayer::new(transcoder))
        .layer(AddExtensionLayer::new(scanner))
        .layer(AddExtensionLayer::new(tmdb));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install ctrl+c signal handler")
        })
        .await?;

    db.close().await;

    Ok(())
}

async fn spa(OriginalUri(uri): OriginalUri, file: FileRequest) -> Result<FileResponse, StatusCode> {
    let path = uri.path().trim_start_matches('/');

    if path.starts_with("api") {
        return Err(StatusCode::NOT_FOUND);
    }

    let path = Path::new("client/web/dist").join(path);
    let path = if path.is_file() {
        path.as_path()
    } else {
        Path::new("client/web/dist/index.html")
    };

    Ok(FileResponse::from_request(file, path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
}
