use eyre::{eyre, Context};
use itertools::Itertools;
use sqlx::SqliteConnection;
use thiserror::Error;
use time::{format_description, Date, OffsetDateTime, Time};
use tmdb::{MovieReleaseDatesResult, MovieSearchQuery, TmdbClient, TvShowSearchQuery};
use tokio::sync::mpsc;

use crate::db::items::MediaItem;
use crate::db::media::{
    MediaImage, MediaImageSrcType, MediaImageType, MediaItemType, MetadataProvider,
};
use crate::db::{self, Db};

#[derive(Debug)]
enum Request {
    FindMatch(i64),
    Refresh(i64),
}

#[derive(Clone)]
pub struct MetadataManager(mpsc::UnboundedSender<Request>);

impl MetadataManager {
    pub fn new(db: Db, tmdb: TmdbClient) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(req) = receiver.recv().await {
                let mut conn = db.acquire().await.unwrap();
                let res = match req {
                    Request::FindMatch(id) => find_match(&mut conn, &tmdb, id).await,
                    Request::Refresh(id) => refresh(&mut conn, &tmdb, id).await,
                };

                if let Err(e) = res {
                    tracing::error!("{e:?}");
                }
            }
        });

        MetadataManager(sender)
    }

    pub fn enqueue_unmatched(&self, id: i64) {
        self.0
            .send(Request::FindMatch(id))
            .expect("failed to send metadata update request");
    }

    pub fn enqueue_outdated(&self, id: i64) {
        self.0
            .send(Request::Refresh(id))
            .expect("failed to send metadata refresh request");
    }

    #[tracing::instrument(skip(self, conn))]
    pub async fn enqueue_all_unmatched(&self, conn: &mut SqliteConnection) -> eyre::Result<()> {
        let sql = "
            SELECT id FROM media_items
            WHERE metadata_provider IS NOT NULL AND metadata_provider_key IS NULL
        ";

        let unmatched = sqlx::query_scalar(sql).fetch_all(conn).await?;

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

    #[tracing::instrument(skip(self, conn))]
    pub async fn enqueue_all_outdated(&self, conn: &mut SqliteConnection) -> eyre::Result<()> {
        let sql = "
            SELECT id FROM media_items
            WHERE metadata_provider IS NOT NULL AND metadata_provider_key IS NOT NULL
                AND metadata_updated_at < strftime('%s') - 60 * 60 * 24 * 7
        ";

        let outdated = sqlx::query_scalar(sql).fetch_all(conn).await?;

        if !outdated.is_empty() {
            tracing::info!(
                count = outdated.len(),
                "enqueuing outdated items for refresh"
            );
        }

        for id in outdated {
            self.enqueue_outdated(id);
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
        MediaItemType::Season => refresh_tv_season_metadata(conn, tmdb, item).await?,
        MediaItemType::Episode => refresh_tv_episode_metadata(conn, tmdb, item).await?,
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

    let metadata_key = result.id.to_string();

    db::items::update_metadata(
        &mut *conn,
        item.id,
        db::items::UpdateMetadata {
            metadata_provider: Some(Some(MetadataProvider::Tmdb)),
            metadata_provider_key: Some(Some(&metadata_key)),
            ..Default::default()
        },
    )
    .await?;

    item.metadata_provider_key = Some(metadata_key);

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

    let metadata_key = result.id.to_string();

    db::items::update_metadata(
        &mut *conn,
        item.id,
        db::items::UpdateMetadata {
            metadata_provider: Some(Some(MetadataProvider::Tmdb)),
            metadata_provider_key: Some(Some(&metadata_key)),
            ..Default::default()
        },
    )
    .await?;

    item.metadata_provider_key = Some(metadata_key);

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
        MediaItemType::Season => refresh_tv_season_metadata(conn, tmdb, item).await?,
        MediaItemType::Episode => refresh_tv_episode_metadata(conn, tmdb, item).await?,
    }

    Ok(())
}

async fn refresh_movie_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    item: MediaItem,
) -> eyre::Result<()> {
    tracing::info!("refreshing movie metadata");

    let Some(key) = item.metadata_provider_key.as_ref().and_then(|key|key.parse().ok()) else {
        tracing::error!(?item, "movie is unmatched");
        return Err(eyre!("unmatched item"));
    };

    let metadata = tmdb.get_movie(key).await?;

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

    fn has_certification(res: &MovieReleaseDatesResult) -> bool {
        matches!(res.release_dates.first(), Some(release_date) if !release_date.certification.is_empty())
    }

    let age_rating = metadata
        .release_dates
        .results
        .iter()
        .find(|it| it.iso_3166_1 == "GB" && has_certification(it)) // FIXME: Hardcoded region
        .and_then(|it| it.release_dates.first())
        .map(|it| format!("GB-{}", it.certification))
        .or_else(|| {
            metadata
                .release_dates
                .results
                .iter()
                .find(|it| it.iso_3166_1 == "US")
                .and_then(|it| it.release_dates.first())
                .map(|it| it.certification.to_owned())
        });

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
        metadata_provider: Some(Some(MetadataProvider::Tmdb)),
        metadata_provider_key: None,
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

    let Some(key) = item.metadata_provider_key.as_ref().and_then(|key|key.parse().ok()) else {
        tracing::error!(?item, "show is unmatched");
        return Err(eyre!("unmatched item"));
    };

    let metadata = tmdb.get_tv_show(key).await?;

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
        .iter()
        .find(|it| it.iso_3166_1 == "GB" && !it.rating.is_empty()) // FIXME: Hardcoded region
        .map(|it| format!("GB-{}", it.rating))
        .or_else(|| {
            metadata
                .content_ratings
                .results
                .iter()
                .find(|it| it.iso_3166_1 == "US")
                .map(|it| it.rating.to_owned())
        });

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
        metadata_provider: Some(Some(MetadataProvider::Tmdb)),
        metadata_provider_key: None,
    };

    db::items::update_metadata(&mut *db, item.id, data).await?;

    Ok(())
}

