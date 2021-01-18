pub mod movies;
pub mod tv_shows;

use std::sync::Arc;

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{self, Ready};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::config::Config;
use crate::db::Db;
use crate::ffmpeg::Ffprobe;
use crate::tmdb::TmdbClient;

#[derive(Clone)]
pub struct SyncService(Sender<Request>);

impl FromRequest for SyncService {
    type Error = ();
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        future::ok(req.app_data::<Self>().unwrap().clone())
    }
}

#[derive(Debug)]
enum Request {
    StartFullSync,
}

impl SyncService {
    pub fn new(db: Db, tmdb: TmdbClient, config: Arc<Config>) -> Self {
        let (tx, rx) = mpsc::channel(1);
        tokio::spawn(sync_service(rx, db, tmdb, config));
        SyncService(tx)
    }

    pub async fn start_full_sync(&mut self) {
        self.0.send(Request::StartFullSync).await.unwrap();
    }
}

async fn sync_service(mut rx: Receiver<Request>, db: Db, tmdb: TmdbClient, config: Arc<Config>) {
    full_sync(&db, &tmdb, &config).await.unwrap();

    while let Some(req) = rx.recv().await {
        match req {
            Request::StartFullSync => full_sync(&db, &tmdb, &config).await.unwrap(),
        }
    }
}

async fn full_sync(db: &Db, tmdb: &TmdbClient, config: &Config) -> eyre::Result<()> {
    log::info!("running full library sync");

    let ffprobe = Ffprobe::new(config.ffprobe_path());
    let mut conn = db.acquire().await?;

    movies::sync_movies(&mut conn, &tmdb, &ffprobe, &config.movie_path).await?;
    tv_shows::sync_tv_shows(&mut conn, &tmdb, &ffprobe, &config.tv_show_path).await?;

    Ok(())
}
