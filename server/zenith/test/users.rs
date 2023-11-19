use hyper::{Body, Request, StatusCode};
use insta::assert_json_snapshot;
use serde_json::json;
use test_macros::test;
use tower::ServiceExt;

use crate::{with_app, TestApp};

#[test(with_app)]
async fn get_users(mut app: TestApp) {
    // hash of "password"
    const PASSWORD_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$cV946Lj8LNOX2F7ClooV3A$bZQHhEei6/LLmfpyuX2Hqupj416sfZ8/LtxmUg0FZqI";

    let mut conn = app.db.acquire().await.unwrap();

    db::users::create(
        &mut *conn,
        db::users::NewUser {
            username: "test2",
            password_hash: PASSWORD_HASH,
        },
    )
    .await
    .unwrap();

    assert_json_snapshot!(app.get("/users").await);
}

#[test(with_app)]
async fn get_authenticated_user(mut app: TestApp) {
    assert_json_snapshot!(app.get("/users/me").await);
}

#[test(with_app)]
async fn create_user_request_authenticated(mut app: TestApp) {
    let cookie = app.login().await;

    let body = json!({
        "username": "test2",
        "password": "password",
    });

    let res = app
        .req_json(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("Content-Type", "application/json")
                .header("Cookie", &cookie)
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await;

    assert_json_snapshot!(res);
}

#[test(with_app)]
async fn create_user_request_with_code(mut app: TestApp) {
    let cookie = app.login().await;

    let res = app
        .req_json(
            Request::builder()
                .method("POST")
                .uri("/users/registrations")
                .header("Content-Type", "application/json")
                .header("Cookie", cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await;

    let code = res
        .as_object()
        .unwrap()
        .get("code")
        .unwrap()
        .as_str()
        .unwrap();

    let body = json!({
        "username": "test2",
        "password": "password",
        "registration_code": code,
    });

    let res = app
        .req_json(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await;

    assert_json_snapshot!(res);
}

#[test(with_app)]
async fn create_user_request_with_invalid_registration_code_is_rejected(mut app: TestApp) {
    let body = json!({
        "username": "test2",
        "password": "password",
    });

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let body = json!({
        "username": "test2",
        "password": "password",
        "registration_code": "123",
    });

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
