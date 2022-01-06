use std::path::Path;
use std::sync::Arc;

use eyre::eyre;
use notify::event::{AccessKind, AccessMode};
use notify::{Event, EventKind, RecursiveMode, Watcher};

use crate::config::Config;
use crate::library::scanner::ScanOptions;

use super::LibraryScanner;

pub fn start(config: Arc<Config>, scanner: Arc<LibraryScanner>) {
    tokio::spawn(async move {
        if let Err(e) = run(config, scanner).await {
            tracing::error!("{:?}", e);
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
                Err(e) => tracing::error!("{:?}", e),
            }
        })?;

    for path in [&config.libraries.movies, &config.libraries.tv_shows] {
        watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
    }

    while let Some(event) = rx.recv().await {
        tracing::debug!(?event);

        match event.kind {
            | EventKind::Access(AccessKind::Close(AccessMode::Write))
            | EventKind::Create(_)
            | EventKind::Remove(_) => scanner.clone().start_scan(ScanOptions::quick()),
            _ => {}
        }
    }

    Ok(())
}
