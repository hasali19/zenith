pub mod episodes;
pub mod seasons;
pub mod shows;

use actix_web::web::post;
use actix_web::{web, Scope};
use web::get;

use super::import;

pub fn service(path: &str) -> Scope {
    let shows = web::resource("/shows")
        .route(get().to(shows::get_shows))
        .route(post().to(import::import_show));

    web::scope(path)
        .service(shows)
        .route("/shows/recent", get().to(shows::get_recently_updated_shows))
        .route("/shows/{id}", get().to(shows::get_show))
        .route("/shows/{id}/seasons", get().to(seasons::get_seasons))
        .route("/shows/{id}/episodes", post().to(import::import_episode))
        .route("/seasons/{id}", get().to(seasons::get_season))
        .route("/seasons/{id}/episodes", get().to(episodes::get_episodes))
        .route("/episodes/{id}", get().to(episodes::get_episode))
}
