use assert_json_diff::assert_json_include;
use serde_json::json;
use test_macros::test;

use crate::{with_app, TestApp};

#[test(with_app)]
async fn get_all_movies(app: TestApp) {
    let json = app.get("/movies").await;
    assert_eq!(json.as_array().unwrap().len(), 3);
    assert_json_include!(
        actual: json,
        expected:
            json!([
                {"title": "Test Movie 1"},
                {"title": "Test Movie 2"},
                {"title": "Test Movie 3"},
            ])
    );
}

#[test(with_app)]
async fn get_movie_by_id(app: TestApp) {
    let json = app.get("/movies/1").await;
    assert_json_include!(actual: json, expected: json!({"title": "Test Movie 1"}));
}

#[test(with_app)]
async fn get_all_tv_shows(app: TestApp) {
    let json = app.get("/tv/shows").await;
    assert_eq!(json.as_array().unwrap().len(), 3);
    assert_json_include!(
        actual: json,
        expected:
            json!([
                {"name": "Test Show 1"},
                {"name": "Test Show 2"},
                {"name": "Test Show 3"},
            ])
    );
}

#[test(with_app)]
async fn get_show_by_id(app: TestApp) {
    let json = app.get("/tv/shows/4").await;
    assert_json_include!(actual: json, expected: json!({"name": "Test Show 1"}));
}

#[test(with_app)]
async fn get_seasons_for_show(app: TestApp) {
    let json = app.get("/tv/shows/4/seasons").await;
    assert_eq!(json.as_array().unwrap().len(), 2);
    assert_json_include!(
        actual: json,
        expected:
            json!([
                {"show_id": 4, "season_number": 1, "name": "Season 1"},
                {"show_id": 4, "season_number": 2, "name": "Season 2"},
            ])
    );
}

#[test(with_app)]
async fn get_season_by_id(app: TestApp) {
    let json = app.get("/tv/seasons/7").await;
    assert_json_include!(
        actual: json,
        expected: json!({"show_id": 4, "season_number": 1, "name": "Season 1"})
    );
}

#[test(with_app)]
async fn get_episodes_for_show(app: TestApp) {
    let json = app.get("/tv/shows/4/episodes").await;
    assert_eq!(json.as_array().unwrap().len(), 4);
    assert_json_include!(
        actual: json,
        expected:
            json!([
                {"season_id": 7, "episode_number": 1, "name": "Episode 1"},
                {"season_id": 7, "episode_number": 2, "name": "Episode 2"},
                {"season_id": 8, "episode_number": 1, "name": "Episode 1"},
                {"season_id": 8, "episode_number": 2, "name": "Episode 2"},
            ])
    );
}

#[test(with_app)]
async fn get_episodes_for_season(app: TestApp) {
    let json = app.get("/tv/seasons/7/episodes").await;
    assert_eq!(json.as_array().unwrap().len(), 2);
    assert_json_include!(
        actual: json,
        expected:
            json!([
                {"season_id": 7, "episode_number": 1, "name": "Episode 1"},
                {"season_id": 7, "episode_number": 2, "name": "Episode 2"},
            ])
    );
}

#[test(with_app)]
async fn get_episode_by_id(app: TestApp) {
    let json = app.get("/tv/episodes/9").await;
    assert_json_include!(
        actual: json,
        expected: json!({"season_id": 7, "episode_number": 1, "name": "Episode 1"})
    );
}