fn parse_season_key(key: &str) -> Option<(i32, i32)> {
    let (show_id, season_number) = key.split_once(':')?;
    Some((show_id.parse().ok()?, season_number.parse().ok()?))
}

async fn refresh_tv_season_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    item: MediaItem,
) -> eyre::Result<()> {
    tracing::info!("refreshing season metadata");

    if item.metadata_provider.is_none() {
        tracing::info!("no metadata provider set");
        return Ok(());
    }

    let (show_id, season_number) = match item
        .metadata_provider_key
        .as_deref()
        .and_then(parse_season_key)
    {
        Some(key) => key,
        None => {
            let parent = item.parent.unwrap();
            let show = db::items::get(&mut *db, parent.id)
                .await?
                .ok_or_else(|| eyre!("show not found for season: {}", item.id))?;

            let show_tmdb_id = show
                .metadata_provider_key
                .ok_or_else(|| eyre!("missing tmdb id for show: {}", show.id))?
                .parse()
                .wrap_err_with(|| eyre!("invalid tmdb id for show: {}", show.id))?;

            (show_tmdb_id, parent.index as i32)
        }
    };

    let metadata = tmdb.get_tv_season(show_id, season_number).await?;

    tracing::debug!(?metadata);

    let poster = metadata.poster_path.as_deref().map(|poster| MediaImage {
        img_type: MediaImageType::Poster,
        src_type: MediaImageSrcType::Tmdb,
        src: poster,
    });

    let metadata_key = format!("{show_id}:{season_number}");
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
        metadata_provider: Some(Some(MetadataProvider::Tmdb)),
        metadata_provider_key: Some(Some(&metadata_key)),
    };

    db::items::update_metadata(&mut *db, item.id, data).await?;

    Ok(())
}

fn parse_episode_key(key: &str) -> Option<(i32, i32, i32)> {
    let (show_id, season_number, episode_number) = key.splitn(3, ':').collect_tuple()?;
    Some((
        show_id.parse().ok()?,
        season_number.parse().ok()?,
        episode_number.parse().ok()?,
    ))
}

async fn refresh_tv_episode_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    item: MediaItem,
) -> eyre::Result<()> {
    tracing::info!("refreshing episode metadata");

    if item.metadata_provider.is_none() {
        tracing::info!("no metadata provider set");
        return Ok(());
    }

    let (show_id, season_number, episode_number) = match item
        .metadata_provider_key
        .as_deref()
        .and_then(parse_episode_key)
    {
        Some(key) => key,
        None => {
            let parent = item.parent.unwrap();
            let grandparent = item.grandparent.unwrap();
            let show = db::items::get(&mut *db, grandparent.id)
                .await?
                .ok_or_else(|| eyre!("show not found for episode: {}", item.id))?;

            let show_tmdb_id = show
                .metadata_provider_key
                .ok_or_else(|| eyre!("missing tmdb id for show: {}", show.id))?
                .parse()
                .wrap_err_with(|| eyre!("invalid tmdb id for show: {}", show.id))?;

            (show_tmdb_id, grandparent.index as i32, parent.index as i32)
        }
    };

    let metadata = tmdb
        .get_tv_episode(show_id, season_number, episode_number)
        .await?;

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

    let metadata_key = format!("{show_id}:{season_number}:{episode_number}");
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
        metadata_provider: Some(Some(MetadataProvider::Tmdb)),
        metadata_provider_key: Some(Some(&metadata_key)),
    };

    db::items::update_metadata(&mut *db, item.id, data).await?;

    Ok(())
}
