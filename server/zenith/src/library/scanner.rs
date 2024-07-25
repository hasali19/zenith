use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use db::media::MediaItemType;
use db::sql::{self, Join};
use db::Db;
use eyre::eyre;
use tokio::sync::Mutex;
use walkdir::WalkDir;

use super::{ChangeType, FileSystemChange, FileType};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait EventHandler: 'static + Send + Sync {
    async fn process_file_system_change(&self, change: FileSystemChange) -> eyre::Result<()>;
    async fn complete_library_scan(&self) -> eyre::Result<()>;
}

pub struct LibraryScanner {
    inner: Mutex<LibraryScannerImpl>,
}

struct LibraryScannerImpl {
    db: Db,
    library_paths: Vec<(VideoFileType, Utf8PathBuf)>,
    event_handler: Box<dyn EventHandler>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub fn new(
        db: Db,
        library_paths: Vec<(VideoFileType, Utf8PathBuf)>,
        event_handler: impl EventHandler,
    ) -> LibraryScanner {
        LibraryScanner {
            inner: Mutex::new(LibraryScannerImpl {
                db,
                library_paths,
                event_handler: Box::new(event_handler),
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

            log_error(scanner.scan_library_files()).await;

            let duration = Instant::now() - start_time;
            let seconds = duration.as_secs_f32();

            tracing::info!("completed scan in {seconds:.3}s");
        } else {
            tracing::info!("scan is already in progress");
        }
    }
}

impl LibraryScannerImpl {
    async fn scan_library_files(&self) -> eyre::Result<()> {
        let mut conn = self.db.acquire().await?;

        tracing::info!("validating video files");

        let sql = sql::select("video_files as v")
            .columns(&["v.id", "v.path", "v.scanned_at", "m.item_type"])
            .joins(&[Join::inner("media_items as m on m.id = v.item_id")])
            .to_sql();

        let video_files: Vec<(i64, Utf8PathBuf, i64, MediaItemType)> =
            sqlx::query_as(&sql).fetch_all(&mut *conn).await?;

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

                    self.event_handler
                        .process_file_system_change(change)
                        .await?;
                }
            } else {
                let change = FileSystemChange {
                    path: file_path,
                    file_type: FileType::Video(video_file_type),
                    change_type: ChangeType::Removed,
                };

                self.event_handler
                    .process_file_system_change(change)
                    .await?;
            }
        }

        tracing::info!("validating subtitles");

        let sql = sql::select("subtitles")
            .columns(&["path"])
            .condition("path IS NOT NULL")
            .to_sql();

        let subtitles: Vec<Utf8PathBuf> = sqlx::query_scalar(&sql).fetch_all(&mut *conn).await?;

        for path in subtitles {
            if let Err(e) = tokio::fs::metadata(&path).await
                && e.kind() == io::ErrorKind::NotFound
            {
                let change = FileSystemChange {
                    path,
                    file_type: FileType::Subtitle,
                    change_type: ChangeType::Removed,
                };

                self.event_handler
                    .process_file_system_change(change)
                    .await?;
            }
        }

        for (video_type, library_path) in &self.library_paths {
            tracing::info!(?video_type, %library_path, "scanning library");

            for file_path in get_video_files(library_path)? {
                let sql = sql::select("video_files")
                    .columns(&["id"])
                    .condition("path = ?")
                    .to_sql();

                let id: Option<i64> = sqlx::query_scalar(&sql)
                    .bind(&file_path)
                    .fetch_optional(&mut *conn)
                    .await?;

                if id.is_none() {
                    let change = FileSystemChange {
                        path: file_path,
                        file_type: FileType::Video(*video_type),
                        change_type: ChangeType::Added,
                    };

                    self.event_handler
                        .process_file_system_change(change)
                        .await?;
                }
            }

            for sub_path in get_subtitle_files(library_path)? {
                let sql = sql::select("subtitles")
                    .columns(&["id"])
                    .condition("path = ?")
                    .to_sql();

                let id: Option<i64> = sqlx::query_scalar(&sql)
                    .bind(&sub_path)
                    .fetch_optional(&mut *conn)
                    .await?;

                if id.is_none() {
                    let change = FileSystemChange {
                        path: sub_path,
                        file_type: FileType::Subtitle,
                        change_type: ChangeType::Added,
                    };

                    self.event_handler
                        .process_file_system_change(change)
                        .await?;
                }
            }
        }

        self.event_handler.complete_library_scan().await?;

        Ok(())
    }
}

fn get_video_files(path: &Utf8Path) -> eyre::Result<impl Iterator<Item = Utf8PathBuf>> {
    walk_dir_with_exts(path, &["mp4", "mkv"])
}

fn get_subtitle_files(path: &Utf8Path) -> eyre::Result<impl Iterator<Item = Utf8PathBuf>> {
    walk_dir_with_exts(path, &["vtt", "srt"])
}

fn walk_dir_with_exts<'a>(
    path: &Utf8Path,
    extensions: &'a [&str],
) -> eyre::Result<impl Iterator<Item = Utf8PathBuf> + 'a> {
    if !path.is_dir() {
        return Err(eyre!("directory does not exist: {path}"));
    }

    Ok(WalkDir::new(path)
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
        }))
}

#[cfg(test)]
mod tests {
    use db::subtitles::NewSubtitle;
    use sqlx::SqliteConnection;
    use tempfile::TempDir;
    use uuid::Uuid;

    use super::*;

