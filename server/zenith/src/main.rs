use std::io::ErrorKind;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use axum::extract::OriginalUri;
use axum::http::StatusCode;
use axum::Extension;
use axum_extra::extract::cookie::Key;
use axum_files::{FileRequest, FileResponse};
use camino::Utf8Path;
use eyre::{bail, Context};
use futures::FutureExt;
use time::OffsetDateTime;
use tmdb::TmdbClient;
use tokio::sync::Notify;
use tokio::time::Instant;
use tower_http::trace::TraceLayer;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};
use zenith::config::{Config, LogFormat};
use zenith::library::scanner::{LibraryScanner, VideoFileType};
use zenith::library::{FileSystemChange, LibraryEvent, MediaLibrary};
use zenith::metadata::MetadataManager;
use zenith::transcoder::{self, Transcoder};
use zenith::video_prober::Ffprobe;
use zenith::{App, Db};

fn init_tracing(config: &Config) {
    let fmt_layer = tracing_subscriber::fmt::layer();

    let fmt_layer = match config.logging.format {
        LogFormat::Compact => fmt_layer.compact().boxed(),
        LogFormat::Pretty => fmt_layer.pretty().boxed(),
    };

    let fmt_layer = fmt_layer.with_filter(if std::env::var_os("RUST_LOG").is_some() {
        EnvFilter::from_default_env()
    } else {
        #[cfg(debug_assertions)]
        let default_filter = "info,zenith=trace";
        #[cfg(not(debug_assertions))]
        let default_filter = "info";
        EnvFilter::from(config.logging.filter.as_deref().unwrap_or(default_filter))
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

fn new_library_scanner(
    db: Db,
    config: Arc<Config>,
    library: Arc<MediaLibrary>,
) -> Arc<LibraryScanner> {
    struct EventHandler(Arc<MediaLibrary>);

    #[async_trait]
    impl zenith::library::scanner::EventHandler for EventHandler {
        async fn process_file_system_change(&self, change: FileSystemChange) -> eyre::Result<()> {
            self.0.process_file_system_change(change).await
        }

        async fn complete_library_scan(&self) -> eyre::Result<()> {
            self.0.validate().await
        }
    }

    let library_paths = vec![
        (VideoFileType::Movie, config.libraries.movies.clone()),
        (VideoFileType::Episode, config.libraries.tv_shows.clone()),
    ];

    Arc::new(LibraryScanner::new(
        db,
        library_paths,
        EventHandler(library),
    ))
}

async fn run_server(config: Arc<Config>) -> eyre::Result<()> {
    let db = Db::init(&config.database.path).await?;
    let tmdb = TmdbClient::new(&config.tmdb.api_key);
    let metadata = MetadataManager::new(db.clone(), tmdb.clone());
    let video_prober = Arc::new(Ffprobe::new(&config.transcoding.ffprobe_path));

    let library = Arc::new(MediaLibrary::new(
        db.clone(),
        config.import.matchers.clone(),
        video_prober.clone(),
    ));

    let transcoder = Transcoder::new(
        db.clone(),
        config.clone(),
        library.clone(),
        video_prober.clone(),
    );

    let scanner = new_library_scanner(db.clone(), config.clone(), library.clone());

    tokio::spawn({
        let metadata = metadata.clone();
        let library = library.clone();
        let transcoder = transcoder.clone();
        async move {
            let mut receiver = library.subscribe();
            while let Ok(event) = receiver.recv().await {
                match event {
                    LibraryEvent::MediaAdded(_, id) => metadata.enqueue_unmatched(id),
                    LibraryEvent::MediaRemoved(_, _) => {}
                    LibraryEvent::VideoAdded(id) => {
                        transcoder.enqueue(transcoder::Job::new(id)).await
                    }
                }
            }
        }
    });

    tokio::spawn({
        let db = db.clone();
        let metadata = metadata.clone();
        async move {
            let now = OffsetDateTime::now_utc();
            let midnight = (now + time::Duration::days(1))
                .replace_time(time::Time::from_hms(0, 0, 0).unwrap());
            let delta = midnight - now;
            let mut interval = tokio::time::interval_at(
                Instant::now() + delta.try_into().unwrap(),
                time::Duration::days(1).try_into().unwrap(),
            );
            loop {
                interval.tick().await;

                if let Err(e) = metadata
                    .enqueue_all_unmatched(&mut db.acquire().await.unwrap())
                    .await
                {
                    tracing::error!("{e:?}");
                }

                if let Err(e) = metadata
                    .enqueue_all_outdated(&mut db.acquire().await.unwrap())
                    .await
                {
                    tracing::error!("{e:?}");
                }
            }
        }
    });

    scanner.clone().start_scan();
    transcoder.clone().start();

    if config.watcher.enabled {
        zenith::library::watcher::start(config.clone(), scanner.clone());
    }

    let addr = SocketAddr::from_str(&format!("{}:{}", config.http.host, config.http.port))?;

    tracing::info!("starting server at http://{addr}");

    let key_path = config.paths.data.join("key");
    let key = load_or_create_key(&key_path)?;

    let app = App { key };

    let router = axum::Router::new()
        .nest("/api", zenith::api::router())
        .with_state(app)
        .fallback_service(axum::routing::get(spa))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(config))
        .layer(Extension(db.clone()))
        .layer(Extension(library.clone()))
        .layer(Extension(metadata))
        .layer(Extension(transcoder))
        .layer(Extension(scanner))
        .layer(Extension(tmdb));

    {
        let shutdown = Notify::new();
        let server = axum::Server::bind(&addr)
            .serve(router.into_make_service())
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

fn load_or_create_key(key_path: &Utf8Path) -> eyre::Result<Key> {
    if key_path.exists() {
        let master_key = std::fs::read(key_path)
            .wrap_err_with(|| format!("failed to read key file: {key_path:?}"))?;
        Key::try_from(master_key.as_slice())
            .wrap_err_with(|| format!("key file is invalid: {key_path:?}"))
    } else {
        let key = Key::generate();
        std::fs::write(key_path, key.master())
            .wrap_err_with(|| format!("failed to write key to file: {key_path:?}"))?;
        Ok(key)
    }
}

async fn spa(OriginalUri(uri): OriginalUri, file: FileRequest) -> Result<FileResponse, StatusCode> {
    let path = uri.path().trim_start_matches('/');

    if path.starts_with("api") {
        return Err(StatusCode::NOT_FOUND);
    }

    let path = Utf8Path::new("web/dist").join(path);
    let path = if path.is_file() {
        path.as_path()
    } else {
        Utf8Path::new("web/dist/index.html")
    };

    FileResponse::from_request(file, path)
        .await
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
}
