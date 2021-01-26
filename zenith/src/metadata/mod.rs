use eyre::eyre;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqliteConnection};
use tokio::sync::mpsc;

use crate::db::media::{MediaImage, MediaImageSrcType, MediaImageType, MediaItemType};
use crate::db::Db;
use crate::tmdb::{MovieSearchQuery, TmdbClient, TvShowSearchQuery};

#[derive(Debug)]
pub enum RefreshRequest {
    Movie(i64),
    TvShow(i64),
    TvSeason(i64),
    TvEpisode(i64),
}

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
                    log::error!("{}", e);
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
    log::info!("updating metadata for movie (id: {})", id);

    let (path,): (String,) =
        sqlx::query_as("SELECT path FROM media_items WHERE item_type = ? AND id = ?")
            .bind(MediaItemType::Movie)
            .bind(id)
            .fetch_one(&mut *db)
            .await?;

    let path = std::path::Path::new(&path);
    let name = path
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or_else(|| eyre!("invalid movie path"))?;

    let (title, year) = crate::sync::movies::parse_movie_dir_name(name)
        .ok_or_else(|| eyre!("failed to parse movie name"))?;

    let query = MovieSearchQuery {
        title: &title,
        page: None,
        primary_release_year: year,
    };

    let metadata = tmdb.search_movies(&query).await?;
    let result = match metadata.results.into_iter().next() {
        Some(result) => result,
        None => {
            return Err(eyre!(
                "no match found for '{} ({})'",
                title,
                year.unwrap_or(-1)
            ))
        }
    };

    log::info!("match found: {}", result.title);

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
        UPDATE media_items
        SET name = ?,
            overview = ?,
            primary_image = ?,
            backdrop_image = ?
        WHERE id = ?
    ";

    sqlx::query(sql)
        .bind(result.title)
        .bind(result.overview)
        .bind(poster.map(|p| p.to_string()))
        .bind(backdrop.map(|b| b.to_string()))
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
    log::info!("updating metadata for tv show (id: {})", id);

    let path: String = sqlx::query("SELECT path FROM media_items WHERE id = ? AND item_type = ?")
        .bind(id)
        .bind(MediaItemType::TvShow)
        .try_map(|row: SqliteRow| row.try_get(0))
        .fetch_one(&mut *db)
        .await?;

    let path = std::path::Path::new(&path);
    let name = path
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or_else(|| eyre!("invalid tv show path"))?;

    let query = TvShowSearchQuery {
        name: &name,
        page: None,
        first_air_date_year: None,
    };

    let metadata = tmdb.search_tv_shows(&query).await?;
    let result = match metadata.results.into_iter().next() {
        Some(result) => result,
        None => return Ok(()),
    };

    log::info!("match found: {}", result.name);

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
        UPDATE media_items
        SET name = ?,
            overview = ?,
            primary_image = ?,
            backdrop_image = ?,
            tmdb_id = ?
        WHERE id = ?
    ";

    sqlx::query(sql)
        .bind(result.name)
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
    log::info!("updating metadata for tv season (id: {})", id);

    let sql = "
        SELECT show.tmdb_id, season.index_number
        FROM media_items AS season
        JOIN media_items AS show ON show.id = season.parent_id
        WHERE season.id = ? AND season.item_type = ?
    ";

    let (tmdb_show_id, season): (i32, i32) = sqlx::query_as(sql)
        .bind(id)
        .bind(MediaItemType::TvSeason)
        .fetch_one(&mut *db)
        .await?;

    let metadata = tmdb.get_tv_season(tmdb_show_id, season).await?;
    let poster = metadata.poster_path.as_deref().map(|poster| MediaImage {
        img_type: MediaImageType::Poster,
        src_type: MediaImageSrcType::Tmdb,
        src: poster,
    });

    log::info!(
        "match found: {}",
        metadata.name.as_deref().unwrap_or("unknown name")
    );

    let sql = "
        UPDATE media_items
        SET name = ?,
            overview = ?,
            primary_image = ?,
            tmdb_id = ?
        WHERE id = ?
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
    log::info!("updating metadata for tv episode (id: {})", id);

    let sql = "
        SELECT show.tmdb_id, season.index_number, episode.index_number
        FROM media_items AS episode
        JOIN media_items AS season ON season.id = episode.parent_id
        JOIN media_items AS show ON show.id = season.parent_id
        WHERE episode.id = ? AND episode.item_type = ?
    ";

    let (tmdb_show_id, season, episode): (i32, i32, i32) = sqlx::query_as(sql)
        .bind(id)
        .bind(MediaItemType::TvEpisode)
        .fetch_one(&mut *db)
        .await?;

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

    log::info!(
        "match found: {}",
        metadata.name.as_deref().unwrap_or("unknown name")
    );

    let sql = "
        UPDATE media_items
        SET name = ?,
            overview = ?,
            primary_image = ?,
            tmdb_id = ?
        WHERE id = ?
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
