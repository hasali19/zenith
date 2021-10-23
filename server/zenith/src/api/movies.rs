use actix_web::get;
use actix_web::web::{Json, Path};

use crate::db::movies::Movie;
use crate::db::{self, Db};

use super::ext::OptionExt;
use super::ApiResult;

#[get("/movies")]
pub async fn get_movies(db: Db) -> ApiResult<Json<Vec<Movie>>> {
    let mut conn = db.acquire().await?;
    let movies = db::movies::get_all(&mut conn).await?;
    Ok(Json(movies))
}

#[get("/movies/{id}")]
pub async fn get_movie(id: Path<i64>, db: Db) -> ApiResult<Json<Movie>> {
    let mut conn = db.acquire().await?;

    let movie = db::movies::get(&mut conn, *id)
        .await?
        .or_not_found("movie not found")?;

    Ok(Json(movie))
}

#[get("/movies/recent")]
pub async fn get_recent_movies(db: Db) -> ApiResult<Json<Vec<Movie>>> {
    let mut conn = db.acquire().await?;
    let movies = db::movies::get_recently_added(&mut conn).await?;
    Ok(Json(movies))
}
