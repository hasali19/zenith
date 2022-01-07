pub mod movies;
pub mod scanner;
pub mod shows;
pub mod video_info;
pub mod watcher;

pub use scanner::LibraryScanner;

use std::sync::Arc;

use crate::db::Db;
use crate::video_prober::VideoProber;

use self::movies::MovieLibrary;
use self::shows::ShowLibrary;

pub struct MediaLibrary {
    movies: MovieLibrary,
    shows: ShowLibrary,
}

impl MediaLibrary {
    pub fn new(db: Db, video_prober: Arc<dyn VideoProber>) -> MediaLibrary {
        MediaLibrary {
            movies: MovieLibrary::new(db.clone(), video_prober.clone()),
            shows: ShowLibrary::new(db, video_prober),
        }
    }

    pub fn movies(&self) -> &MovieLibrary {
        &self.movies
    }

    pub fn shows(&self) -> &ShowLibrary {
        &self.shows
    }
}
