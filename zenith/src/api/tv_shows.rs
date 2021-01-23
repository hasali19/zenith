use sqlx::SqliteConnection;

use crate::db::media::MediaItemType;
use crate::server::{App, JsonResponse, Request};
use crate::{utils, AppState};

use super::{ApiError, ApiResult};

pub fn configure(app: &mut App<AppState>) {
    app.get("/api/tv_shows", get_tv_shows);
    app.get("/api/tv_shows/:id", get_tv_show);
}

#[derive(serde::Serialize)]
pub struct TvShow {
    id: i64,
    name: String,
    poster_url: Option<String>,
    season_count: u32,
}

async fn get_tv_shows(state: AppState, _: Request) -> ApiResult<JsonResponse> {
    let mut conn = state.db.acquire().await.unwrap();

    let sql = "
        SELECT id, name, primary_image, (SELECT COUNT(id) FROM media_items WHERE parent_id = show.id)
        FROM media_items AS show WHERE item_type = ?
        ORDER BY name
    ";

    let shows: Vec<(i64, String, Option<String>, i64)> = sqlx::query_as(sql)
        .bind(MediaItemType::TvShow)
        .fetch_all(&mut conn)
        .await
        .map_err(|_| ApiError::internal_server_error())?;

    let res: Vec<TvShow> = shows
        .into_iter()
        .map(|(id, name, primary, season_count)| TvShow {
            id,
            name,
            poster_url: primary.as_deref().map(utils::get_image_url),
            season_count: season_count as u32,
        })
        .collect();

    Ok(JsonResponse::from(res))
}

#[derive(serde::Serialize)]
struct TvShowFull {
    id: i64,
    name: String,
    overview: Option<String>,
    poster_url: Option<String>,
    backdrop_url: Option<String>,
    seasons: Vec<TvSeason>,
}

#[derive(serde::Serialize)]
struct TvSeason {
    id: i64,
    season: u32,
    name: Option<String>,
    overview: Option<String>,
    poster_url: Option<String>,
    episodes: Vec<TvEpisode>,
}

#[derive(serde::Serialize)]
pub struct TvEpisode {
    id: i64,
    episode: u32,
    overview: Option<String>,
    thumbnail_url: Option<String>,
    stream_id: i64,
    duration: f64,
}

async fn get_tv_show(state: AppState, req: Request) -> ApiResult<JsonResponse> {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await.unwrap();

    type Row = (i64, String, Option<String>, Option<String>, Option<String>);

    let sql = "
        SELECT id, name, overview, primary_image, backdrop_image
        FROM media_items WHERE id = ? AND item_type = ?
    ";

    let (id, name, overview, poster, backdrop): Row = sqlx::query_as(sql)
        .bind(id)
        .bind(MediaItemType::TvShow)
        .fetch_optional(&mut conn)
        .await
        .map_err(|_| ApiError::internal_server_error())?
        .ok_or_else(ApiError::not_found)?;

    let show = TvShowFull {
        id,
        name,
        overview,
        poster_url: poster.as_deref().map(utils::get_image_url),
        backdrop_url: backdrop.as_deref().map(utils::get_image_url),
        seasons: get_seasons_by_show_id(&mut conn, id).await?,
    };

    Ok(JsonResponse::from(show))
}

async fn get_seasons_by_show_id(
    conn: &mut SqliteConnection,
    show_id: i64,
) -> ApiResult<Vec<TvSeason>> {
    let sql = "
        SELECT id, index_number, name, overview, primary_image
        FROM media_items
        WHERE parent_id = ? AND item_type = ?
        ORDER BY index_number
    ";

    type Row = (i64, i64, Option<String>, Option<String>, Option<String>);

    let results: Vec<Row> = sqlx::query_as(sql)
        .bind(show_id)
        .bind(MediaItemType::TvSeason)
        .fetch_all(&mut *conn)
        .await
        .map_err(|_| ApiError::internal_server_error())?;

    let mut seasons = vec![];

    for (id, index_number, name, overview, primary) in results {
        seasons.push(TvSeason {
            id,
            season: index_number as u32,
            name,
            overview,
            poster_url: primary.as_deref().map(utils::get_image_url),
            episodes: get_episodes_by_season_id(&mut *conn, id).await?,
        })
    }

    Ok(seasons)
}

async fn get_episodes_by_season_id(
    conn: &mut SqliteConnection,
    season_id: i64,
) -> ApiResult<Vec<TvEpisode>> {
    let sql = "
        SELECT episode.id, episode.index_number, episode.overview, episode.primary_image,
               file.id, file.duration
        FROM media_items AS episode
        JOIN video_files AS file ON episode.id = file.item_id
        WHERE episode.parent_id = ? AND episode.item_type = ?
        ORDER BY episode.index_number
    ";

    type Row = (i64, i64, Option<String>, Option<String>, i64, f64);

    let results: Vec<Row> = sqlx::query_as(sql)
        .bind(season_id)
        .bind(MediaItemType::TvEpisode)
        .fetch_all(conn)
        .await
        .map_err(|_| ApiError::internal_server_error())?;

    let mut episodes = vec![];

    for (id, episode, overview, primary, file_id, duration) in results {
        episodes.push(TvEpisode {
            id,
            episode: episode as u32,
            overview,
            thumbnail_url: primary.as_deref().map(utils::get_image_url),
            stream_id: file_id,
            duration,
        });
    }

    Ok(episodes)
}
