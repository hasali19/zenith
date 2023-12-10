mod auth;
mod media;
mod metadata;
mod users;

use std::sync::Arc;

use axum::body::Body;
use axum::http::{HeaderValue, Request};
use axum::Extension;
use axum_extra::extract::cookie::Key;
use bytes::Buf;
use camino::{Utf8Path, Utf8PathBuf};
use futures::future::LocalBoxFuture;
use futures::Future;
use http_body_util::BodyExt;
use hyper::StatusCode;
use libtest_mimic::{Arguments, Failed, Trial};
use serde_json::{json, Value};
use sqlx::SqliteConnection;
use tempfile::TempDir;
use tmdb::TmdbClient;
use tokio::task::LocalSet;
use tower::ServiceExt;
use tower_http::trace::TraceLayer;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};
use uuid::Uuid;
use wiremock::MockServer;
use zenith::config::{self, Config};
use zenith::library::MediaLibrary;
use zenith::video_prober::MockVideoProber;
use zenith::{App, Db, MediaItemType};

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
    // hash of "password"
    const PASSWORD_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$cV946Lj8LNOX2F7ClooV3A$bZQHhEei6/LLmfpyuX2Hqupj416sfZ8/LtxmUg0FZqI";

    // Create a user
    db::users::create(
        &mut *conn,
        db::users::NewUser {
            username: "test",
            password_hash: PASSWORD_HASH,
        },
    )
    .await?;

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
            INSERT INTO media_item_user_data (item_id, user_id, position, position_updated_at)
            VALUES (?, ?, ?, 1662911415)";

        sqlx::query(sql)
            .bind(id)
            .bind(1)
            .bind(50.0)
            .execute(&mut *conn)
            .await?;
    }

    Ok(())
}

struct TestApp {
    _config: Arc<Config>,
    db: Db,
    router: axum::Router,
    mock_server: MockServer,
}

fn json_body(v: &impl serde::ser::Serialize) -> Body {
    Body::from(serde_json::to_vec(v).unwrap())
}

impl TestApp {
    fn router(&mut self) -> &mut axum::Router {
        &mut self.router
    }

    async fn get(&mut self, path: &str) -> Value {
        let cookie = self.login().await;
        self.req_json(
            Request::builder()
                .method("GET")
                .uri(path)
                .header("Cookie", &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
    }

    async fn req_json(&mut self, req: Request<Body>) -> Value {
        let res = self.router().oneshot(req).await.unwrap();

        if !res.status().is_success() {
            panic!("request failed with status {}", res.status());
        }

        let body = res.into_body().collect().await.unwrap();
        serde_json::from_reader(body.aggregate().reader()).unwrap()
    }

    async fn login(&mut self) -> HeaderValue {
        self.login_with("test", "password").await
    }

    async fn login_with(&mut self, username: &str, password: &str) -> HeaderValue {
        let body = json!({
            "username": username,
            "password": password,
        });

        let mut res = self
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

        res.headers_mut().remove("Set-Cookie").unwrap()
    }
}

async fn with_app<F>(f: fn(TestApp) -> F) -> Result<(), ()>
where
    F: Future<Output = ()> + 'static,
{
    let id = Uuid::new_v4();

    let subtitles_dir = TempDir::new().unwrap();

    let config = Arc::new(Config {
        logging: config::Logging::default(),
        http: config::Http::default(),
        libraries: config::Libraries {
            movies: Utf8PathBuf::from(""),
            tv_shows: Utf8PathBuf::from(""),
        },
        paths: config::Paths {
            cache: Utf8PathBuf::from(""),
            data: Utf8PathBuf::from(""),
        },
        tmdb: config::Tmdb {
            api_key: "".to_owned(),
        },
        transcoding: config::Transcoding::default(),
        database: config::Database::default(),
        import: config::Import::default(),
        subtitles: config::Subtitles {
            path: Utf8Path::from_path(subtitles_dir.path())
                .unwrap()
                .to_owned(),
        },
        watcher: config::Watcher::default(),
    });

    let db = Db::init(&format!("file:zenith_{id}?mode=memory&cache=shared"))
        .await
        .unwrap();
    tracing::debug!("opened db {id}");

    init_test_data(&mut db.acquire().await.unwrap())
        .await
        .unwrap();

    let app = App {
        key: Key::generate(),
    };

    let mock_server = MockServer::start().await;
    let tmdb_client = TmdbClient::new(mock_server.uri(), "");

    let media_library = Arc::new(MediaLibrary::new(
        db.clone(),
        vec![],
        Arc::new(MockVideoProber::new()),
    ));

    let router = zenith::api::router(app)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(config.clone()))
        .layer(Extension(db.clone()))
        .layer(Extension(media_library))
        .layer(Extension(tmdb_client));

    let app = TestApp {
        _config: config,
        db: db.clone(),
        router,
        mock_server,
    };

    let res = tokio::task::spawn_local(f(app)).await;

    db.close().await;
    tracing::debug!("closed db {id}");

    res.map_err(|_| {})
}

struct TestCase(
    &'static str,
    fn() -> LocalBoxFuture<'static, Result<(), ()>>,
);

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
                    .block_on(LocalSet::new().run_until(f()))
                    .map_err(|_| Failed::without_message())
            })
        })
        .collect();

    libtest_mimic::run(&args, tests).exit();
}
