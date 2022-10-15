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
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};
use zenith::config::{Config, LogFormat};
use zenith::db::media::MediaItemType;
use zenith::db::Db;
use zenith::library::scanner::{LibraryScanner, ScanOptions};
use zenith::library::{LibraryEvent, MediaLibrary};
use zenith::metadata::MetadataManager;
use zenith::transcoder::{self, Transcoder};
use zenith::video_prober::Ffprobe;

fn init_tracing(config: &Config) {
    let fmt_layer = tracing_subscriber::fmt::layer();

    let fmt_layer = match config.logging.format {
        LogFormat::Compact => fmt_layer.compact().boxed(),
        LogFormat::Pretty => fmt_layer.pretty().boxed(),
    };

    let fmt_layer = fmt_layer.with_filter(if std::env::var_os("RUST_LOG").is_some() {
        EnvFilter::from_default_env()
    } else {
        EnvFilter::from("info")
    });

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = Arc::new(Config::load("config.yml")?);

    color_eyre::install()?;
    init_tracing(&config);

    match std::env::args().nth(1).as_deref().unwrap_or("serve") {
        "openapi" => {
            let spec = zenith::api::openapi_spec();
            let json = serde_json::to_string_pretty(&spec)?;
            println!("{json}");
        }
        "serve" => {
            run_server(config).await?;
        }
        cmd => {
            eprintln!("unrecognised command: {cmd}");
        }
    }

    Ok(())
}

async fn run_server(config: Arc<Config>) -> eyre::Result<()> {
    let db = Db::init(&config.database.path).await?;
    let tmdb = TmdbClient::new(&config.tmdb.api_key);
    let metadata = MetadataManager::new(db.clone(), tmdb.clone());
    let video_prober = Arc::new(Ffprobe::new(&config.transcoding.ffprobe_path));
    let library = Arc::new(MediaLibrary::new(db.clone(), video_prober.clone()));
    let transcoder = Transcoder::new(db.clone(), config.clone(), video_prober.clone());
    let scanner = Arc::new(LibraryScanner::new(
        db.clone(),
        library.clone(),
        config.clone(),
        video_prober,
    ));

    tokio::spawn({
        let metadata = metadata.clone();
        let transcoder = transcoder.clone();
        async move {
            let mut receiver = library.subscribe();
            while let Ok(event) = receiver.recv().await {
                if let LibraryEvent::Added(item_type, id) = event {
                    if let MediaItemType::Movie | MediaItemType::Episode = item_type {
                        transcoder.enqueue(transcoder::Job::new(id)).await;
                    }

                    metadata.enqueue(id);
                }
            }
        }
    });

    scanner.clone().start_scan(ScanOptions::default());
    transcoder.clone().start();

    if config.watcher.enabled {
        zenith::library::watcher::start(config.clone(), scanner.clone());
    }

    let addr = SocketAddr::from_str(&format!("{}:{}", config.http.host, config.http.port))?;

    tracing::info!("starting server at http://{addr}");

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

    let path = Path::new("web/dist").join(path);
    let path = if path.is_file() {
        path.as_path()
    } else {
        Path::new("web/dist/index.html")
    };

    FileResponse::from_request(file, path)
        .await
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
}
