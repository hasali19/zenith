use std::collections::HashMap;
use std::sync::Arc;

use camino::Utf8PathBuf;
use db::subtitles::Subtitle;
use db::video_files::VideoFile;
use pretty_assertions::assert_eq;
use serde_json::json;
use sqlx::FromRow;
use time::macros::datetime;
use uuid::Uuid;

use crate::config::Config;
use crate::library::scanner::VideoFileType;
use crate::library::{ChangeType, FileSystemChange, FileType, MediaLibrary};
use crate::video_prober::{Format, MockVideoProber, Stream, StreamTags, VideoInfo};
use crate::{Db, MediaItemType};

#[derive(FromRow)]
struct MediaItem {
    id: i64,
    item_type: MediaItemType,
    name: String,
    start_date: Option<i64>,
    parent_id: Option<i64>,
    parent_index: Option<u32>,
    grandparent_id: Option<i64>,
    grandparent_index: Option<u32>,
}

async fn test_db() -> Db {
    let id = Uuid::new_v4();
    Db::init(&format!("file:zenith_{id}?mode=memory&cache=shared"))
        .await
        .unwrap()
}

#[tokio::test]
async fn import_movie() -> eyre::Result<()> {
    let db = test_db().await;
    let config: Config = serde_yaml::from_str(include_str!("config.yml"))?;

    let mut video_prober = MockVideoProber::new();

    video_prober.expect_probe().returning(|path| {
        assert_eq!(
            path,
            "/media/movies/Movie Name (2023)/Movie Name (2023).mkv"
        );

        Ok(VideoInfo {
            format: Format {
                duration: "1000.0".to_owned(),
                format_name: "matroska".to_owned(),
            },
            streams: vec![
                Stream {
                    index: 0,
                    codec_type: "video".to_owned(),
                    codec_name: Some("h264".to_owned()),
                    width: Some(1920),
                    height: Some(1080),
                    channels: None,
                    channel_layout: None,
                    tags: None,
                    properties: HashMap::new(),
                },
                Stream {
                    index: 1,
                    codec_type: "audio".to_owned(),
                    codec_name: Some("aac".to_owned()),
                    width: None,
                    height: None,
                    channels: Some(2),
                    channel_layout: Some("stereo".to_owned()),
                    tags: Some(StreamTags {
                        title: None,
                        language: Some("eng".to_owned()),
                    }),
                    properties: HashMap::new(),
                },
                Stream {
                    index: 1,
                    codec_type: "subtitle".to_owned(),
                    codec_name: Some("subrip".to_owned()),
                    width: None,
                    height: None,
                    channels: None,
                    channel_layout: None,
                    tags: Some(StreamTags {
                        title: Some("English (SDH)".to_owned()),
                        language: Some("eng".to_owned()),
                    }),
                    properties: {
                        let mut props = HashMap::new();
                        props.insert(
                            "disposition".to_owned(),
                            json!({
                                "hearing_impaired": 0,
                                "forced": 0,
                            }),
                        );
                        props
                    },
                },
            ],
        })
    });

    let library = MediaLibrary::new(db.clone(), config.import.matchers, Arc::new(video_prober));

    library
        .process_file_system_change(FileSystemChange {
            path: Utf8PathBuf::from("/media/movies/Movie Name (2023)/Movie Name (2023).mkv"),
            file_type: FileType::Video(VideoFileType::Movie),
            change_type: ChangeType::Added,
        })
        .await?;

    let mut conn = db.acquire().await?;

    let media_items: Vec<MediaItem> = sqlx::query_as("SELECT * FROM media_items")
        .fetch_all(&mut *conn)
        .await?;

    assert_eq!(media_items.len(), 1);
    let row = media_items.first().unwrap();

    assert_eq!(row.item_type, MediaItemType::Movie);
    assert_eq!(row.name, "Movie Name");
    assert_eq!(
        row.start_date,
        Some(datetime!(2023-01-01 0:00 UTC).unix_timestamp())
    );

    let video_files: Vec<VideoFile> = sqlx::query_as("SELECT * FROM video_files")
        .fetch_all(&mut *conn)
        .await?;

    assert_eq!(video_files.len(), 1);
    let row = video_files.first().unwrap();

    assert_eq!(
        row.path,
        "/media/movies/Movie Name (2023)/Movie Name (2023).mkv"
    );

    assert_eq!(
        row.path_stem,
        "/media/movies/Movie Name (2023)/Movie Name (2023)"
    );

    assert_eq!(row.duration, Some(1000.0));
    assert_eq!(row.format_name.as_deref(), Some("matroska"));

    Ok(())
}

