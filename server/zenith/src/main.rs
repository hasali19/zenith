use std::io::ErrorKind;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::OriginalUri;
use axum::http::StatusCode;
use axum::Extension;
use axum_files::{FileRequest, FileResponse};
use eyre::bail;
use futures::FutureExt;
use tmdb::TmdbClient;
use tokio::sync::Notify;
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
        .layer(Extension(config))
        .layer(Extension(db.clone()))
        .layer(Extension(metadata))
        .layer(Extension(transcoder))
        .layer(Extension(scanner))
        .layer(Extension(tmdb));

    {
        let shutdown = Notify::new();
        let server = axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown.notified());

        let ctrl_c =
            tokio::signal::ctrl_c().map(|r| r.expect("failed to install ctrl+c signal handler"));

        tokio::pin!(server, ctrl_c);

        tokio::select! {
            _ = &mut server => bail!("server shut down unexpectedly"),
            _ = ctrl_c => tracing::info!("shutdown triggered, waiting for open connections"),
        }

        shutdown.notify_waiters();

        tokio::select! {
            _ = &mut server => {},
            _ = tokio::time::sleep(Duration::from_secs(3)) => {
                tracing::warn!("server took too long to respond, forcing shutdown");
            }
        }
    }

    if tokio::time::timeout(Duration::from_secs(3), db.close())
        .await
        .is_err()
    {
        tracing::warn!("failed to close database connection");
    }

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

    FileResponse::from_request(file, path)
        .await
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
}
