use axum::body::Body;
use db::images::ImageSourceType;
use hyper::{Request, StatusCode};
use serde_json::json;
use test_macros::test;
use tower::ServiceExt;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::{TestApp, json_body, with_app};

#[test(with_app)]
async fn find_match_for_movie(mut app: TestApp) {
    let cookie = app.login().await;

    Mock::given(method("GET"))
        .and(path("/search/movie"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "page": 0,
            "results": [
                {
                    "id": 123,
                    "title": "Test Movie 1",
                },
            ],
            "total_results": 0,
            "total_pages": 0,
        })))
        .mount(&app.mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/movie/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123,
            "title": "Test Movie 1",
            "genres": [
                {"id": 1, "name": "Action"},
                {"id": 2, "name": "Comedy"},
            ],
            "external_ids": {"imdb_id": null},
            "release_dates": {
                "results": [
                    {
                        "iso_3166_1": "GB",
                        "release_dates": [
                            {"certification": "12A"},
                        ],
                    },
                ],
            },
            "videos": {
                "results": [],
            },
            "credits": {
                "cast": [],
                "crew": [],
            },
            "poster_path": "/poster.jpg",
        })))
        .mount(&app.mock_server)
        .await;

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/metadata/1/find_match")
                .header("Cookie", cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let mut conn = app.db.acquire().await.unwrap();
    let item = db::items::get(&mut conn, 1).await.unwrap().unwrap();

    assert!(item.poster.is_some());

    let poster = db::images::get(&mut conn, item.poster.as_deref().unwrap())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(poster.source_type, ImageSourceType::Tmdb);
    assert_eq!(poster.source, "/poster.jpg");
}

#[test(with_app)]
async fn find_match_for_nonexistent_item(mut app: TestApp) {
    let cookie = app.login().await;

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/metadata/4269/find_match")
                .header("Cookie", cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[test(with_app)]
async fn refresh_metadata_for_show(mut app: TestApp) {
    let cookie = app.login().await;

    let mut conn = app.db.acquire_write().await.unwrap();

    db::items::update_metadata(
        &mut conn,
        4,
        db::items::UpdateMetadata {
            metadata_provider: Some(Some(db::media::MetadataProvider::Tmdb)),
            metadata_provider_key: Some(Some("123")),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    drop(conn);

    Mock::given(method("GET"))
        .and(path("/tv/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123,
            "name": "Test Show 1",
            "genres": [
                {"id": 1, "name": "Action"},
                {"id": 2, "name": "Comedy"},
            ],
            "external_ids": {"imdb_id": null},
            "content_ratings": {
                "results": [],
            },
            "videos": {
                "results": [],
            },
            "aggregate_credits": {
                "cast": [],
                "crew": [],
            },
        })))
        .mount(&app.mock_server)
        .await;

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/metadata/4/refresh")
                .header("Cookie", cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}

#[test(with_app)]
async fn set_match_for_show(mut app: TestApp) {
    let cookie = app.login().await;

    Mock::given(method("GET"))
        .and(path("/tv/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123,
            "name": "Test Show 1",
            "genres": [
                {"id": 1, "name": "Action"},
                {"id": 2, "name": "Comedy"},
            ],
            "external_ids": {"imdb_id": null},
            "content_ratings": {
                "results": [],
            },
            "videos": {
                "results": [],
            },
            "aggregate_credits": {
                "cast": [],
                "crew": [],
            },
        })))
        .mount(&app.mock_server)
        .await;

    let body = json_body(&json!({
        "tmdb_id": 123,
    }));

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/metadata/4/set_match")
                .header("Content-Type", "application/json")
                .header("Cookie", cookie)
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}

#[test(with_app)]
async fn set_match_for_season(mut app: TestApp) {
    let cookie = app.login().await;

    Mock::given(method("GET"))
        .and(path("/tv/123/season/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123,
            "external_ids": {"imdb_id": null},
            "videos": {
                "results": [],
            },
            "aggregate_credits": {
                "cast": [],
                "crew": [],
            },
        })))
        .mount(&app.mock_server)
        .await;

    let body = json_body(&json!({
        "tmdb_id": 123,
        "season_number": 1,
    }));

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/metadata/7/set_match")
                .header("Content-Type", "application/json")
                .header("Cookie", cookie)
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}

#[test(with_app)]
async fn set_match_for_unmatched_episode(mut app: TestApp) {
    let cookie = app.login().await;

    Mock::given(method("GET"))
        .and(path("/tv/123/season/1/episode/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123,
            "name": "Test Episode 1",
            "external_ids": {"imdb_id": null},
            "images": {
                "stills": [],
            },
            "videos": {
                "results": [],
            },
            "credits": {
                "cast": [],
                "crew": [],
            },
        })))
        .mount(&app.mock_server)
        .await;

    let body = json_body(&json!({
        "tmdb_id": 123,
        "season_number": 1,
        "episode_number": 1,
    }));

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/metadata/9/set_match")
                .header("Content-Type", "application/json")
                .header("Cookie", cookie)
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}

#[test(with_app)]
async fn set_match_for_previously_matched_episode(mut app: TestApp) {
    let cookie = app.login().await;

    let mut conn = app.db.acquire_write().await.unwrap();

    db::items::update_metadata(
        &mut conn,
        9,
        db::items::UpdateMetadata {
            metadata_provider: Some(Some(db::media::MetadataProvider::Tmdb)),
            metadata_provider_key: Some(Some("123:1:1")),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    drop(conn);

    Mock::given(method("GET"))
        .and(path("/tv/123/season/1/episode/2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123,
            "name": "Test Episode 1",
            "external_ids": {"imdb_id": null},
            "images": {
                "stills": [],
            },
            "videos": {
                "results": [],
            },
            "credits": {
                "cast": [],
                "crew": [],
            },
        })))
        .mount(&app.mock_server)
        .await;

    let body = json_body(&json!({
        "season_number": 1,
        "episode_number": 2,
    }));

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/metadata/9/set_match")
                .header("Content-Type", "application/json")
                .header("Cookie", cookie)
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}

#[test(with_app)]
async fn set_match_for_unmatched_episode_with_matched_show(mut app: TestApp) {
    let cookie = app.login().await;

    let mut conn = app.db.acquire_write().await.unwrap();

    db::items::update_metadata(
        &mut conn,
        4,
        db::items::UpdateMetadata {
            metadata_provider: Some(Some(db::media::MetadataProvider::Tmdb)),
            metadata_provider_key: Some(Some("123")),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    drop(conn);

    Mock::given(method("GET"))
        .and(path("/tv/123/season/1/episode/2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123,
            "name": "Test Episode 1",
            "external_ids": {"imdb_id": null},
            "images": {
                "stills": [],
            },
            "videos": {
                "results": [],
            },
            "credits": {
                "cast": [],
                "crew": [],
            },
        })))
        .mount(&app.mock_server)
        .await;

    let body = json_body(&json!({
        "season_number": 1,
        "episode_number": 2,
    }));

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/metadata/9/set_match")
                .header("Content-Type", "application/json")
                .header("Cookie", cookie)
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}
