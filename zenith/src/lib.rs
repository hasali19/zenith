pub mod api;
pub mod config;
pub mod db;
pub mod ffmpeg;
pub mod fs;
pub mod library;
pub mod lifecycle;
pub mod metadata;
pub mod middleware;
pub mod sync;
pub mod tmdb;
pub mod transcoder;
pub mod utils;
pub mod watcher;

use std::sync::Arc;

use config::Config;
use db::Db;
use metadata::MetadataManager;
use sync::LibrarySync;
use transcoder::HlsTranscoder;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Db,
    pub sync: LibrarySync,
    pub metadata: MetadataManager,
    pub transcoder: Arc<HlsTranscoder>,
}