    async fn test_db() -> Db {
        let id = Uuid::new_v4();
        Db::init(&format!("file:zenith_{id}?mode=memory&cache=shared"))
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn scan_movie_with_subtitle() -> eyre::Result<()> {
        let db = test_db().await;
        let tmp = TempDir::new()?;

        tokio::fs::write(tmp.path().join("Test Movie (2023).mkv"), &[]).await?;
        tokio::fs::write(tmp.path().join("Test Movie (2023).srt"), &[]).await?;

        let mut event_handler = MockEventHandler::new();

        event_handler
            .expect_process_file_system_change()
            .withf(|change| {
                change.change_type == ChangeType::Added
                    && change.file_type == FileType::Video(VideoFileType::Movie)
            })
            .times(1)
            .returning(|_| Ok(()));

        event_handler
            .expect_process_file_system_change()
            .withf(|change| {
                change.change_type == ChangeType::Added && change.file_type == FileType::Subtitle
            })
            .times(1)
            .returning(|_| Ok(()));

        event_handler
            .expect_complete_library_scan()
            .once()
            .returning(|| Ok(()));

        let library_paths = vec![(VideoFileType::Movie, tmp.path().to_owned().try_into()?)];

        let scanner = Arc::new(LibraryScanner::new(
            db.clone(),
            library_paths,
            event_handler,
        ));

        scanner.run_scan().await;

        Ok(())
    }

    #[tokio::test]
    async fn scan_updated_movie() -> eyre::Result<()> {
        let db = test_db().await;
        let tmp = TempDir::new()?;
        let movie_path = tmp.path().join("Test Movie (2023).mkv");

        {
            let mut conn = db.acquire().await?;
            insert_movie(&mut conn, "Test Movie", movie_path.as_path().try_into()?).await?;
        }

        tokio::fs::write(movie_path, &[]).await?;

        let mut event_handler = MockEventHandler::new();

        event_handler
            .expect_process_file_system_change()
            .withf(|change| {
                change.change_type == ChangeType::Modified
                    && change.file_type == FileType::Video(VideoFileType::Movie)
            })
            .times(1)
            .returning(|_| Ok(()));

        event_handler
            .expect_complete_library_scan()
            .once()
            .returning(|| Ok(()));

        let library_paths = vec![(VideoFileType::Movie, tmp.path().to_owned().try_into()?)];

        let scanner = Arc::new(LibraryScanner::new(
            db.clone(),
            library_paths,
            event_handler,
        ));

        scanner.run_scan().await;

        Ok(())
    }

    #[tokio::test]
    async fn scan_removed_movie() -> eyre::Result<()> {
        let db = test_db().await;
        let tmp = TempDir::new()?;
        let movie_path = tmp.path().join("Test Movie (2023).mkv");
        let sub_path = tmp.path().join("Test Movie (2023).srt");

        {
            let mut conn = db.acquire().await?;
            let (_, video_id) =
                insert_movie(&mut conn, "Test Movie", movie_path.as_path().try_into()?).await?;
            insert_subtitle(&mut conn, video_id, sub_path.as_path().try_into()?).await?;
        }

        let mut event_handler = MockEventHandler::new();

        event_handler
            .expect_process_file_system_change()
            .withf(|change| {
                change.change_type == ChangeType::Removed
                    && change.file_type == FileType::Video(VideoFileType::Movie)
            })
            .times(1)
            .returning(|_| Ok(()));

        event_handler
            .expect_process_file_system_change()
            .withf(|change| {
                change.change_type == ChangeType::Removed && change.file_type == FileType::Subtitle
            })
            .times(1)
            .returning(|_| Ok(()));

        event_handler
            .expect_complete_library_scan()
            .once()
            .returning(|| Ok(()));

        let library_paths = vec![(VideoFileType::Movie, tmp.path().to_owned().try_into()?)];

        let scanner = Arc::new(LibraryScanner::new(
            db.clone(),
            library_paths,
            event_handler,
        ));

        scanner.run_scan().await;

        Ok(())
    }

    async fn insert_movie(
        conn: &mut SqliteConnection,
        name: &str,
        path: &Utf8Path,
    ) -> eyre::Result<(i64, i64)> {
        let sql = sql::insert("media_items")
            .columns(&["item_type", "name"])
            .values(&["?", "?"])
            .returning(&["id"])
            .to_sql();

        let item_id: i64 = sqlx::query_scalar(&sql)
            .bind(MediaItemType::Movie)
            .bind(name)
            .fetch_one(&mut *conn)
            .await?;

        let sql = sql::insert("video_files")
            .columns(&["item_id", "path", "scanned_at"])
            .values(&["?", "?", "strftime('%s')"])
            .returning(&["id"])
            .to_sql();

        let video_id: i64 = sqlx::query_scalar(&sql)
            .bind(item_id)
            .bind(path)
            .fetch_one(&mut *conn)
            .await?;

        Ok((item_id, video_id))
    }

    async fn insert_subtitle(
        conn: &mut SqliteConnection,
        video_id: i64,
        path: &Utf8Path,
    ) -> eyre::Result<()> {
        let sub = NewSubtitle {
            video_id,
            path: Some(path),
            stream_index: None,
            title: None,
            language: None,
            format: None,
            forced: false,
            sdh: false,
        };

        db::subtitles::insert(conn, &sub).await?;

        Ok(())
    }

    #[tokio::test]
    async fn fail_if_library_path_doesnt_exist() {
        let db = test_db().await;
        let scanner = LibraryScannerImpl {
            db,
            library_paths: vec![(VideoFileType::Movie, Utf8PathBuf::from("path/doesnt/exist"))],
            event_handler: Box::new(MockEventHandler::new()),
        };

        assert!(scanner.scan_library_files().await.is_err());
    }
}
