use eyre::eyre;
use itertools::Itertools;
use sqlx::SqliteConnection;
use thiserror::Error;
use time::{format_description, Date, OffsetDateTime, Time};
use tmdb::{MovieSearchQuery, TmdbClient, TvShowSearchQuery};
use tokio::sync::mpsc;

use crate::db::items::MediaItem;
use crate::db::media::{MediaImage, MediaImageSrcType, MediaImageType, MediaItemType};
use crate::db::{self, Db};

#[derive(Clone)]
pub struct MetadataManager(mpsc::UnboundedSender<i64>);

impl MetadataManager {
    pub fn new(db: Db, tmdb: TmdbClient) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(id) = receiver.recv().await {
                let mut conn = db.acquire().await.unwrap();
                let res = find_match(&mut conn, &tmdb, id).await;
                if let Err(e) = res {
                    tracing::error!("{e:?}");
                }
            }
        });

        MetadataManager(sender)
    }

    pub fn enqueue_unmatched(&self, id: i64) {
        self.0
            .send(id)
            .expect("failed to send metadata update request");
    }

    #[tracing::instrument(skip(self, conn))]
    pub async fn enqueue_all_unmatched(&self, conn: &mut SqliteConnection) -> eyre::Result<()> {
        let unmatched = sqlx::query_scalar("SELECT id FROM media_items WHERE tmdb_id IS NULL")
            .fetch_all(conn)
            .await?;

        if !unmatched.is_empty() {
            tracing::info!(
                count = unmatched.len(),
                "enqueuing unmatched items for matching"
            );
        }

        for id in unmatched {
            self.enqueue_unmatched(id);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("item not found")]
    NotFound,
    #[error(transparent)]
    Other(#[from] eyre::Report),
}

#[tracing::instrument(skip(conn, tmdb))]
pub async fn find_match(
    conn: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> Result<(), Error> {
    let item = db::items::get(&mut *conn, id)
        .await
        .map_err(Error::Other)?
        .ok_or(Error::NotFound)?;

    match item.kind {
        MediaItemType::Movie => find_match_for_movie(conn, tmdb, item).await?,
        MediaItemType::Show => find_match_for_show(conn, tmdb, item).await?,
        MediaItemType::Season => refresh_tv_season_metadata(conn, tmdb, id).await?,
        MediaItemType::Episode => refresh_tv_episode_metadata(conn, tmdb, id).await?,
    }

    Ok(())
}

async fn find_match_for_movie(
    conn: &mut SqliteConnection,
    tmdb: &TmdbClient,
    mut item: MediaItem,
) -> eyre::Result<()> {
    tracing::info!("finding match for movie");

    let title = &item.name;
    let year = item
        .start_date
        .and_then(|ts| OffsetDateTime::from_unix_timestamp(ts).ok())
        .map(|dt| dt.year());

    let query = MovieSearchQuery {
        title,
        page: None,
        year,
    };

    let metadata = tmdb.search_movies(&query).await?;
    let result = match metadata.results.into_iter().next() {
        Some(result) => result,
        None => {
            return if let Some(year) = year {
                Err(eyre!("no match found for '{title} ({year})'"))
            } else {
                Err(eyre!("no match found for '{title}'"))
            }
        }
    };

    tracing::info!(result.title, "match found");

    db::items::update_metadata(
        &mut *conn,
        item.id,
        db::items::UpdateMetadata {
            tmdb_id: Some(Some(result.id)),
            ..Default::default()
        },
    )
    .await?;

    item.tmdb_id = Some(result.id);

    refresh_movie_metadata(conn, tmdb, item).await
}

async fn find_match_for_show(
    conn: &mut SqliteConnection,
    tmdb: &TmdbClient,
    mut item: MediaItem,
) -> eyre::Result<()> {
    tracing::info!("finding match for show");

    let name = &item.name;

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

    tracing::info!(result.name, "match found");

    db::items::update_metadata(
        &mut *conn,
        item.id,
        db::items::UpdateMetadata {
            tmdb_id: Some(Some(result.id)),
            ..Default::default()
        },
    )
    .await?;

    item.tmdb_id = Some(result.id);

    refresh_tv_show_metadata(conn, tmdb, item).await
}

#[tracing::instrument(skip(conn, tmdb))]
pub async fn refresh(conn: &mut SqliteConnection, tmdb: &TmdbClient, id: i64) -> Result<(), Error> {
    let item = db::items::get(&mut *conn, id)
        .await
        .map_err(Error::Other)?
        .ok_or(Error::NotFound)?;

    match item.kind {
        MediaItemType::Movie => refresh_movie_metadata(conn, tmdb, item).await?,
        MediaItemType::Show => refresh_tv_show_metadata(conn, tmdb, item).await?,
        MediaItemType::Season => refresh_tv_season_metadata(conn, tmdb, id).await?,
        MediaItemType::Episode => refresh_tv_episode_metadata(conn, tmdb, id).await?,
    }

    Ok(())
}

async fn refresh_movie_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    item: MediaItem,
) -> eyre::Result<()> {
    tracing::info!("refreshing movie metadata");

    let Some(tmdb_id) = item.tmdb_id else {
        tracing::error!(?item, "movie is unmatched");
        return Err(eyre!("unmatched item"));
    };

    let metadata = tmdb.get_movie(tmdb_id).await?;

    tracing::debug!(?metadata);

    let poster = metadata.poster_path.as_deref().map(|poster| MediaImage {
        img_type: MediaImageType::Poster,
        src_type: MediaImageSrcType::Tmdb,
        src: poster,
    });

    let backdrop = metadata
        .backdrop_path
        .as_deref()
        .map(|backdrop| MediaImage {
            img_type: MediaImageType::Backdrop,
            src_type: MediaImageSrcType::Tmdb,
            src: backdrop,
        });

    let genres = metadata
        .genres
        .iter()
        .map(|g| g.name.as_str())
        .collect_vec();

    let age_rating = metadata
        .release_dates
        .results
        .into_iter()
        .find(|it| it.iso_3166_1 == "GB") // FIXME: Hardcoded region
        .and_then(|it| it.release_dates.into_iter().next())
        .map(|it| format!("GB-{}", it.certification));

    let data = db::items::UpdateMetadata {
        name: Some(&metadata.title),
        overview: Some(metadata.overview.as_deref()),
        start_date: None,
        end_date: None,
        poster: Some(poster),
        backdrop: Some(backdrop),
        thumbnail: None,
        age_rating: Some(age_rating.as_deref()),
        genres: Some(&genres),
        tmdb_id: Some(Some(metadata.id)),
        imdb_id: Some(metadata.external_ids.imdb_id.as_deref()),
    };

    db::items::update_metadata(&mut *db, item.id, data).await?;

    Ok(())
}

async fn refresh_tv_show_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    item: MediaItem,
) -> eyre::Result<()> {
    tracing::info!("refreshing show metadata");

    let Some(tmdb_id) = item.tmdb_id else {
        tracing::error!(?item, "show is unmatched");
        return Err(eyre!("unmatched item"));
    };

    let metadata = tmdb.get_tv_show(tmdb_id).await?;

    tracing::debug!(?metadata);

    let date_fmt = format_description::parse("[year]-[month]-[day]")?;
    let first_air_date = metadata
        .first_air_date
        .and_then(|date| Date::parse(&date, &date_fmt).ok())
        .and_then(|date| Some(date.with_time(Time::from_hms(0, 0, 0).ok()?)))
        .map(|dt| dt.assume_utc().unix_timestamp());

    let last_air_date = metadata
        .last_air_date
        .and_then(|date| Date::parse(&date, &date_fmt).ok())
        .and_then(|date| Some(date.with_time(Time::from_hms(0, 0, 0).ok()?)))
        .map(|dt| dt.assume_utc().unix_timestamp());

    let poster = metadata.poster_path.as_deref().map(|poster| MediaImage {
        img_type: MediaImageType::Poster,
        src_type: MediaImageSrcType::Tmdb,
        src: poster,
    });

    let backdrop = metadata
        .backdrop_path
        .as_deref()
        .map(|backdrop| MediaImage {
            img_type: MediaImageType::Backdrop,
            src_type: MediaImageSrcType::Tmdb,
            src: backdrop,
        });

    let genres = metadata
        .genres
        .iter()
        .map(|g| g.name.as_str())
        .collect_vec();

    let age_rating = metadata
        .content_ratings
        .results
        .into_iter()
        .find(|it| it.iso_3166_1 == "GB") // FIXME: Hardcoded region
        .map(|it| format!("GB-{}", it.rating));

    let data = db::items::UpdateMetadata {
        name: Some(&metadata.name),
        start_date: Some(first_air_date),
        end_date: Some(last_air_date),
        overview: Some(metadata.overview.as_deref()),
        poster: Some(poster),
        thumbnail: None,
        backdrop: Some(backdrop),
        age_rating: Some(age_rating.as_deref()),
        genres: Some(&genres),
        tmdb_id: Some(Some(metadata.id)),
        imdb_id: Some(metadata.external_ids.imdb_id.as_deref()),
    };

    db::items::update_metadata(&mut *db, item.id, data).await?;

    Ok(())
}

