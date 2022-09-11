use insta::assert_json_snapshot;
use test_macros::test;

use crate::{with_app, TestApp};

macro_rules! test_snapshot {
    ($name:ident, $path:expr) => {
        #[test(with_app)]
        async fn $name(app: TestApp) {
            assert_json_snapshot!(app.get($path).await);
        }
    };
}

test_snapshot!(get_all_movies, "/movies");
test_snapshot!(get_movie_by_id, "/movies/1");
test_snapshot!(get_all_tv_shows, "/tv/shows");
test_snapshot!(get_show_by_id, "/tv/shows/4");
test_snapshot!(get_seasons_for_show, "/tv/shows/4/seasons");
test_snapshot!(get_season_by_id, "/tv/seasons/7");
test_snapshot!(get_episodes_for_show, "/tv/shows/4/episodes");
test_snapshot!(get_episodes_for_season, "/tv/seasons/7/episodes");
test_snapshot!(get_episode_by_id, "/tv/episodes/9");
