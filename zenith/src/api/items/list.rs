use serde::Deserialize;
use sqlx::sqlite::SqliteArguments;
use sqlx::Arguments;
use zenith_server::{Request, Response};

use crate::api::{ApiError, ApiResult};
use crate::db::media::MediaItemType;
use crate::AppState;

use super::common::MediaItem;

#[derive(Deserialize)]
struct Query {
    item_type: Option<QueryItemType>,
    parent_id: Option<i64>,
    watch_status: Option<WatchStatus>,
    is_watching: Option<bool>,
    #[serde(default)]
    sort_by: Vec<SortKey>,
    limit: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum WatchStatus {
    Watched,
    Unwatched,
    NeverWatched,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum SortKey {
    ItemType,
    IndexNumber,
    Name,
    AddedAt,
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
    const SQL: &str = "
        SELECT *
        FROM media_items AS m
        LEFT JOIN user_item_data AS u ON m.id = u.item_id
        WHERE TRUE
    ";

    let mut args = SqliteArguments::default();
    let mut sql = SQL.to_string();

    if let Some(item_type) = query.item_type {
        sql += " AND item_type = ?";
        args.add(MediaItemType::from(item_type));
    }

    if let Some(parent_id) = query.parent_id {
        sql += " AND parent_id = ?";
        args.add(parent_id);
    }

    if let Some(watch_status) = query.watch_status {
        sql += match watch_status {
            WatchStatus::Watched => " AND is_watched = 1",
            WatchStatus::Unwatched => " AND COALESCE(is_watched, 0) = 0",
            WatchStatus::NeverWatched => " AND is_watched IS NULL",
        };
    }

    if let Some(is_watching) = query.is_watching {
        if is_watching {
            sql += " AND position > 0 AND position < duration";
        } else {
            sql += " AND COALESCE(position, 0) = 0"
        }
    }

    if !query.sort_by.is_empty() {
        let keys: Vec<_> = query
            .sort_by
            .into_iter()
            .map(|k| match k {
                SortKey::ItemType => "item_type",
                SortKey::IndexNumber => "index_number",
                SortKey::Name => "name",
                SortKey::AddedAt => "added_at DESC",
            })
            .collect();

        sql += " ORDER BY ";
        sql += keys.join(",").as_str();
    } else {
        sql += " ORDER BY item_type, index_number, name";
    }

    if let Some(limit) = query.limit {
        sql += " LIMIT ";
        sql += limit.to_string().as_str();
    }

    (sql, args)
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
