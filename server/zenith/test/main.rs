mod media;

use std::sync::Arc;

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
use zenith::config::Config;
use zenith::db::media::MediaItemType;
use zenith::db::Db;

async fn init_test_data(conn: &mut SqliteConnection) -> eyre::Result<()> {
    // Create some movies
    for i in 1..=3 {
        sqlx::query("INSERT INTO media_items (id, item_type) VALUES (?, ?)")
            .bind(i)
            .bind(MediaItemType::Movie)
            .execute(&mut *conn)
            .await?;

        sqlx::query("INSERT INTO movies (item_id, title) VALUES (?, ?)")
            .bind(i)
            .bind(format!("Test Movie {i}"))
            .execute(&mut *conn)
            .await?;

        sqlx::query("INSERT INTO video_files (item_id, path, duration) VALUES (?, ?, ?)")
            .bind(i)
            .bind(format!("/path/to/Test Movie {i}/Test Movie {i}.mp4"))
            .bind(0.0)
            .execute(&mut *conn)
            .await?;
    }

    // Create some shows
    for i in 1..=3 {
        let id = i + 3;

        sqlx::query("INSERT INTO media_items (id, item_type) VALUES (?, ?)")
            .bind(id)
            .bind(MediaItemType::Show)
            .execute(&mut *conn)
            .await?;

        sqlx::query("INSERT INTO tv_shows (item_id, path, name) VALUES (?, ?, ?)")
            .bind(id)
            .bind(format!("/path/to/Test Show {i}"))
            .bind(format!("Test Show {i}"))
            .execute(&mut *conn)
            .await?;
    }

    // Create some seasons for Show 1
    for i in 1..=2 {
        let id = i + 6;

        sqlx::query("INSERT INTO media_items (id, item_type) VALUES (?, ?)")
            .bind(id)
            .bind(MediaItemType::Season)
            .execute(&mut *conn)
            .await?;

        sqlx::query(
            "INSERT INTO tv_seasons (item_id, show_id, season_number, name) VALUES (?, ?, ?, ?)",
        )
        .bind(id)
        .bind(4)
        .bind(i)
        .bind(format!("Season {i}"))
        .execute(&mut *conn)
        .await?;
    }

    // Create some episodes for Season 1
    for i in 1..=2 {
        let id = i + 8;

        sqlx::query("INSERT INTO media_items (id, item_type) VALUES (?, ?)")
            .bind(id)
            .bind(MediaItemType::Episode)
            .execute(&mut *conn)
            .await?;

        sqlx::query("INSERT INTO tv_episodes (item_id, season_id, episode_number, name) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(7)
            .bind(i)
            .bind(format!("Episode {i}"))
            .execute(&mut *conn)
            .await?;

        sqlx::query("INSERT INTO video_files (item_id, path, duration) VALUES (?, ?, ?)")
            .bind(id)
            .bind(format!("/path/to/Test Episode {i}"))
            .bind(0.0)
            .execute(&mut *conn)
            .await?;
    }

    // Create some episodes for Season 2
    for i in 1..=2 {
        let id = i + 10;

        sqlx::query("INSERT INTO media_items (id, item_type) VALUES (?, ?)")
            .bind(id)
            .bind(MediaItemType::Episode)
            .execute(&mut *conn)
            .await?;

        sqlx::query("INSERT INTO tv_episodes (item_id, season_id, episode_number, name) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(8)
            .bind(i)
            .bind(format!("Episode {i}"))
            .execute(&mut *conn)
            .await?;

        sqlx::query("INSERT INTO video_files (item_id, path, duration) VALUES (?, ?, ?)")
            .bind(id)
            .bind(format!("/path/to/Test Show 1/S02E{i:02}.mp4"))
            .bind(0.0)
            .execute(&mut *conn)
            .await?;
    }

    Ok(())
}

struct TestApp {
    router: axum::Router,
}

pub async fn init_test_app(db: &Db) -> axum::Router {
    let config = Arc::new(Config::load("../../config.yml").unwrap());

    init_test_data(&mut db.acquire().await.unwrap())
        .await
        .unwrap();

    zenith::api::router()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(config))
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
    let db = Db::init(&format!("file:zenith_{}?mode=memory&cache=shared", id))
        .await
        .unwrap();
    tracing::debug!("opened db {id}");

    let app = TestApp {
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
