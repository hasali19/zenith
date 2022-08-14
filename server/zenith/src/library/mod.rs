pub mod movies;
pub mod scanner;
pub mod shows;
pub mod video_info;
pub mod watcher;

pub use scanner::LibraryScanner;
use tokio::sync::broadcast;

use std::sync::Arc;

use crate::db::media::MediaItemType;
use crate::db::Db;
use crate::video_prober::VideoProber;

#[derive(Clone)]
pub enum LibraryEvent {
    Added(MediaItemType, i64),
    Removed(MediaItemType, i64),
}

pub struct MediaLibrary {
    db: Db,
    video_prober: Arc<dyn VideoProber>,
    notifier: broadcast::Sender<LibraryEvent>,
}

impl MediaLibrary {
    pub fn new(db: Db, video_prober: Arc<dyn VideoProber>) -> MediaLibrary {
        MediaLibrary {
            db,
            video_prober,
            notifier: broadcast::channel(8).0,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<LibraryEvent> {
        self.notifier.subscribe()
    }
}
