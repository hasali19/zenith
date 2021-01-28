use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteArguments, SqliteRow};
use sqlx::{Arguments, FromRow, Row};
use time::OffsetDateTime;
use zenith_server::{Request, Response};

use crate::api::{ApiError, ApiResult};
use crate::db::media::MediaItemType;
use crate::{utils, AppState};

#[derive(Deserialize)]
struct Query {
    item_type: Option<QueryItemType>,
    parent_id: Option<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum QueryItemType {
    Movie,
    TvShow,
    TvSeason,
    TvEpisode,
}

#[derive(Deserialize)]
enum QuerySortKey {
    Name,
    IndexNumber,
}

#[derive(Serialize)]
struct MediaItem {
    id: i64,
    parent_id: i64,
    name: Option<String>,
    #[serde(flatten)]
    data: ItemData,
}

#[derive(Serialize)]
#[serde(tag = "item_type", rename_all = "snake_case")]
enum ItemData {
    Movie {
        release_year: Option<i32>,
        poster_url: Option<String>,
    },
    TvShow {
        start_year: Option<i32>,
        poster_url: Option<String>,
    },
    TvSeason {
        season_number: i32,
        poster_url: Option<String>,
    },
    TvEpisode {
        episode_number: i32,
        overview: Option<String>,
        thumbnail_url: Option<String>,
    },
}

pub(super) async fn get(state: AppState, req: Request) -> ApiResult {
    let query: Query = req
        .query()
        .map_err(|e| ApiError::bad_request().body(e.to_string()))?;

    get_query(state, query).await
}

pub(super) async fn get_children(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut query: Query = req
        .query()
        .map_err(|e| ApiError::bad_request().body(e.to_string()))?;

    query.parent_id = Some(id);

    get_query(state, query).await
}

async fn get_query(state: AppState, query: Query) -> ApiResult {
    let mut conn = state.db.acquire().await?;

    let (sql, args) = build_sql_query(query);
    let items: Vec<MediaItem> = sqlx::query_as_with(&sql, args).fetch_all(&mut conn).await?;

    Ok(Response::new().json(&items)?)
}

fn build_sql_query<'a>(query: Query) -> (String, SqliteArguments<'a>) {
    let mut args = SqliteArguments::default();
    let mut sql = "SELECT * FROM media_items WHERE TRUE".to_string();

    if let Some(item_type) = query.item_type {
        sql.push_str(" AND item_type = ?");
        args.add(MediaItemType::from(item_type));
    }

    if let Some(parent_id) = query.parent_id {
        sql.push_str(" AND parent_id = ?");
        args.add(parent_id);
    }

    sql += " ORDER BY item_type, index_number, name";

    (sql, args)
}

impl<'r> FromRow<'r, SqliteRow> for MediaItem {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let primary_img = row.try_get::<Option<_>, _>(9)?.map(utils::get_image_url);

        let release_date: Option<i64> = row.try_get(6)?;
        let release_year = release_date.map(|v| OffsetDateTime::from_unix_timestamp(v).year());

        let item = MediaItem {
            id: row.try_get(0)?,
            parent_id: row.try_get(1)?,
            name: row.try_get(4)?,
            data: match row.try_get(2)? {
                MediaItemType::Movie => ItemData::Movie {
                    release_year,
                    poster_url: primary_img,
                },
                MediaItemType::TvShow => ItemData::TvShow {
                    start_year: release_year,
                    poster_url: primary_img,
                },
                MediaItemType::TvSeason => ItemData::TvSeason {
                    season_number: row.try_get(5)?,
                    poster_url: primary_img,
                },
                MediaItemType::TvEpisode => ItemData::TvEpisode {
                    episode_number: row.try_get(5)?,
                    overview: row.try_get(7)?,
                    thumbnail_url: primary_img,
                },
            },
        };

        Ok(item)
    }
}

impl From<QueryItemType> for MediaItemType {
    fn from(item_type: QueryItemType) -> Self {
        match item_type {
            QueryItemType::Movie => MediaItemType::Movie,
            QueryItemType::TvShow => MediaItemType::TvShow,
            QueryItemType::TvSeason => MediaItemType::TvSeason,
            QueryItemType::TvEpisode => MediaItemType::TvEpisode,
        }
    }
}
