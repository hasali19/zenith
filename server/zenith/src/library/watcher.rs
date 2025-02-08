use std::sync::Arc;
use std::time::Duration;

use eyre::bail;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;
use tokio::time::timeout;

use crate::config::Config;

use super::LibraryScanner;

pub fn start(config: Arc<Config>, scanner: Arc<LibraryScanner>) {
    tokio::spawn(async move {
        if let Err(e) = run(config, scanner).await {
            tracing::error!("{e:?}");
        }
    });
}

#[tracing::instrument(skip_all)]
async fn run(config: Arc<Config>, scanner: Arc<LibraryScanner>) -> eyre::Result<()> {
    let (event_sender, event_receiver) = mpsc::channel(1);

    let event_handler = tokio::spawn(handle_events(event_receiver, scanner));

    let watcher = tokio::task::spawn_blocking(move || -> eyre::Result<RecommendedWatcher> {
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            let e = match res {
                Ok(e) => e,
                Err(e) => {
                    tracing::error!("{e}");
                    return;
                }
            };

            if !is_write_event(e.kind) {
                return;
            }

            if let Err(e) = event_sender.blocking_send(e) {
                tracing::warn!("failed to send event: {e}");
            }
        })?;

        let movies_lib = &config.libraries.movies;
        let shows_lib = &config.libraries.tv_shows;

        for path in [movies_lib, shows_lib] {
            tracing::info!("watching path: {path}");
            watcher.watch(path.as_std_path(), RecursiveMode::Recursive)?;
        }

        Ok(watcher)
    })
    .await?;

    event_handler.await?;

    drop(watcher);

    bail!("filesystem watcher terminated unexpectedly")
}

async fn handle_events(
    mut event_receiver: mpsc::Receiver<notify::Event>,
    scanner: Arc<LibraryScanner>,
) {
    let wait_time = Duration::from_secs(5);

    'outer: while let Some(e) = event_receiver.recv().await {
        tracing::info!("filesystem change detected, waiting a bit for more changes: {e:?}");

        while let Ok(e) = timeout(wait_time, event_receiver.recv()).await {
            let Some(e) = e else {
                continue 'outer;
            };

            tracing::trace!("filesystem change detected, restarting wait: {e:?}");
        }

        scanner.clone().start_scan();
    }
}

fn is_write_event(event_kind: notify::EventKind) -> bool {
    use notify::EventKind::*;
    match event_kind {
        Any => false,
        Access(_) => false,
        Create(_) => true,
        Modify(_) => true,
        Remove(_) => true,
        Other => false,
    }
}
