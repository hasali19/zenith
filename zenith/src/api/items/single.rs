use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use time::OffsetDateTime;
use zenith_server::{Request, Response};

use crate::api::{ApiError, ApiResult};
use crate::db::media::MediaItemType;
use crate::metadata::RefreshRequest;
use crate::{utils, AppState};

#[derive(Serialize)]
struct MediaItem {
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
enum ItemData {
    Movie {
        release_year: Option<i32>,
        duration: f64,
        poster_url: Option<String>,
        backdrop_url: Option<String>,
    },
    TvShow {
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
    },
}

pub(super) async fn get(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

    let sql = "SELECT * FROM media_items WHERE id = ?";
    let row: SqliteRow = sqlx::query(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .ok_or_else(ApiError::not_found)?;

    Ok(Response::new().json(&to_media_item(&row).await?)?)
}

pub(super) async fn refresh_metadata(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await?;

    let (item_type,): (MediaItemType,) =
        sqlx::query_as("SELECT item_type FROM media_items WHERE id = ?")
            .bind(id)
            .fetch_optional(&mut conn)
            .await?
            .ok_or_else(ApiError::not_found)?;

    let req = match item_type {
        MediaItemType::Movie => RefreshRequest::Movie(id),
        MediaItemType::TvShow => RefreshRequest::TvShow(id),
        MediaItemType::TvSeason => RefreshRequest::TvSeason(id),
        MediaItemType::TvEpisode => RefreshRequest::TvEpisode(id),
    };

    state.metadata.enqueue(req);

    Ok(Response::new())
}

async fn to_media_item(row: &SqliteRow) -> sqlx::Result<MediaItem> {
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
    })
}

fn get_tv_show_data(row: &SqliteRow) -> sqlx::Result<ItemData> {
    let primary_img = row.try_get::<Option<_>, _>(9)?.map(utils::get_image_url);
    let backdrop_img = row.try_get::<Option<_>, _>(10)?.map(utils::get_image_url);

    Ok(ItemData::TvShow {
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
    })
}
