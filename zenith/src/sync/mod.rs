pub mod movies;
pub mod tv_shows;

use std::sync::Arc;

use futures::FutureExt;
use tokio::sync::mpsc;

use crate::config::Config;
use crate::db::Db;
use crate::ffmpeg::Ffprobe;
use crate::lifecycle::AppLifecycle;
use crate::metadata::MetadataManager;

#[derive(Clone)]
pub struct LibrarySync(mpsc::UnboundedSender<Request>);

#[derive(Debug)]
enum Request {
    StartFullSync,
}

impl LibrarySync {
    pub fn new(
        db: Db,
        metadata: MetadataManager,
        config: Arc<Config>,
        lifecycle: &AppLifecycle,
    ) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let task = tokio::spawn(sync_service(rx, db, metadata, config));

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
    db: Db,
    metadata: MetadataManager,
    config: Arc<Config>,
) {
    while let Some(req) = rx.recv().await {
        match req {
            Request::StartFullSync => {
                // Consume all pending requests, to avoid running unnecessary sync jobs
                // TODO: This will break if other request types are added
                while rx.recv().now_or_never().flatten().is_some() {}

                // Actually do the sync
                if let Err(e) = full_sync(&db, &metadata, &config).await {
                    log::error!("sync failed: {}", e.to_string());
                }
            }
        }
    }
}

async fn full_sync(db: &Db, metadata: &MetadataManager, config: &Config) -> eyre::Result<()> {
    log::info!("running full library sync");

    let ffprobe = Ffprobe::new(&config.transcoding.ffprobe_path);
    let mut conn = db.acquire().await?;

    movies::sync_movies(&mut conn, &metadata, &ffprobe, &config.libraries.movies).await?;
    tv_shows::sync_tv_shows(&mut conn, &metadata, &ffprobe, &config.libraries.tv_shows).await?;

    log::info!("sync complete");

    Ok(())
}