#[tokio::test]
async fn import_movie_with_multiple_files() -> eyre::Result<()> {
    let db = test_db().await;
    let config: Config = serde_yaml::from_str(include_str!("config.yml"))?;

    let mut video_prober = MockVideoProber::new();

    video_prober.expect_probe().returning(|_path| {
        Ok(VideoInfo {
            format: Format {
                duration: "1000.0".to_owned(),
                format_name: "matroska".to_owned(),
            },
            streams: vec![],
        })
    });

    let library = MediaLibrary::new(db.clone(), config.import.matchers, Arc::new(video_prober));

    library
        .process_file_system_change(FileSystemChange {
            path: Utf8PathBuf::from("/media/movies/Movie Name (2023)/Movie Name (2023) - 720p.mkv"),
            file_type: FileType::Video(VideoFileType::Movie),
            change_type: ChangeType::Added,
        })
        .await?;

    let mut conn = db.acquire().await?;

    let media_items: Vec<MediaItem> = sqlx::query_as("SELECT * FROM media_items")
        .fetch_all(&mut *conn)
        .await?;

    assert_eq!(media_items.len(), 1);
    let original_movie = media_items.first().unwrap();

    library
        .process_file_system_change(FileSystemChange {
            path: Utf8PathBuf::from(
                "/media/movies/Movie Name (2023)/Movie Name (2023) - 1080p.mkv",
            ),
            file_type: FileType::Video(VideoFileType::Movie),
            change_type: ChangeType::Added,
        })
        .await?;

    let media_items: Vec<MediaItem> = sqlx::query_as("SELECT * FROM media_items")
        .fetch_all(&mut *conn)
        .await?;

    assert_eq!(media_items.len(), 1);
    let movie = media_items.first().unwrap();

    assert_eq!(movie.id, original_movie.id);

    let video_files: Vec<VideoFile> = sqlx::query_as("SELECT * FROM video_files")
        .fetch_all(&mut *conn)
        .await?;

    assert_eq!(video_files.len(), 2);
    assert!(video_files.iter().all(|f| f.item_id == movie.id));

    Ok(())
}

#[tokio::test]
async fn remove_movie_with_no_video_files() -> eyre::Result<()> {
    let db = test_db().await;
    let config: Config = serde_yaml::from_str(include_str!("config.yml"))?;
    let video_prober = MockVideoProber::new();
    let library = MediaLibrary::new(db.clone(), config.import.matchers, Arc::new(video_prober));

    let mut conn = db.acquire().await?;

    let sql = "
        INSERT INTO media_items (item_type, name)
        VALUES (?, ?)
    ";

    sqlx::query(sql)
        .bind(MediaItemType::Movie)
        .bind("Movie")
        .execute(&mut *conn)
        .await?;

    let item_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM media_items")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(item_count, 1);

    library.validate_movies().await?;

    let item_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM media_items")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(item_count, 0);

    Ok(())
}

#[tokio::test]
async fn import_episode() -> eyre::Result<()> {
    let db = test_db().await;
    let config: Config = serde_yaml::from_str(include_str!("config.yml"))?;

    let mut video_prober = MockVideoProber::new();

    video_prober.expect_probe().returning(|path| {
        assert_eq!(path, "/media/shows/Show Name/S02E06.mkv");

        Ok(VideoInfo {
            format: Format {
                duration: "1000.0".to_owned(),
                format_name: "matroska".to_owned(),
            },
            streams: vec![],
        })
    });

    let library = MediaLibrary::new(db.clone(), config.import.matchers, Arc::new(video_prober));

    library
        .process_file_system_change(FileSystemChange {
            path: Utf8PathBuf::from("/media/shows/Show Name/S02E06.mkv"),
            file_type: FileType::Video(VideoFileType::Episode),
            change_type: ChangeType::Added,
        })
        .await?;

    let mut conn = db.acquire().await?;

    let media_items: Vec<MediaItem> = sqlx::query_as("SELECT * FROM media_items")
        .fetch_all(&mut *conn)
        .await?;

    let show = media_items
        .iter()
        .find(|it| it.item_type == MediaItemType::Show)
        .unwrap();

    assert_eq!(show.name, "Show Name");

    let season = media_items
        .iter()
        .find(|it| it.item_type == MediaItemType::Season)
        .unwrap();

    assert_eq!(season.name, "Season 2");
    assert_eq!(season.parent_id, Some(show.id));
    assert_eq!(season.parent_index, Some(2));

    let episode = media_items
        .iter()
        .find(|it| it.item_type == MediaItemType::Episode)
        .unwrap();

    assert_eq!(episode.name, "S02E06");
    assert_eq!(episode.parent_id, Some(season.id));
    assert_eq!(episode.parent_index, Some(6));
    assert_eq!(episode.grandparent_id, Some(show.id));
    assert_eq!(episode.grandparent_index, Some(2));

    let video_files: Vec<VideoFile> = sqlx::query_as("SELECT * FROM video_files")
        .fetch_all(&mut *conn)
        .await?;

    assert_eq!(video_files.len(), 1);
    let row = video_files.first().unwrap();

    assert_eq!(row.item_id, episode.id);
    assert_eq!(row.path, "/media/shows/Show Name/S02E06.mkv");
    assert_eq!(row.path_stem, "/media/shows/Show Name/S02E06");
    assert_eq!(row.duration, Some(1000.0));
    assert_eq!(row.format_name.as_deref(), Some("matroska"));

    Ok(())
}

