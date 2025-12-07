mod movies;
mod parser;
pub mod scanner;
mod shows;
mod subtitles;
#[cfg(test)]
mod tests;
mod videos;
pub mod watcher;

use camino::Utf8PathBuf;
use db::media::MediaItemType;
use db::{Db, WriteConnection};
pub use scanner::LibraryScanner;
use tokio::sync::broadcast;

use std::sync::Arc;

use crate::config::ImportMatcher;
use crate::video_prober::VideoProber;

use self::parser::PathParser;
use self::scanner::VideoFileType;

#[derive(Clone)]
pub enum LibraryEvent {
    MediaAdded(MediaItemType, i64),
    MediaRemoved(MediaItemType, i64),
    VideoAdded { media_id: i64, video_id: i64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeType {
    Added,
    Modified,
    Removed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    Video(VideoFileType),
    Subtitle,
}

#[derive(Debug)]
pub struct FileSystemChange {
    path: Utf8PathBuf,
    file_type: FileType,
    change_type: ChangeType,
}

pub struct MediaLibrary {
    db: Db,
    matchers: Vec<ImportMatcher>,
    video_prober: Arc<dyn VideoProber>,
    notifier: broadcast::Sender<LibraryEvent>,
}

impl MediaLibrary {
    pub fn new(
        db: Db,
        matchers: Vec<ImportMatcher>,
        video_prober: Arc<dyn VideoProber>,
    ) -> MediaLibrary {
        MediaLibrary {
            db,
            matchers,
            video_prober,
            notifier: broadcast::channel(8).0,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<LibraryEvent> {
        self.notifier.subscribe()
    }

    pub async fn process_file_system_change(&self, change: FileSystemChange) -> eyre::Result<()> {
        match change.file_type {
            FileType::Video(video_type) => match change.change_type {
                ChangeType::Added => self.import_video(video_type, &change.path).await?,
                ChangeType::Modified => self.rescan_video(&change.path).await?,
                ChangeType::Removed => self.remove_video(&change.path).await?,
            },
            FileType::Subtitle => match change.change_type {
                ChangeType::Added => self.import_subtitle(&change.path).await?,
                ChangeType::Modified => self.rescan_subtitle(&change.path).await?,
                ChangeType::Removed => self.remove_subtitle(&change.path).await?,
            },
        }

        Ok(())
    }

    pub async fn validate(&self) -> eyre::Result<()> {
        self.validate_movies().await?;
        self.validate_shows().await?;
        Ok(())
    }

    async fn remove_item(
        &self,
        conn: &mut WriteConnection,
        id: i64,
        media_type: MediaItemType,
    ) -> eyre::Result<()> {
        let mut transaction = conn.begin().await?;
        db::items::remove(&mut transaction, id).await?;
        transaction.commit().await?;
        let _ = self
            .notifier
            .send(LibraryEvent::MediaRemoved(media_type, id));
        Ok(())
    }

    fn parser(&self) -> PathParser<'_> {
        PathParser::new(&self.matchers)
    }
}
