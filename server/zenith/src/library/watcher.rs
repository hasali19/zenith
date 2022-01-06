use std::path::Path;
use std::sync::Arc;

use eyre::eyre;
use notify::event::{AccessKind, AccessMode};
use notify::{Event, EventKind, RecursiveMode, Watcher};

use crate::config::Config;
use crate::library::scanner::ScanOptions;

use super::scanner::VideoFileType;
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

    let movies_lib = Path::new(&config.libraries.movies).canonicalize()?;
    let shows_lib = Path::new(&config.libraries.tv_shows).canonicalize()?;

    for path in [&movies_lib, &shows_lib] {
        watcher.watch(path, RecursiveMode::Recursive)?;
    }

    while let Some(event) = rx.recv().await {
        tracing::debug!(?event);

        match event.kind {
            | EventKind::Access(AccessKind::Close(AccessMode::Write))
            | EventKind::Create(_)
            | EventKind::Remove(_) => {
                // Rescan all files associated with the event
                for path in event.paths {
                    let path = match path.canonicalize() {
                        Ok(path) => path,
                        Err(_) => continue,
                    };

                    let file_type = if path.starts_with(&movies_lib) {
                        VideoFileType::Movie
                    } else if path.starts_with(&shows_lib) {
                        VideoFileType::Episode
                    } else {
                        continue;
                    };

                    scanner
                        .scan_file(file_type, path, ScanOptions::rescan_files())
                        .await?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