#[tokio::test]
async fn remove_show_with_empty_season() -> eyre::Result<()> {
    let db = test_db().await;
    let config: Config = serde_yaml::from_str(include_str!("config.yml"))?;
    let video_prober = MockVideoProber::new();
    let library = MediaLibrary::new(db.clone(), config.import.matchers, Arc::new(video_prober));

    let mut conn = db.acquire().await?;

    let sql = "
        INSERT INTO media_items (item_type, name)
        VALUES (?, ?)
    ";

    sqlx::query(sql)
        .bind(MediaItemType::Show)
        .bind("Show")
        .execute(&mut *conn)
        .await?;

    let sql = "
        INSERT INTO media_items (item_type, name, parent_id, parent_index)
        VALUES (?, ?, ?, ?)
    ";

    sqlx::query(sql)
        .bind(MediaItemType::Season)
        .bind("Season")
        .bind(1)
        .bind(1)
        .execute(&mut *conn)
        .await?;

    let item_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM media_items")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(item_count, 2);

    library.validate_shows().await?;

    let item_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM media_items")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(item_count, 0);

    Ok(())
}

#[tokio::test]
async fn import_subtitle() -> eyre::Result<()> {
    let db = test_db().await;
    let config: Config = serde_yaml::from_str(include_str!("config.yml"))?;

    let mut video_prober = MockVideoProber::new();

    video_prober.expect_probe().returning(|path| {
        assert_eq!(path, "/media/shows/Show Name/S02E06.mkv");

        Ok(VideoInfo {
            format: Format {
                duration: "1000.0".to_owned(),
                format_name: "matroska".to_owned(),
            },
            streams: vec![],
        })
    });

    let library = MediaLibrary::new(db.clone(), config.import.matchers, Arc::new(video_prober));

    library
        .process_file_system_change(FileSystemChange {
            path: Utf8PathBuf::from("/media/shows/Show Name/S02E06.mkv"),
            file_type: FileType::Video(VideoFileType::Episode),
            change_type: ChangeType::Added,
        })
        .await?;

    library
        .process_file_system_change(FileSystemChange {
            path: Utf8PathBuf::from("/media/shows/Show Name/S02E06.en.sdh.vtt"),
            file_type: FileType::Subtitle,
            change_type: ChangeType::Added,
        })
        .await?;

    let mut conn = db.acquire().await?;

    let subtitles: Vec<Subtitle> = sqlx::query_as("SELECT * FROM subtitles")
        .fetch_all(&mut *conn)
        .await?;

    assert_eq!(subtitles.len(), 1);
    let row = subtitles.first().unwrap();

    assert_eq!(row.stream_index, None);
    assert_eq!(
        row.path,
        Some(Utf8PathBuf::from(
            "/media/shows/Show Name/S02E06.en.sdh.vtt"
        ))
    );
    assert_eq!(
        row.title.as_deref(),
        Some("/media/shows/Show Name/S02E06.en.sdh.vtt")
    );
    assert_eq!(row.language.as_deref(), Some("en"));
    assert_eq!(row.format.as_deref(), Some("webvtt"));
    assert_eq!(row.sdh, true);
    assert_eq!(row.forced, false);

    Ok(())
}

#[tokio::test]
async fn remove_subtitle_with_missing_file() -> eyre::Result<()> {
    let db = test_db().await;
    let config: Config = serde_yaml::from_str(include_str!("config.yml"))?;

    let mut video_prober = MockVideoProber::new();

    video_prober.expect_probe().returning(|path| {
        assert_eq!(path, "/media/shows/Show Name/S02E06.mkv");

        Ok(VideoInfo {
            format: Format {
                duration: "1000.0".to_owned(),
                format_name: "matroska".to_owned(),
            },
            streams: vec![],
        })
    });

    let library = MediaLibrary::new(db.clone(), config.import.matchers, Arc::new(video_prober));

    library
        .process_file_system_change(FileSystemChange {
            path: Utf8PathBuf::from("/media/shows/Show Name/S02E06.mkv"),
            file_type: FileType::Video(VideoFileType::Episode),
            change_type: ChangeType::Added,
        })
        .await?;

    library
        .process_file_system_change(FileSystemChange {
            path: Utf8PathBuf::from("/media/shows/Show Name/S02E06.en.sdh.vtt"),
            file_type: FileType::Subtitle,
            change_type: ChangeType::Added,
        })
        .await?;

    let mut conn = db.acquire().await?;

    let sub_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM subtitles")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(sub_count, 1);

    library
        .process_file_system_change(FileSystemChange {
            path: Utf8PathBuf::from("/media/shows/Show Name/S02E06.en.sdh.vtt"),
            file_type: FileType::Subtitle,
            change_type: ChangeType::Removed,
        })
        .await?;

    let sub_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM subtitles")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(sub_count, 0);

    Ok(())
}
