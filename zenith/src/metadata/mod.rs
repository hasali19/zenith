use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqliteConnection};

use crate::tmdb::{MovieSearchQuery, TmdbClient, TvShowSearchQuery};

pub async fn refresh_movie_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    log::info!("updating metadata for movie (id: {})", id);

    let title: String = sqlx::query("SELECT title FROM movies WHERE id = ?")
        .bind(id)
        .try_map(|row: SqliteRow| row.try_get(0))
        .fetch_one(&mut *db)
        .await?;

    let query = MovieSearchQuery {
        title: &title,
        page: None,
        primary_release_year: None,
    };

    let metadata = tmdb.search_movies(&query).await?;
    let result = match metadata.results.into_iter().next() {
        Some(result) => result,
        None => return Ok(()),
    };

    log::info!("match found: {}", result.title);

    let sql = "
        UPDATE movies
        SET title = ?,
            overview = ?,
            poster = ?,
            backdrop = ?
        WHERE id = ?
    ";

    sqlx::query(sql)
        .bind(result.title)
        .bind(result.overview)
        .bind(result.poster_path.map(|v| format!("tmdb.poster|{}", v)))
        .bind(result.backdrop_path.map(|v| format!("tmdb.backdrop|{}", v)))
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}

pub async fn refresh_tv_show_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    log::info!("updating metadata for tv show (id: {})", id);

    let name: String = sqlx::query("SELECT name FROM tv_shows WHERE id = ?")
        .bind(id)
        .try_map(|row: SqliteRow| row.try_get(0))
        .fetch_one(&mut *db)
        .await?;

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

    let sql = "
        UPDATE tv_shows
        SET name = ?,
            overview = ?,
            poster = ?,
            backdrop = ?,
            tmdb_id = ?
        WHERE id = ?
    ";

    sqlx::query(sql)
        .bind(result.name)
        .bind(result.overview)
        .bind(result.poster_path.map(|v| format!("tmdb.poster|{}", v)))
        .bind(result.backdrop_path.map(|v| format!("tmdb.backdrop|{}", v)))
        .bind(result.id)
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}

pub async fn refresh_tv_episode_metadata(
    db: &mut SqliteConnection,
    tmdb: &TmdbClient,
    id: i64,
) -> eyre::Result<()> {
    log::info!("updating metadata for tv episode (id: {})", id);

    let sql = "
        SELECT show.tmdb_id, episode.season, episode.episode
        FROM tv_episodes AS episode
        JOIN tv_shows AS show ON episode.show_id = show.id
        WHERE episode.id = ?
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

    log::info!(
        "match found: {}",
        metadata.name.as_deref().unwrap_or("unknown name")
    );

    let sql = "
        UPDATE tv_episodes
        SET name = ?,
            overview = ?,
            thumbnail = ?
        WHERE id = ?
    ";

    sqlx::query(sql)
        .bind(metadata.name)
        .bind(metadata.overview)
        .bind(thumbnail.map(|v| format!("tmdb.still|{}", v)))
        .bind(id)
        .execute(&mut *db)
        .await?;

    Ok(())
}
