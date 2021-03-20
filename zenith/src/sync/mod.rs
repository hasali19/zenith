pub mod movies;
pub mod tv_shows;

use std::sync::Arc;

use futures::FutureExt;
use tokio::sync::mpsc;

use crate::config::Config;
use crate::ffmpeg::Ffprobe;
use crate::fs::RealFs;
use crate::library::MediaLibrary;
use crate::lifecycle::AppLifecycle;

#[derive(Clone)]
pub struct LibrarySync(mpsc::UnboundedSender<Request>);

#[derive(Debug)]
enum Request {
    StartFullSync,
}

impl LibrarySync {
    pub fn new(
        library: impl MediaLibrary + Send + Sync + 'static,
        config: Arc<Config>,
        lifecycle: &AppLifecycle,
    ) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let task = tokio::spawn(sync_service(rx, library, config));

        lifecycle.on_stopped(async move {
            task.abort();
        });

        LibrarySync(tx)
    }

    pub fn start_full_sync(&mut self) {
        self.0.send(Request::StartFullSync).unwrap();
    }
}

async fn sync_service(
    mut rx: mpsc::UnboundedReceiver<Request>,
    library: impl MediaLibrary,
    config: Arc<Config>,
) {
    while let Some(req) = rx.recv().await {
        match req {
            Request::StartFullSync => {
                // Consume all pending requests, to avoid running unnecessary sync jobs
                // TODO: This will break if other request types are added
                while rx.recv().now_or_never().flatten().is_some() {}

                // Actually do the sync
                if let Err(e) = full_sync(&library, &config).await {
                    tracing::error!("sync failed: {}", e.to_string());
                }
            }
        }
    }
}

async fn full_sync(library: &impl MediaLibrary, config: &Config) -> eyre::Result<()> {
    tracing::info!("running full library sync");

    let ffprobe = Ffprobe::new(&config.transcoding.ffprobe_path);

    movies::sync_movies(library, &RealFs, &ffprobe, &config.libraries.movies).await?;
    tv_shows::sync_tv_shows(library, &RealFs, &ffprobe, &config.libraries.tv_shows).await?;

    tracing::info!("sync complete");

    Ok(())
}
