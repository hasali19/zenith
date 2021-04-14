pub mod movies;
pub mod scanner;
pub mod shows;

pub use scanner::LibraryScanner;

use std::sync::Arc;

use crate::db::Db;
use crate::ffmpeg::VideoInfoProvider;

use self::movies::MovieLibrary;
use self::shows::ShowLibrary;

pub struct MediaLibrary {
    movies: MovieLibrary,
    shows: ShowLibrary,
}

impl MediaLibrary {
    pub fn new(db: Db, video_info: Arc<dyn VideoInfoProvider>) -> MediaLibrary {
        MediaLibrary {
            movies: MovieLibrary::new(db.clone(), video_info.clone()),
            shows: ShowLibrary::new(db, video_info),
        }
    }

    pub fn movies(&self) -> &MovieLibrary {
        &self.movies
    }

    pub fn shows(&self) -> &ShowLibrary {
        &self.shows
    }
}
