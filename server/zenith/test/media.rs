use hyper::{Body, Request, StatusCode};
use insta::assert_json_snapshot;
use pretty_assertions::assert_eq;
use serde_json::json;
use test_macros::test;
use tower::ServiceExt;

use crate::{json_body, with_app, TestApp};

macro_rules! test_snapshot {
    ($name:ident, $path:expr $(,)?) => {
        #[test(with_app)]
        async fn $name(mut app: TestApp) {
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
async fn get_collections(mut app: TestApp) {
    let mut conn = app.db.acquire().await.unwrap();

    db::collections::create(
        &mut *conn,
        db::collections::NewCollection {
            name: "collection 1",
        },
    )
    .await
    .unwrap();

    db::collections::create(
        &mut *conn,
        db::collections::NewCollection {
            name: "collection 2",
        },
    )
    .await
    .unwrap();

    assert_json_snapshot!(app.get("/collections").await);
}

#[test(with_app)]
async fn get_collection(mut app: TestApp) {
    let mut conn = app.db.acquire().await.unwrap();

    db::collections::create(
        &mut *conn,
        db::collections::NewCollection {
            name: "collection 1",
        },
    )
    .await
    .unwrap();

    let collection_2 = db::collections::create(
        &mut *conn,
        db::collections::NewCollection {
            name: "collection 2",
        },
    )
    .await
    .unwrap();

    assert_json_snapshot!(app.get(&format!("/collections/{}", collection_2.id)).await);
}

#[test(with_app)]
async fn create_collection(mut app: TestApp) {
    let body = json!({
        "name": "collection 1"
    });

    let res = app
        .req_json(
            Request::builder()
                .method("POST")
                .uri("/collections")
                .header("Content-Type", "application/json")
                .body(json_body(&body))
                .unwrap(),
        )
        .await;

    assert_json_snapshot!(res);

    let mut conn = app.db.acquire().await.unwrap();

    let collections: Vec<String> = sqlx::query_scalar("SELECT name FROM collections")
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    assert_eq!(collections.len(), 1);
    assert_eq!(collections[0], "collection 1");
}

#[test(with_app)]
async fn delete_collection(mut app: TestApp) {
    let mut conn = app.db.acquire().await.unwrap();

    let collection = db::collections::create(
        &mut *conn,
        db::collections::NewCollection {
            name: "collection 1",
        },
    )
    .await
    .unwrap();

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/collections/{}", collection.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let mut conn = app.db.acquire().await.unwrap();

    let collections: Vec<String> = sqlx::query_scalar("SELECT name FROM collections")
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    assert!(collections.is_empty());
}

#[test(with_app)]
async fn update_collection(mut app: TestApp) {
    let mut conn = app.db.acquire().await.unwrap();

    let collection = db::collections::create(
        &mut *conn,
        db::collections::NewCollection {
            name: "collection 1",
        },
    )
    .await
    .unwrap();

    let body = json!({
        "meta": {
            "name": "collection 2",
            "overview": "overview of the collection",
        },
        "items": [1, 2],
    });

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/collections/{}", collection.id))
                .header("Content-Type", "application/json")
                .body(json_body(&body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let mut conn = app.db.acquire().await.unwrap();

    let collections = db::collections::get_all(&mut *conn).await.unwrap();

    assert_eq!(collections[0].name, "collection 2");
    assert_eq!(
        collections[0].overview.as_deref(),
        Some("overview of the collection")
    );

    let items = db::items::query(
        &mut *conn,
        db::items::Query {
            collection_id: Some(collection.id),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    assert!(items.iter().any(|i| i.id == 1));
    assert!(items.iter().any(|i| i.id == 2));
}

#[test(with_app)]
async fn update_progress(mut app: TestApp) {
    let cookie = app.login().await;

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/progress/1?position=100")
                .header("Cookie", &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let mut conn = app.db.acquire().await.unwrap();
    let position: f64 =
        sqlx::query_scalar("SELECT position FROM media_item_user_data WHERE item_id = 1")
            .fetch_one(&mut *conn)
            .await
            .unwrap();

    assert_eq!(position, 100.0);
}

#[test(with_app)]
async fn update_user_data_for_single_movie(mut app: TestApp) {
    let cookie = app.login().await;

    let body = json!({
        "is_watched": true,
        "position": 100.0,
    });

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/items/1/user_data")
                .header("Cookie", &cookie)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let mut conn = app.db.acquire().await.unwrap();
    let (position, is_watched): (f64, bool) =
        sqlx::query_as("SELECT position, is_watched FROM media_item_user_data WHERE item_id = 1")
            .fetch_one(&mut *conn)
            .await
            .unwrap();

    assert_eq!(position, 100.0);
    assert_eq!(is_watched, true);
}

#[test(with_app)]
async fn update_user_data_for_show(mut app: TestApp) {
    let cookie = app.login().await;

    let body = json!({
        "is_watched": true,
        "position": 100.0,
    });

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/items/4/user_data")
                .header("Cookie", &cookie)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    for id in [9, 10, 11, 12] {
        let mut conn = app.db.acquire().await.unwrap();

        let sql = "SELECT position, is_watched FROM media_item_user_data WHERE item_id = ?";
        let (position, is_watched): (f64, bool) = sqlx::query_as(sql)
            .bind(id)
            .fetch_one(&mut *conn)
            .await
            .unwrap();

        assert_eq!(position, 100.0);
        assert_eq!(is_watched, true);
    }
}

#[test(with_app)]
async fn update_user_data_for_season(mut app: TestApp) {
    let cookie = app.login().await;

    let body = json!({
        "is_watched": true,
        "position": 100.0,
    });

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/items/8/user_data")
                .header("Cookie", &cookie)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    for id in [11, 12] {
        let mut conn = app.db.acquire().await.unwrap();

        let sql = "SELECT position, is_watched FROM media_item_user_data WHERE item_id = ?";
        let (position, is_watched): (f64, bool) = sqlx::query_as(sql)
            .bind(id)
            .fetch_one(&mut *conn)
            .await
            .unwrap();

        assert_eq!(position, 100.0);
        assert_eq!(is_watched, true);
    }
}

#[test(with_app)]
async fn delete_item(mut app: TestApp) {
    let cookie = app.login().await;

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/items/1")
                .header("Cookie", &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let mut conn = app.db.acquire().await.unwrap();
    let result: Option<i64> = sqlx::query_scalar("SELECT 1 FROM media_items WHERE id = 1")
        .fetch_optional(&mut *conn)
        .await
        .unwrap();

    assert!(result.is_none());
}
