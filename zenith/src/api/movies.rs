use atium::responder::Json;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request, Responder};

use crate::db::{self, Db};

use super::ext::OptionExt;
use super::import::import_movie;

pub fn routes(router: &mut Router) {
    router.route("/movies").get(get_movies).post(import_movie);
    router.route("/movies/:id").get(get_movie);
    router.route("/movies/recent").get(get_recent_movies);
}

#[endpoint]
async fn get_movies(req: &mut Request) -> eyre::Result<impl Responder> {
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;
    let movies = db::movies::get_all(&mut conn).await?;
    Ok(Json(movies))
}

#[endpoint]
async fn get_movie(req: &mut Request) -> eyre::Result<impl Responder> {
    let id: i64 = req.param("id")?;
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let movie = db::movies::get(&mut conn, id)
        .await?
        .or_not_found("movie not found")?;

    Ok(Json(movie))
}

#[endpoint]
async fn get_recent_movies(req: &mut Request) -> eyre::Result<impl Responder> {
    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;
    let movies = db::movies::get_recently_added(&mut conn).await?;
    Ok(Json(movies))
}
