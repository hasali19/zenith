use crate::server::{App, JsonResponse, Request};
use crate::{utils, AppState};

use super::{ApiError, ApiResult};

pub fn configure(app: &mut App<AppState>) {
    app.get("/api/movies", get_movies);
    app.get("/api/movies/:id", get_movie);
}

#[derive(serde::Serialize)]
pub struct MovieListItem {
    id: i64,
    title: String,
    year: Option<i32>,
    poster_url: Option<String>,
}

async fn get_movies(state: AppState, _: Request) -> ApiResult<JsonResponse> {
    let mut conn = state.db.acquire().await.unwrap();

    let sql = "
        SELECT id, name, CAST(strftime('%Y', datetime(release_date, 'unixepoch')) as INTEGER), primary_image
        FROM media_items WHERE item_type = 1
        ORDER BY name
    ";

    let movies: Vec<(i64, String, Option<i32>, Option<String>)> = sqlx::query_as(sql)
        .fetch_all(&mut conn)
        .await
        .map_err(|_| ApiError::internal_server_error())?;

    let res: Vec<MovieListItem> = movies
        .into_iter()
        .map(|(id, title, year, primary)| MovieListItem {
            id,
            title,
            year,
            poster_url: primary.as_deref().map(utils::get_image_url),
        })
        .collect();

    Ok(JsonResponse::from(res))
}

#[derive(serde::Serialize)]
pub struct MovieDetails {
    id: i64,
    title: String,
    year: Option<i32>,
    overview: Option<String>,
    poster_url: Option<String>,
    backdrop_url: Option<String>,
    stream_id: i64,
    duration: f64,
}

async fn get_movie(state: AppState, req: Request) -> ApiResult<JsonResponse> {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mut conn = state.db.acquire().await.unwrap();

    type Row = (
        i64,
        String,
        Option<i32>,
        Option<String>,
        Option<String>,
        Option<String>,
        i64,
        f64,
    );

    let sql = "
        SELECT movie.id, name, CAST(strftime('%Y', datetime(release_date, 'unixepoch')) as INTEGER),
               overview, primary_image, backdrop_image, file.id, file.duration
        FROM media_items AS movie
        JOIN video_files AS file ON movie.id = file.item_id
        WHERE movie.id = ? AND item_type = 1
    ";

    let (id, title, year, overview, poster, backdrop, file_id, duration): Row = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(|_| ApiError::internal_server_error())?
        .ok_or_else(ApiError::not_found)?;

    Ok(JsonResponse::from(MovieDetails {
        id,
        title,
        year,
        overview,
        poster_url: poster.as_deref().map(utils::get_image_url),
        backdrop_url: backdrop.as_deref().map(utils::get_image_url),
        stream_id: file_id,
        duration,
    }))
}
