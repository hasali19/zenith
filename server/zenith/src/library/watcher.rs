use std::sync::Arc;
use std::time::Duration;

use eyre::{bail, eyre};
use notify::{Event, RecursiveMode, Watcher};
use tokio::time::timeout;

use crate::config::Config;
use crate::library::scanner::ScanOptions;

use super::LibraryScanner;

pub fn start(config: Arc<Config>, scanner: Arc<LibraryScanner>) {
    tokio::spawn(async move {
        if let Err(e) = run(config, scanner).await {
            tracing::error!("{e:?}");
        }
    });
}

#[tracing::instrument(skip(config, scanner))]
async fn run(config: Arc<Config>, scanner: Arc<LibraryScanner>) -> eyre::Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    let mut watcher =
        notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            match res.map_err(|e| eyre!(e)) {
                Ok(e) => tx.blocking_send(e).unwrap(),
                Err(e) => tracing::error!("{e:?}"),
            }
        })?;

    let movies_lib = &config.libraries.movies;
    let shows_lib = &config.libraries.tv_shows;

    for path in [movies_lib, shows_lib] {
        watcher.watch(path.as_std_path(), RecursiveMode::Recursive)?;
    }

    let wait_time = Duration::from_secs(5);

    while rx.recv().await.is_some() {
        tracing::info!("filesystem change detected, waiting a bit for more changes");

        while timeout(wait_time, rx.recv()).await.is_ok() {
            tracing::trace!("filesystem change detected, restarting wait");
            continue;
        }

        scanner.clone().start_scan(ScanOptions::quick());
    }

    bail!("filesystem watcher terminated unexpectedly")
}
