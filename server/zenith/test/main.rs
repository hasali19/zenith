mod media;

use axum::body::Body;
use axum::http::Request;
use axum::Extension;
use futures::future::LocalBoxFuture;
use futures::Future;
use libtest_mimic::{Arguments, Trial};
use serde_json::Value;
use sqlx::SqliteConnection;
use tokio::task::LocalSet;
use tower::ServiceExt;
use tower_http::trace::TraceLayer;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};
use uuid::Uuid;
use zenith::{Db, MediaItemType};

async fn insert_video_file(
    conn: &mut SqliteConnection,
    item_id: i64,
    path: &str,
) -> eyre::Result<()> {
    let id = sqlx::query("INSERT INTO video_files (item_id, path, duration) VALUES (?, ?, ?)")
        .bind(item_id)
        .bind(path)
        .bind(100.0)
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

    db::streams::insert_video_stream(
        &mut *conn,
        &db::streams::NewVideoStream {
            video_id: id,
            index: 0,
            codec_name: "h264",
            width: 1920,
            height: 1080,
        },
    )
    .await?;

    db::streams::insert_audio_stream(
        &mut *conn,
        &db::streams::NewAudioStream {
            video_id: id,
            index: 1,
            codec_name: "aac",
            language: Some("eng"),
            channels: Some(2),
            channel_layout: Some("stereo"),
        },
    )
    .await?;

    Ok(())
}

async fn init_test_data(conn: &mut SqliteConnection) -> eyre::Result<()> {
    // Create some movies
    for i in 1..=3 {
        sqlx::query("INSERT INTO media_items (id, item_type, name) VALUES (?, ?, ?)")
            .bind(i)
            .bind(MediaItemType::Movie)
            .bind(format!("Test Movie {i}"))
            .execute(&mut *conn)
            .await?;

        insert_video_file(
            &mut *conn,
            i,
            &format!("/path/to/Test Movie {i}/Test Movie {i}.mp4"),
        )
        .await?;
    }

    // Create some shows
    for i in 1..=3 {
        let id = i + 3;

        sqlx::query("INSERT INTO media_items (id, item_type, name) VALUES (?, ?, ?)")
            .bind(id)
            .bind(MediaItemType::Show)
            .bind(format!("Test Show {i}"))
            .execute(&mut *conn)
            .await?;
    }

    // Create some seasons for Show 1
    for i in 1..=2 {
        let id = i + 6;

        let sql = "
            INSERT INTO media_items (id, item_type, name, parent_id, parent_index)
            VALUES (?, ?, ?, ?, ?)";

        sqlx::query(sql)
            .bind(id)
            .bind(MediaItemType::Season)
            .bind(format!("Season {i}"))
            .bind(4)
            .bind(i)
            .execute(&mut *conn)
            .await?;
    }

    // Create some episodes for Season 1
    for i in 1..=2 {
        let id = i + 8;

        let sql = "
            INSERT INTO media_items (id, item_type, name, parent_id, parent_index, grandparent_id, grandparent_index)
            VALUES (?, ?, ?, ?, ?, ?, ?)";

        sqlx::query(sql)
            .bind(id)
            .bind(MediaItemType::Episode)
            .bind(format!("Episode {i}"))
            .bind(7)
            .bind(i)
            .bind(4)
            .bind(1)
            .execute(&mut *conn)
            .await?;

        insert_video_file(&mut *conn, id, &format!("/path/to/Test Episode {i}")).await?;
    }

    // Create some episodes for Season 2
    for i in 1..=2 {
        let id = i + 10;

        let sql = "
            INSERT INTO media_items (id, item_type, name, parent_id, parent_index, grandparent_id, grandparent_index)
            VALUES (?, ?, ?, ?, ?, ?, ?)";

        sqlx::query(sql)
            .bind(id)
            .bind(MediaItemType::Episode)
            .bind(format!("Episode {i}"))
            .bind(8)
            .bind(i)
            .bind(4)
            .bind(2)
            .execute(&mut *conn)
            .await?;

        insert_video_file(
            &mut *conn,
            id,
            &format!("/path/to/Test Show 1/S02E{i:02}.mp4"),
        )
        .await?;
    }

    for id in [1, 2, 10, 11] {
        let sql = "
            INSERT INTO user_item_data (item_id, position, last_watched_at)
            VALUES (?, ?, 1662911415)";

        sqlx::query(sql)
            .bind(id)
            .bind(50.0)
            .execute(&mut *conn)
            .await?;
    }

    Ok(())
}

struct TestApp {
    db: Db,
    router: axum::Router,
}

pub async fn init_test_app(db: &Db) -> axum::Router {
    init_test_data(&mut db.acquire().await.unwrap())
        .await
        .unwrap();

    zenith::api::router()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db.clone()))
}

impl TestApp {
    async fn get(self, path: &str) -> Value {
        let res = self
            .router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        if !res.status().is_success() {
            panic!("request failed with status {}", res.status());
        }

        let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
        serde_json::from_slice(&body).unwrap()
    }
}

async fn with_app<F>(f: fn(TestApp) -> F)
where
    F: Future<Output = ()> + 'static,
{
    let id = Uuid::new_v4();
    let db = Db::init(&format!("file:zenith_{id}?mode=memory&cache=shared"))
        .await
        .unwrap();
    tracing::debug!("opened db {id}");

    let app = TestApp {
        db: db.clone(),
        router: init_test_app(&db).await,
    };

    let res = tokio::task::spawn_local(f(app)).await;

    db.close().await;
    tracing::debug!("closed db {id}");

    res.unwrap();
}

struct TestCase(&'static str, fn() -> LocalBoxFuture<'static, ()>);

inventory::collect!(TestCase);

fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_filter(EnvFilter::from("debug")),
        )
        .with(ErrorLayer::default())
        .init();

    let args = Arguments::from_args();
    let tests = inventory::iter::<TestCase>
        .into_iter()
        .map(|TestCase(name, f)| {
            Trial::test(*name, move || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?
                    .block_on(LocalSet::new().run_until(f()));
                Ok(())
            })
        })
        .collect();

    libtest_mimic::run(&args, tests).exit();
}
