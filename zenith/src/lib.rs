pub mod api;
pub mod config;
pub mod db;
pub mod ffmpeg;
pub mod lifecycle;
pub mod metadata;
pub mod middleware;
pub mod sync;
pub mod tmdb;
pub mod utils;
pub mod watcher;

use std::sync::Arc;

use config::Config;
use db::Db;
use sync::LibrarySync;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Db,
    pub sync: LibrarySync,
}
