use eyre::eyre;
use sqlx::SqliteConnection;
use time::{format_description, Date, Time};
use tokio::sync::mpsc;

use crate::db::media::{MediaImage, MediaImageSrcType, MediaImageType};
use crate::db::Db;
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

    let sql = "
        SELECT path FROM movies
        JOIN video_files USING (item_id)
        WHERE item_id = ?
    ";

    let path: String = sqlx::query_scalar(sql).bind(id).fetch_one(&mut *db).await?;

    let path = std::path::Path::new(&path);
    let name = path
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or_else(|| eyre!("invalid movie path"))?;

    let (title, year) =
        scanner::parse_movie_filename(name).ok_or_else(|| eyre!("failed to parse movie name"))?;

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

    let sql = "
        UPDATE movies
        SET title    = ?,
            overview = ?,
            poster   = ?,
            backdrop = ?,
            tmdb_id  = ?
        WHERE item_id = ?
    ";

    sqlx::query(sql)
        .bind(result.title)
        .bind(result.overview)
        .bind(poster.map(|p| p.to_string()))
        .bind(backdrop.map(|b| b.to_string()))
        .bind(result.id)
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}

async fn refresh_tv_show_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    tracing::info!("updating metadata for tv show (id: {})", id);

    let path: String = sqlx::query_scalar("SELECT path FROM tv_shows WHERE item_id = ?")
        .bind(id)
        .fetch_one(&mut *db)
        .await?;

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

    let first_air_date = result
        .first_air_date
        .and_then(|date| Date::parse(&date, &format_description::parse("%F").ok()?).ok())
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

    let sql = "
        UPDATE tv_shows
        SET name = ?,
            start_date = ?,
            overview = ?,
            poster = ?,
            backdrop = ?,
            tmdb_id = ?
        WHERE item_id = ?
    ";

    sqlx::query(sql)
        .bind(result.name)
        .bind(first_air_date)
        .bind(result.overview)
        .bind(poster.map(|p| p.to_string()))
        .bind(backdrop.map(|b| b.to_string()))
        .bind(result.id)
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}

async fn refresh_tv_season_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    tracing::info!("updating metadata for tv season (id: {})", id);

    let sql = "
        SELECT show.tmdb_id, season.season_number
        FROM tv_seasons AS season
        JOIN tv_shows AS show ON show.item_id = season.show_id
        WHERE season.item_id = ?
    ";

    let (tmdb_show_id, season): (i32, i32) =
        sqlx::query_as(sql).bind(id).fetch_one(&mut *db).await?;

    let metadata = tmdb.get_tv_season(tmdb_show_id, season).await?;
    let poster = metadata.poster_path.as_deref().map(|poster| MediaImage {
        img_type: MediaImageType::Poster,
        src_type: MediaImageSrcType::Tmdb,
        src: poster,
    });

    tracing::info!(
        "match found: {}",
        metadata.name.as_deref().unwrap_or("unknown name")
    );

    let sql = "
        UPDATE tv_seasons
        SET name = ?,
            overview = ?,
            poster = ?,
            tmdb_id = ?
        WHERE item_id = ?
    ";

    sqlx::query(sql)
        .bind(metadata.name)
        .bind(metadata.overview)
        .bind(poster.map(|p| p.to_string()))
        .bind(metadata.id)
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}

async fn refresh_tv_episode_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    tracing::info!("updating metadata for tv episode (id: {})", id);

    let sql = "
        SELECT show.tmdb_id, season.season_number, episode.episode_number
        FROM tv_episodes AS episode
        JOIN tv_seasons AS season ON season.item_id = episode.season_id
        JOIN tv_shows AS show ON show.item_id = season.show_id
        WHERE episode.item_id = ?
    ";

    let (tmdb_show_id, season, episode): (i32, i32, i32) =
        sqlx::query_as(sql).bind(id).fetch_one(&mut *db).await?;

    let metadata = tmdb.get_tv_episode(tmdb_show_id, season, episode).await?;
    let thumbnail = tmdb
        .get_tv_episode_images(tmdb_show_id, season, episode)
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

    let sql = "
        UPDATE tv_episodes
        SET name = ?,
            overview = ?,
            thumbnail = ?,
            tmdb_id = ?
        WHERE item_id = ?
    ";

    sqlx::query(sql)
        .bind(metadata.name)
        .bind(metadata.overview)
        .bind(thumbnail.map(|t| t.to_string()))
        .bind(metadata.id)
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}
