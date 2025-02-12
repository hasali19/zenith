use hyper::{Request, StatusCode};
use serde_json::json;
use test_macros::test;
use tower::ServiceExt;

use crate::{TestApp, json_body, with_app};

#[test(with_app)]
async fn update_progress(mut app: TestApp) {
    let cookie = app.login().await;

    let res = app
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/playback/1")
                .header("Cookie", &cookie)
                .header("Content-Type", "application/json")
                .body(json_body(&json!({
                    "action": "progress",
                    "position": 100,
                })))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let mut conn = app.db.acquire().await.unwrap();
    let position: f64 =
        sqlx::query_scalar("SELECT position FROM media_item_user_data WHERE item_id = 1")
            .fetch_one(&mut *conn)
            .await
            .unwrap();

    assert_eq!(position, 100.0);
}
