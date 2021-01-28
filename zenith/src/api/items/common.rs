use core::f64;

use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};
use time::OffsetDateTime;

use crate::db::media::MediaItemType;
use crate::utils;

#[derive(Serialize)]
pub struct MediaItem {
    id: i64,
    parent_id: i64,
    name: Option<String>,
    overview: Option<String>,
    #[serde(flatten)]
    data: ItemData,
    added_at: i64,
    updated_at: Option<i64>,
}

#[derive(Serialize)]
#[serde(tag = "item_type", rename_all = "snake_case")]
pub enum ItemData {
    Movie {
        release_year: Option<i32>,
        duration: f64,
        poster_url: Option<String>,
        backdrop_url: Option<String>,
        user_data: UserData,
    },
    TvShow {
        start_year: Option<i32>,
        poster_url: Option<String>,
        backdrop_url: Option<String>,
    },
    TvSeason {
        season_number: i32,
        poster_url: Option<String>,
    },
    TvEpisode {
        episode_number: i32,
        duration: f64,
        thumbnail_url: Option<String>,
        user_data: UserData,
    },
}

#[derive(Serialize)]
pub struct UserData {
    position: f64,
    is_watched: bool,
}

impl<'r> FromRow<'r, SqliteRow> for MediaItem {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        to_media_item(row)
    }
}

pub fn to_media_item(row: &SqliteRow) -> sqlx::Result<MediaItem> {
    let item = MediaItem {
        id: row.try_get(0)?,
        parent_id: row.try_get(1)?,
        name: row.try_get(4)?,
        overview: row.try_get(7)?,
        data: match row.try_get(2)? {
            MediaItemType::Movie => get_movie_data(row)?,
            MediaItemType::TvShow => get_tv_show_data(row)?,
            MediaItemType::TvSeason => get_tv_season_data(row)?,
            MediaItemType::TvEpisode => get_tv_episode_data(row)?,
        },
        added_at: row.try_get(12)?,
        updated_at: row.try_get(13)?,
    };

    Ok(item)
}

fn get_movie_data(row: &SqliteRow) -> sqlx::Result<ItemData> {
    let primary_img = row.try_get::<Option<_>, _>(9)?.map(utils::get_image_url);
    let backdrop_img = row.try_get::<Option<_>, _>(10)?.map(utils::get_image_url);

    let release_date: Option<i64> = row.try_get(6)?;
    let release_year = release_date.map(|v| OffsetDateTime::from_unix_timestamp(v).year());

    Ok(ItemData::Movie {
        release_year,
        duration: row.try_get(8)?,
        poster_url: primary_img,
        backdrop_url: backdrop_img,
        user_data: get_user_data(row)?,
    })
}

fn get_tv_show_data(row: &SqliteRow) -> sqlx::Result<ItemData> {
    let primary_img = row.try_get::<Option<_>, _>(9)?.map(utils::get_image_url);
    let backdrop_img = row.try_get::<Option<_>, _>(10)?.map(utils::get_image_url);

    let release_date: Option<i64> = row.try_get(6)?;
    let start_year = release_date.map(|v| OffsetDateTime::from_unix_timestamp(v).year());

    Ok(ItemData::TvShow {
        start_year,
        poster_url: primary_img,
        backdrop_url: backdrop_img,
    })
}

fn get_tv_season_data(row: &SqliteRow) -> sqlx::Result<ItemData> {
    let primary_img = row.try_get::<Option<_>, _>(9)?.map(utils::get_image_url);

    Ok(ItemData::TvSeason {
        season_number: row.try_get(5)?,
        poster_url: primary_img,
    })
}

fn get_tv_episode_data(row: &SqliteRow) -> sqlx::Result<ItemData> {
    let primary_img = row.try_get::<Option<_>, _>(9)?.map(utils::get_image_url);

    Ok(ItemData::TvEpisode {
        episode_number: row.try_get(5)?,
        duration: row.try_get(8)?,
        thumbnail_url: primary_img,
        user_data: get_user_data(row)?,
    })
}

fn get_user_data(row: &SqliteRow) -> sqlx::Result<UserData> {
    let position: Option<f64> = row.try_get(15)?;
    let is_watched: Option<bool> = row.try_get(16)?;

    Ok(UserData {
        position: position.unwrap_or(0.0),
        is_watched: is_watched.unwrap_or(false),
    })
}
