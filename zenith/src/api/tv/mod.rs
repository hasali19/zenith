mod episodes;
mod seasons;
mod shows;

use zenith_http::App;

use crate::AppState;

pub fn configure(app: &mut App<AppState>) {
    app.get("/api/tv/shows", shows::get_shows);
    app.get("/api/tv/shows/:id", shows::get_show);
    app.get("/api/tv/shows/:id/seasons", seasons::get_seasons);

    app.get("/api/tv/shows/recent", shows::get_recently_updated_shows);

    app.get("/api/tv/seasons/:id", seasons::get_season);
    app.get("/api/tv/seasons/:id/episodes", episodes::get_episodes);

    app.get("/api/tv/episodes/:id", episodes::get_episode);
}
