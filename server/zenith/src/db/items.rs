use std::fmt::Write;

use serde::Serialize;
use speq::Reflect;
use sqlx::SqliteConnection;

use super::episodes::{self, Episode};
use super::media::{self, MediaImageType, MediaItemType};
use super::movies::{self, Movie};
use super::seasons::{self, Season};
use super::shows::{self, Show};

#[derive(Serialize, Reflect)]
pub struct ExternalIds {
    pub tmdb: Option<i32>,
}

#[derive(Serialize, Reflect)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MediaItem {
    Movie(Movie),
    Show(Show),
    Season(Season),
    Episode(Episode),
}

impl MediaItem {
    pub fn image(&self, img_type: MediaImageType) -> Option<&str> {
        match self {
            MediaItem::Movie(movie) => movie.image(img_type),
            MediaItem::Show(show) => show.image(img_type),
            MediaItem::Season(season) => season.image(img_type),
            MediaItem::Episode(episode) => episode.image(img_type),
        }
    }
}

pub async fn get(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<MediaItem>> {
    let item_type = match media::get_item_type(&mut *conn, id).await? {
        Some(item_type) => item_type,
        None => return Ok(None),
    };

    let item = match item_type {
        MediaItemType::Movie => movies::get(&mut *conn, id).await?.map(MediaItem::Movie),
        MediaItemType::Show => shows::get(&mut *conn, id).await?.map(MediaItem::Show),
        MediaItemType::Season => seasons::get(&mut *conn, id).await?.map(MediaItem::Season),
        MediaItemType::Episode => episodes::get(&mut *conn, id).await?.map(MediaItem::Episode),
    };

    Ok(item)
}

pub async fn get_multiple(
    conn: &mut SqliteConnection,
    ids: impl IntoIterator<Item = i64>,
) -> eyre::Result<Vec<MediaItem>> {
    // TODO: Reduce number of db queries
    let mut items = vec![];

    for id in ids {
        if let Some(item) = get(&mut *conn, id).await? {
            items.push(item);
        }
    }

    Ok(items)
}

pub async fn get_continue_watching(
    conn: &mut SqliteConnection,
    limit: Option<u32>,
) -> eyre::Result<Vec<MediaItem>> {
    // This beautiful query does two things:
    // - for movies, we grab ids of all movies where the user position is within the "currently watching" range
    // - for each show, we grab the last episode that was watched; if that episode was finished, then we instead
    //   get the next episode if it exists
    let mut sql = "
        SELECT id, last_watched_at FROM (
            SELECT m.item_id AS id, u.last_watched_at AS last_watched_at FROM movies AS m
            JOIN video_files AS v ON v.item_id = m.item_id
            LEFT JOIN user_item_data AS u ON m.item_id = u.item_id
            WHERE u.position > (0.05 * v.duration) AND u.position < (0.9 * v.duration) AND u.last_watched_at IS NOT NULL
        )
        UNION
        SELECT id, last_watched_at FROM (
            SELECT IIF(
                u.position < (0.9 * v.duration),
                -- return current episode if the position is below 'completed' threshold
                e.item_id,
                -- otherwise find the next episode
                (
                    SELECT e1.item_id FROM tv_episodes AS e1
                    JOIN tv_seasons AS season1 ON season1.item_id = e1.season_id
                    JOIN tv_shows AS show1 ON show1.item_id = season1.show_id
                    WHERE show1.item_id = show.item_id
                        AND (season1.season_number > season.season_number
                            OR (season1.season_number = season.season_number AND e1.episode_number > e.episode_number))
                    ORDER BY season1.season_number, e1.episode_number
                    LIMIT 1
                )
            ) AS id, MAX(last_watched_at) AS last_watched_at FROM tv_episodes AS e
            JOIN tv_seasons AS season ON season.item_id = e.season_id
            JOIN tv_shows AS show ON show.item_id = season.show_id
            JOIN video_files AS v ON v.item_id = e.item_id
            LEFT JOIN user_item_data AS u ON e.item_id = u.item_id
            WHERE u.position > (0.05 * v.duration) AND u.last_watched_at IS NOT NULL
            GROUP BY show.item_id
        )
        ORDER BY last_watched_at DESC
    ".to_owned();

    if let Some(limit) = limit {
        write!(sql, "LIMIT {}", limit).unwrap();
    }

    let ids: Vec<i64> = sqlx::query_scalar(&sql).fetch_all(&mut *conn).await?;

    get_multiple(conn, ids).await
}
