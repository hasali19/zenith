use hyper::{Body, Request, StatusCode};
use insta::assert_json_snapshot;
use pretty_assertions::assert_eq;
use test_macros::test;
use tower::ServiceExt;

use crate::{with_app, TestApp};

macro_rules! test_snapshot {
    ($name:ident, $path:expr $(,)?) => {
        #[test(with_app)]
        async fn $name(app: TestApp) {
            assert_json_snapshot!(app.get($path).await);
        }
    };
}

test_snapshot!(get_all_movies, "/movies");
test_snapshot!(get_recently_added_movies, "/movies/recent");
test_snapshot!(get_movie_by_id, "/movies/1");
test_snapshot!(get_all_tv_shows, "/shows");
test_snapshot!(get_recently_updated_shows, "/shows/recent");
test_snapshot!(get_seasons_for_show, "/shows/4/seasons");
test_snapshot!(get_episodes_for_show, "/shows/4/episodes");
test_snapshot!(get_episodes_for_season, "/seasons/7/episodes");
test_snapshot!(get_single_item_by_id, "/items/1");
test_snapshot!(
    get_multiple_items_by_ids,
    "/items?ids[]=1&ids[]=4&ids[]=7&ids[]=9",
);
test_snapshot!(get_continue_watching, "/items/continue_watching");

#[test(with_app)]
async fn update_progress(app: TestApp) {
    let res = app
        .router
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/progress/1?position=100")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let mut conn = app.db.acquire().await.unwrap();
    let (position,): (f64,) =
        sqlx::query_as("SELECT position FROM user_item_data WHERE item_id = 1")
            .fetch_one(&mut conn)
            .await
            .unwrap();

    assert_eq!(position, 100.0);
}
