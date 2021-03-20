use std::collections::HashSet;
use std::path::Path;

use regex::Regex;

use crate::ffmpeg::VideoInfoProvider;
use crate::fs::{DirEntryType, FileSystem};
use crate::library::{MediaLibrary, NewMovie};

pub(super) async fn sync_movies(
    library: &impl MediaLibrary,
    fs: &impl FileSystem,
    video_info: &impl VideoInfoProvider,
    path: &str,
) -> eyre::Result<()> {
    // Find all existing movies
    let mut movies = library
        .get_movie_ids()
        .await?
        .into_iter()
        .collect::<HashSet<_>>();

    // Search for all movies on the filesystem
    for entry in fs.list_dir(Path::new(path))? {
        if !matches!(entry.entry_type, DirEntryType::Directory) {
            continue;
        }

        for entry in fs.list_dir(&entry.path)? {
            if !matches!(entry.entry_type, DirEntryType::File) {
                continue;
            }

            let file_path = entry.path;
            let file_name = match file_path.file_name().and_then(|v| v.to_str()) {
                Some(name) => name,
                None => continue,
            };

            let (title, year) = match parse_movie_file_name(file_name) {
                Some(v) => v,
                None => continue,
            };

            let path = match file_path.to_str() {
                Some(path) => path,
                None => continue,
            };

            match library.get_movie_id(&path).await? {
                // Remove from the set of ids if the movie exists
                Some(id) => {
                    movies.remove(&id);
                }
                // Otherwise create a new movie
                None => {
                    tracing::info!("adding movie: {}", file_name);

                    let info = match video_info.get_video_info(&path).await {
                        Ok(info) => info,
                        Err(e) => {
                            tracing::warn!("{}", e);
                            continue;
                        }
                    };

                    let release_date = year
                        .and_then(|year| time::Date::try_from_yo(year, 1).ok())
                        .and_then(|date| date.try_with_hms(0, 0, 0).ok())
                        .map(|dt| dt.assume_utc().unix_timestamp());

                    let movie = NewMovie {
                        path,
                        title: &title,
                        release_date,
                        duration: info.duration,
                    };

                    library.add_movie(movie).await?;
                }
            }
        }
    }

    // Any remaining ids no longer exist on the filesystem, so
    // may be deleted from the database
    for id in movies {
        tracing::info!("removing movie: {}", id);
        library.remove_movie(id).await?;
    }

    Ok(())
}

pub fn parse_movie_file_name(name: &str) -> Option<(String, Option<i32>)> {
    lazy_static::lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^(\S.*?)(?: \((\d\d\d\d)\))?\.(mkv|mp4)$").unwrap();
    }

    REGEX.captures(name).map(|captures| {
        let name = captures.get(1).unwrap().as_str().to_owned();
        let year = captures
            .get(2)
            .map(|m| m.as_str().parse::<i32>().ok())
            .flatten();

        (name, year)
    })
}
