use serde::Serialize;
use sqlx::SqliteConnection;

use super::episodes::{self, Episode};
use super::media::{self, MediaItemType};
use super::movies::{self, Movie};
use super::seasons::{self, Season};
use super::shows::{self, Show};

#[derive(Serialize)]
pub struct ExternalIds {
    pub tmdb: Option<i32>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MediaItem {
    Movie(Movie),
    Show(Show),
    Season(Season),
    Episode(Episode),
}

pub async fn get(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<MediaItem>> {
    let item_type = match media::get_item_type(&mut *conn, id).await? {
        Some(item_type) => item_type,
        None => return Ok(None),
    };

    let item = match item_type {
        MediaItemType::Movie => movies::get(&mut *conn, id).await?.map(MediaItem::Movie),
        MediaItemType::TvShow => shows::get(&mut *conn, id).await?.map(MediaItem::Show),
        MediaItemType::TvSeason => seasons::get(&mut *conn, id).await?.map(MediaItem::Season),
        MediaItemType::TvEpisode => episodes::get(&mut *conn, id).await?.map(MediaItem::Episode),
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
