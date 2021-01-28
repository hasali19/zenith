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
