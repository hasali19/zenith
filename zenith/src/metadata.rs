use eyre::eyre;
use sqlx::SqliteConnection;
use time::{format_description, Date, Time};
use tokio::sync::mpsc;

use crate::db::media::{MediaImage, MediaImageSrcType, MediaImageType};
use crate::db::{self, Db};
use crate::library::scanner;
use crate::tmdb::{MovieSearchQuery, TmdbClient, TvShowSearchQuery};

#[derive(Debug)]
pub enum RefreshRequest {
    Movie(i64),
    TvShow(i64),
    TvSeason(i64),
    TvEpisode(i64),
}

#[derive(Clone)]
pub struct MetadataManager(mpsc::UnboundedSender<RefreshRequest>);

impl MetadataManager {
    pub fn new(db: Db, tmdb: TmdbClient) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(req) = receiver.recv().await {
                let mut conn = db.acquire().await.unwrap();
                let res = match req {
                    RefreshRequest::Movie(id) => refresh_movie_metadata(&mut conn, &tmdb, id).await,

                    RefreshRequest::TvShow(id) => {
                        refresh_tv_show_metadata(&mut conn, &tmdb, id).await
                    }

                    RefreshRequest::TvSeason(id) => {
                        refresh_tv_season_metadata(&mut conn, &tmdb, id).await
                    }

                    RefreshRequest::TvEpisode(id) => {
                        refresh_tv_episode_metadata(&mut conn, &tmdb, id).await
                    }
                };

                if let Err(e) = res {
                    tracing::error!("{}", e);
                }
            }
        });

        MetadataManager(sender)
    }

    pub fn enqueue(&self, req: RefreshRequest) {
        self.0
            .send(req)
            .expect("failed to send metadata refresh request");
    }
}

async fn refresh_movie_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    tracing::info!("updating metadata for movie (id: {})", id);

    let movie = db::movies::get(&mut *db, id)
        .await?
        .ok_or_else(|| eyre!("movie not found: {}", id))?;

    let path = std::path::Path::new(&movie.video_info.path);
    let name = path
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or_else(|| eyre!("invalid movie path: {:?}", path))?;

    let (title, year) = scanner::parse_movie_filename(name)
        .ok_or_else(|| eyre!("failed to parse movie name: {}", name))?;

    let query = MovieSearchQuery {
        title: &title,
        page: None,
        primary_release_year: year.map(|dt| dt.year()),
    };

    let metadata = tmdb.search_movies(&query).await?;
    let result = match metadata.results.into_iter().next() {
        Some(result) => result,
        None => {
            return Err(eyre!(
                "no match found for '{} ({})'",
                title,
                year.map(|dt| dt.year()).unwrap_or(-1)
            ))
        }
    };

    tracing::info!("match found: {}", result.title);

    let poster = result.poster_path.as_deref().map(|poster| MediaImage {
        img_type: MediaImageType::Poster,
        src_type: MediaImageSrcType::Tmdb,
        src: poster,
    });

    let backdrop = result.backdrop_path.as_deref().map(|backdrop| MediaImage {
        img_type: MediaImageType::Backdrop,
        src_type: MediaImageSrcType::Tmdb,
        src: backdrop,
    });

    let data = db::movies::UpdateMetadata {
        title: &result.title,
        overview: result.overview.as_deref(),
        poster,
        backdrop,
        tmdb_id: Some(result.id),
    };

    db::movies::update_metadata(&mut *db, id, data).await?;

    Ok(())
}

