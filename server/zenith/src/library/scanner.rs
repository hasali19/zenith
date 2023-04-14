use std::io;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use camino::{Utf8Path, Utf8PathBuf};
use time::Instant;
use tokio::sync::Mutex;
use walkdir::WalkDir;

use crate::config::Config;
use crate::db::media::MediaItemType;
use crate::db::Db;
use crate::sql::{self, Join};

use super::{ChangeType, FileSystemChange, FileType, MediaLibrary};

pub struct LibraryScanner {
    inner: Mutex<LibraryScannerImpl>,
}

struct LibraryScannerImpl {
    db: Db,
    config: Arc<Config>,
    library: Arc<MediaLibrary>,
}

#[derive(Clone, Copy, Debug)]
pub enum VideoFileType {
    Movie,
    Episode,
}

#[derive(Debug)]
pub enum FileScanResult {
    Added(i64),
    Updated(i64),
    Removed,
    Ignored,
}

impl FileScanResult {
    pub fn id(&self) -> Option<i64> {
        match self {
            FileScanResult::Added(id) | FileScanResult::Updated(id) => Some(*id),
            FileScanResult::Removed | FileScanResult::Ignored => None,
        }
    }
}

impl LibraryScanner {
    pub fn new(db: Db, library: Arc<MediaLibrary>, config: Arc<Config>) -> LibraryScanner {
        LibraryScanner {
            inner: Mutex::new(LibraryScannerImpl {
                db,
                config,
                library,
            }),
        }
    }

    /// Starts a library scan if one is not already running.
    ///
    /// Returns immediately without waiting for the scan to finish.
    pub fn start_scan(self: Arc<Self>) {
        tokio::spawn(async move { self.run_scan().await });
    }

    /// Runs a library scan if one is not already running.
    ///
    /// This will validate libraries by checking that existing media items still exist on disk, and
    /// scan the disk for new files to be added to the library.
    ///
    /// If a scan is already running, this will return immediately without waiting for it
    /// to finish.
    #[tracing::instrument(skip(self))]
    pub async fn run_scan(self: Arc<Self>) {
        if let Ok(scanner) = self.inner.try_lock() {
            let start_time = Instant::now();

            tracing::info!("starting library scan");

            async fn log_error(fut: impl std::future::Future<Output = eyre::Result<()>>) {
                if let Err(e) = fut.await {
                    tracing::error!("{e:?}");
                }
            }

            log_error(scanner.scan_library_files(&[
                (VideoFileType::Movie, &scanner.config.libraries.movies),
                (VideoFileType::Episode, &scanner.config.libraries.tv_shows),
            ]))
            .await;

            let duration = Instant::now() - start_time;
            let seconds = duration.as_seconds_f32();

            tracing::info!("completed scan in {seconds:.3}s");
        } else {
            tracing::info!("scan is already in progress");
        }
    }
}

impl LibraryScannerImpl {
    async fn scan_library_files(
        &self,
        library_paths: &[(VideoFileType, &Utf8Path)],
    ) -> eyre::Result<()> {
        let mut conn = self.db.acquire().await?;

        tracing::info!("validating video files");

        let sql = sql::select("video_files as v")
            .columns(&["v.id", "v.path", "v.scanned_at", "m.item_type"])
            .joins(&[Join::inner("media_items as m on m.id = v.item_id")])
            .to_sql();

        let video_files: Vec<(i64, Utf8PathBuf, i64, MediaItemType)> =
            sqlx::query_as(&sql).fetch_all(&mut conn).await?;

        for (id, file_path, scanned_at, item_type) in video_files {
            let video_file_type = match item_type {
                MediaItemType::Movie => VideoFileType::Movie,
                MediaItemType::Episode => VideoFileType::Episode,
                _ => eyre::bail!("video file {id} has unexpected media type {item_type:?}"),
            };

            if file_path.is_file() {
                let metadata = tokio::fs::metadata(&file_path).await?;
                let scanned_at = SystemTime::UNIX_EPOCH + Duration::from_secs(scanned_at as u64);
                if metadata.modified()? > scanned_at {
                    let change = FileSystemChange {
                        path: file_path,
                        file_type: FileType::Video(video_file_type),
                        change_type: ChangeType::Modified,
                    };

                    self.library.process_file_system_change(change).await?;
                }
            } else {
                let change = FileSystemChange {
                    path: file_path,
                    file_type: FileType::Video(video_file_type),
                    change_type: ChangeType::Removed,
                };

                self.library.process_file_system_change(change).await?;
            }
        }

        tracing::info!("validating subtitles");

        let sql = sql::select("subtitles")
            .columns(&["path"])
            .condition("path IS NOT NULL")
            .to_sql();

        let subtitles: Vec<Utf8PathBuf> = sqlx::query_scalar(&sql).fetch_all(&mut conn).await?;

        for path in subtitles {
            if let Err(e) = tokio::fs::metadata(&path).await
                    && e.kind() == io::ErrorKind::NotFound
                {
                    let change = FileSystemChange {
                        path,
                        file_type: FileType::Subtitle,
                        change_type: ChangeType::Removed,
                    };

                    self.library.process_file_system_change(change).await?;
                }
        }

        for &(video_type, library_path) in library_paths {
            tracing::info!(?video_type, %library_path, "scanning library");

            for file_path in get_video_files(library_path) {
                let sql = sql::select("video_files")
                    .columns(&["id"])
                    .condition("path = ?")
                    .to_sql();

                let id: Option<i64> = sqlx::query_scalar(&sql)
                    .bind(&file_path)
                    .fetch_optional(&mut conn)
                    .await?;

                if id.is_none() {
                    let change = FileSystemChange {
                        path: file_path,
                        file_type: FileType::Video(video_type),
                        change_type: ChangeType::Added,
                    };

                    self.library.process_file_system_change(change).await?;
                }
            }

            for sub_path in get_subtitle_files(library_path) {
                let sql = sql::select("subtitles")
                    .columns(&["id"])
                    .condition("path = ?")
                    .to_sql();

                let id: Option<i64> = sqlx::query_scalar(&sql)
                    .bind(&sub_path)
                    .fetch_optional(&mut conn)
                    .await?;

                if id.is_none() {
                    let change = FileSystemChange {
                        path: sub_path,
                        file_type: FileType::Subtitle,
                        change_type: ChangeType::Added,
                    };

                    self.library.process_file_system_change(change).await?;
                }
            }
        }

        self.library.validate().await?;

        Ok(())
    }
}

fn get_video_files(path: &Utf8Path) -> impl Iterator<Item = Utf8PathBuf> {
    walk_dir_with_exts(path, &["mp4", "mkv"])
}

fn get_subtitle_files(path: &Utf8Path) -> impl Iterator<Item = Utf8PathBuf> {
    walk_dir_with_exts(path, &["vtt", "srt"])
}

fn walk_dir_with_exts<'a>(
    path: &Utf8Path,
    extensions: &'a [&str],
) -> impl Iterator<Item = Utf8PathBuf> + 'a {
    WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter(|it| it.file_type().is_file())
        .filter_map(|e| {
            let path = Utf8PathBuf::from_path_buf(e.into_path()).ok()?;
            let ext = path.extension()?;
            if extensions.contains(&ext) {
                Some(path)
            } else {
                None
            }
        })
}
