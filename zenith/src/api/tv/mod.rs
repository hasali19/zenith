pub mod episodes;
pub mod seasons;
pub mod shows;

use actix_web::web::{self, get, post, ServiceConfig};
use episodes::{get_episode, get_episodes};
use seasons::{get_season, get_seasons};
use shows::{get_recently_updated_shows, get_show, get_shows};

use super::import::{import_episode, import_show};

pub fn configure(config: &mut ServiceConfig) {
    let shows = web::resource("/tv/shows")
        .route(get().to(get_shows))
        .route(post().to(import_show));

    config
        .service(shows)
        .route("/tv/shows/recent", get().to(get_recently_updated_shows))
        .route("/tv/shows/{id}", get().to(get_show))
        .route("/tv/shows/{id}/seasons", get().to(get_seasons))
        .route("/tv/shows/{id}/episodes", post().to(import_episode))
        .route("/tv/seasons/{id}", get().to(get_season))
        .route("/tv/seasons/{id}/episodes", get().to(get_episodes))
        .route("/tv/episodes/{id}", get().to(get_episode));
}