async fn refresh_tv_show_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    tracing::info!("updating metadata for tv show (id: {})", id);

    let path = db::shows::get_path(&mut *db, id)
        .await?
        .ok_or_else(|| eyre!("show not found: {}", id))?;

    let path = std::path::Path::new(&path);
    let name = path
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or_else(|| eyre!("invalid tv show path"))?;

    let query = TvShowSearchQuery {
        name,
        page: None,
        first_air_date_year: None,
    };

    let metadata = tmdb.search_tv_shows(&query).await?;
    let result = match metadata.results.into_iter().next() {
        Some(result) => result,
        None => return Ok(()),
    };

    tracing::info!("match found: {}", result.name);

    let date_fmt = format_description::parse("[year]-[month]-[day]")?;
    let first_air_date = result
        .first_air_date
        .and_then(|date| Date::parse(&date, &date_fmt).ok())
        .and_then(|date| Some(date.with_time(Time::from_hms(0, 0, 0).ok()?)))
        .map(|dt| dt.assume_utc().unix_timestamp());

    let details = tmdb.get_tv_show(result.id).await?;
    let last_air_date = details
        .last_air_date
        .and_then(|date| Date::parse(&date, &date_fmt).ok())
        .and_then(|date| Some(date.with_time(Time::from_hms(0, 0, 0).ok()?)))
        .map(|dt| dt.assume_utc().unix_timestamp());

    let poster = result.poster_path.as_deref().map(|poster| MediaImage {
        img_type: MediaImageType::Poster,
        src_type: MediaImageSrcType::Tmdb,
        src: poster,
    });

    let backdrop = result.backdrop_path.as_deref().map(|backdrop| MediaImage {
        img_type: MediaImageType::Backdrop,
        src_type: MediaImageSrcType::Tmdb,
        src: backdrop,
    });

    let data = db::shows::UpdateMetadata {
        name: &result.name,
        start_date: first_air_date,
        end_date: last_air_date,
        overview: result.overview.as_deref(),
        poster,
        backdrop,
        tmdb_id: Some(result.id),
    };

    db::shows::update_metadata(&mut *db, id, data).await?;

    Ok(())
}

async fn refresh_tv_season_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    tracing::info!("updating metadata for tv season (id: {})", id);

    let season = db::seasons::get(&mut *db, id)
        .await?
        .ok_or_else(|| eyre!("season not found: {}", id))?;

    let show = db::shows::get(&mut *db, season.show_id)
        .await?
        .ok_or_else(|| eyre!("show not found for season: {}", id))?;

    let show_tmdb_id = show
        .external_ids
        .tmdb
        .ok_or_else(|| eyre!("missing tmdb id for show: {}", show.id))?;

    let metadata = tmdb
        .get_tv_season(show_tmdb_id, season.season_number as i32)
        .await?;

    let poster = metadata.poster_path.as_deref().map(|poster| MediaImage {
        img_type: MediaImageType::Poster,
        src_type: MediaImageSrcType::Tmdb,
        src: poster,
    });

    tracing::info!(
        "match found: {}",
        metadata.name.as_deref().unwrap_or("unknown name")
    );

    let data = db::seasons::UpdateMetadata {
        name: metadata.name.as_deref(),
        overview: metadata.overview.as_deref(),
        poster,
        tmdb_id: Some(metadata.id),
    };

    db::seasons::update_metadata(&mut *db, id, data).await?;

    Ok(())
}

async fn refresh_tv_episode_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    tracing::info!("updating metadata for tv episode (id: {})", id);

    let episode = db::episodes::get(&mut *db, id)
        .await?
        .ok_or_else(|| eyre!("show not found: {}", id))?;

    let show = db::shows::get(&mut *db, episode.show_id)
        .await?
        .ok_or_else(|| eyre!("show not found for episode: {}", episode.show_id))?;

    let show_tmdb_id = show
        .external_ids
        .tmdb
        .ok_or_else(|| eyre!("missing tmdb id for show: {}", show.id))?;

    let season = episode.season_number as i32;
    let episode = episode.episode_number as i32;

    let metadata = tmdb.get_tv_episode(show_tmdb_id, season, episode).await?;
    let thumbnail = tmdb
        .get_tv_episode_images(show_tmdb_id, season, episode)
        .await
        .map(|images| images.stills.into_iter().next())
        .ok()
        .flatten()
        .map(|image| image.file_path);

    let thumbnail = thumbnail.as_deref().map(|thumbnail| MediaImage {
        img_type: MediaImageType::Thumbnail,
        src_type: MediaImageSrcType::Tmdb,
        src: thumbnail,
    });

    tracing::info!(
        "match found: {}",
        metadata.name.as_deref().unwrap_or("unknown name")
    );

    let data = db::episodes::UpdateMetadata {
        name: metadata.name.as_deref(),
        overview: metadata.overview.as_deref(),
        thumbnail,
        tmdb_id: Some(metadata.id),
    };

    db::episodes::update_metadata(&mut *db, id, data).await?;

    Ok(())
}
