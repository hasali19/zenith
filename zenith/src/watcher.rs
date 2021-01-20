use std::path::Path;
use std::time::Duration;

use notify::{RecommendedWatcher, Watcher};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
}

impl FileWatcher {
    pub fn spawn(mut callback: impl FnMut(notify::DebouncedEvent) + Send + 'static) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let watcher = notify::watcher(tx, Duration::from_secs(10)).unwrap();

        tokio::task::spawn_blocking(move || {
            while let Ok(event) = rx.recv() {
                callback(event);
            }
        });

        FileWatcher { watcher }
    }

    pub fn watch(&mut self, path: impl AsRef<Path>) {
        self.watcher
            .watch(path, notify::RecursiveMode::Recursive)
            .unwrap();
    }
}
