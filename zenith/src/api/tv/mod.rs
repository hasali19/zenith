mod episodes;
mod seasons;
mod shows;

use actix_web::{web, Scope};
use web::get;

pub fn service(path: &str) -> Scope {
    web::scope(path)
        .route("/shows", get().to(shows::get_shows))
        .route("/shows/recent", get().to(shows::get_recently_updated_shows))
        .route("/shows/{id}", get().to(shows::get_show))
        .route("/shows/{id}/seasons", get().to(seasons::get_seasons))
        .route("/seasons/{id}", get().to(seasons::get_season))
        .route("/seasons/{id}/episodes", get().to(episodes::get_episodes))
        .route("/episodes/{id}", get().to(episodes::get_episode))
}
