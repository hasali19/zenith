use axum::extract::{Extension, Path};
use axum::Json;
use axum_codegen::get;

use crate::db::movies::Movie;
use crate::db::{self, Db};

use super::ext::OptionExt;
use super::ApiResult;

#[get("/movies")]
#[response(model = Vec<Movie>)]
pub async fn get_movies(db: Extension<Db>) -> ApiResult<Json<Vec<Movie>>> {
    let mut conn = db.acquire().await?;
    let movies = db::movies::get_all(&mut conn).await?;
    Ok(Json(movies))
}

#[get("/movies/:id")]
#[path(i64)]
#[response(model = Movie)]
pub async fn get_movie(id: Path<i64>, db: Extension<Db>) -> ApiResult<Json<Movie>> {
    let mut conn = db.acquire().await?;

    let movie = db::movies::get(&mut conn, *id)
        .await?
        .or_not_found("movie not found")?;

    Ok(Json(movie))
}

#[get("/movies/recent")]
#[response(model = Vec<Movie>)]
pub async fn get_recent_movies(db: Extension<Db>) -> ApiResult<Json<Vec<Movie>>> {
    let mut conn = db.acquire().await?;
    let movies = db::movies::get_recently_added(&mut conn).await?;
    Ok(Json(movies))
}