async fn refresh_tv_season_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    tracing::info!("refreshing season metadata");

    let season = db::seasons::get(&mut *db, id)
        .await?
        .ok_or_else(|| eyre!("season not found: {id}"))?;

    let show = db::items::get(&mut *db, season.show_id)
        .await?
        .ok_or_else(|| eyre!("show not found for season: {id}"))?;

    let show_tmdb_id = show
        .tmdb_id
        .ok_or_else(|| eyre!("missing tmdb id for show: {}", show.id))?;

    let metadata = tmdb
        .get_tv_season(show_tmdb_id, season.season_number as i32)
        .await?;

    tracing::debug!(?metadata);

    let poster = metadata.poster_path.as_deref().map(|poster| MediaImage {
        img_type: MediaImageType::Poster,
        src_type: MediaImageSrcType::Tmdb,
        src: poster,
    });

    let data = db::items::UpdateMetadata {
        name: metadata.name.as_deref(),
        overview: Some(metadata.overview.as_deref()),
        start_date: None,
        end_date: None,
        poster: Some(poster),
        backdrop: None,
        thumbnail: None,
        age_rating: None,
        genres: None,
        tmdb_id: Some(Some(metadata.id)),
        imdb_id: Some(metadata.external_ids.imdb_id.as_deref()),
    };

    db::items::update_metadata(&mut *db, id, data).await?;

    Ok(())
}

async fn refresh_tv_episode_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    tracing::info!("refreshing episode metadata");

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

    tracing::debug!(?metadata);

    let thumbnail = metadata
        .images
        .stills
        .into_iter()
        .next()
        .map(|image| image.file_path);

    let thumbnail = thumbnail.as_deref().map(|thumbnail| MediaImage {
        img_type: MediaImageType::Thumbnail,
        src_type: MediaImageSrcType::Tmdb,
        src: thumbnail,
    });

    let data = db::items::UpdateMetadata {
        name: metadata.name.as_deref(),
        overview: Some(metadata.overview.as_deref()),
        start_date: None,
        end_date: None,
        poster: None,
        backdrop: None,
        thumbnail: Some(thumbnail),
        age_rating: None,
        genres: None,
        tmdb_id: Some(Some(metadata.id)),
        imdb_id: Some(metadata.external_ids.imdb_id.as_deref()),
    };

    db::items::update_metadata(&mut *db, id, data).await?;

    Ok(())
}
