mod episodes;
mod seasons;
mod shows;

use atium::router::Router;

use self::episodes::{get_episode, get_episodes};
use self::seasons::{get_season, get_seasons};
use self::shows::{get_recently_updated_shows, get_show, get_shows};

use super::import::{import_episode, import_show};

pub fn routes(router: &mut Router) {
    router.route("/tv/shows").get(get_shows).post(import_show);
    router.route("/tv/shows/:id").get(get_show);
    router
        .route("/tv/shows/recent")
        .get(get_recently_updated_shows);
    router.route("/tv/shows/:id/seasons").get(get_seasons);
    router
        .route("/tv/shows/:show_id/episodes")
        .post(import_episode);
    router.route("/tv/seasons/:id").get(get_season);
    router.route("/tv/seasons/:id/episodes").get(get_episodes);
    router.route("/tv/episodes/:id").get(get_episode);
}
