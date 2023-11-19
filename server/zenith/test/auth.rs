use hyper::{Body, Request, StatusCode};
use serde_json::json;
use test_macros::test;
use tower::ServiceExt;

use crate::{with_app, TestApp};

#[test(with_app)]
async fn request_with_invalid_token_is_rejected(mut app: TestApp) {
    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users/me")
                .header("Cookie", "auth=garbage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[test(with_app)]
async fn request_with_unknown_user_id_is_rejected(mut app: TestApp) {
    // hash of "password"
    const PASSWORD_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$cV946Lj8LNOX2F7ClooV3A$bZQHhEei6/LLmfpyuX2Hqupj416sfZ8/LtxmUg0FZqI";

    let mut conn = app.db.acquire().await.unwrap();

    db::users::create(
        &mut conn,
        db::users::NewUser {
            username: "test2",
            password_hash: PASSWORD_HASH,
        },
    )
    .await
    .unwrap();

    let cookie = app.login_with("test2", "password").await;

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users/me")
                .header("Cookie", &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    sqlx::query("DELETE FROM users WHERE username = 'test2'")
        .execute(&mut *conn)
        .await
        .unwrap();

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users/me")
                .header("Cookie", &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[test(with_app)]
async fn successful_login_returns_auth_cookie(mut app: TestApp) {
    let body = json!({
        "username": "test",
        "password": "password",
    });

    let mut res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("Set-Cookie"));

    let cookie = res.headers_mut().remove("Set-Cookie").unwrap();

    tracing::info!("COOKIE: {}", cookie.to_str().unwrap());

    assert!(cookie.to_str().unwrap().starts_with("auth="));
}

#[test(with_app)]
async fn login_with_bad_credentials_is_rejected(mut app: TestApp) {
    let body = json!({
        "username": "test2",
        "password": "password",
    });

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let body = json!({
        "username": "test",
        "password": "password2",
    });

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[test(with_app)]
async fn logout_removes_auth_cookie(mut app: TestApp) {
    let cookie = app.login().await;

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/logout")
                .header("Cookie", &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(res
        .headers()
        .get("Set-Cookie")
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with("auth=;"));
}
