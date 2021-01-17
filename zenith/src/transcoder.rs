use std::path::{Path, PathBuf};
use std::time::Duration;

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, Result};
use futures::future::{self, Ready};
use sqlx::SqliteConnection;
use tokio::process::Child;
use tokio::sync::{mpsc, oneshot};

use crate::config::Config;
use crate::db::Db;
use crate::ffmpeg::{Ffmpeg, TranscodeOptions};

#[derive(Clone)]
pub struct Transcoder(mpsc::Sender<Request>);

impl FromRequest for Transcoder {
    type Error = ();
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        future::ok(req.app_data::<Self>().unwrap().clone())
    }
}

#[derive(Debug)]
enum Request {
    TranscodeSegment {
        video_id: i64,
        segment: u32,
        tx: oneshot::Sender<PathBuf>,
    },
    Cancel {
        video_id: i64,
        tx: oneshot::Sender<()>,
    },
}

impl Transcoder {
    pub fn new(db: Db, config: &Config) -> Self {
        let (tx, rx) = mpsc::channel(1);

        tokio::spawn(transcoder(
            rx,
            db,
            config.transcode_dir.to_string(),
            config.ffmpeg_path().to_string(),
        ));

        Transcoder(tx)
    }

    pub async fn transcode_segment(&mut self, video_id: i64, segment: u32) -> PathBuf {
        let (tx, rx) = oneshot::channel();
        let req = Request::TranscodeSegment {
            video_id,
            segment,
            tx,
        };

        self.0.send(req).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn cancel(&mut self, video_id: i64) {
        let (tx, rx) = oneshot::channel();
        let req = Request::Cancel { video_id, tx };

        self.0.send(req).await.unwrap();
        rx.await.unwrap()
    }
}

struct Job {
    video_id: i64,
    out_dir: PathBuf,
    start_segment: u32,
    process: Child,
}

impl Job {
    pub fn last_segment(&self) -> Option<u32> {
        std::fs::read_dir(&self.out_dir)
            .ok()?
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_str().unwrap().to_string())
            .filter(|n| n.ends_with(".ts"))
            .filter_map(|n| get_segment_number_from_name(&n))
            .max()
    }
}

async fn transcoder(
    mut rx: mpsc::Receiver<Request>,
    db: Db,
    temp_dir: String,
    ffmpeg_path: String,
) {
    let ffmpeg = Ffmpeg::new(ffmpeg_path);
    let transcode_dir = PathBuf::from(temp_dir);

    // Ensure transcode directory exists
    if !transcode_dir.is_dir() {
        std::fs::create_dir_all(&transcode_dir).unwrap();
    }

    // Delete anything in the transcode directory from a previous run
    clear_dir(&transcode_dir).unwrap();

    log::info!("listening for requests");

    let mut current_job: Option<Job> = None;

    while let Some(req) = rx.recv().await {
        match req {
            Request::TranscodeSegment {
                video_id,
                segment,
                tx,
            } => {
                log::info!("requested segment: {}:{}", video_id, segment);

                let mut conn = db.acquire().await.unwrap();
                let path = get_video_path_by_id(&mut conn, video_id).await.unwrap();

                let job = match current_job.take() {
                    None => start_job(&ffmpeg, &transcode_dir, video_id, &path, segment),
                    Some(job) => match should_restart_job(&job, video_id, segment) {
                        false => Ok(job),
                        true => {
                            kill_job(job, &transcode_dir).await.unwrap();
                            start_job(&ffmpeg, &transcode_dir, video_id, &path, segment)
                        }
                    },
                };

                current_job = Some(job.unwrap());

                let segment_path = transcode_dir.join(format!("{}.ts", segment));

                // Wait until the file is ready
                while !segment_path.is_file() {
                    tokio::time::delay_for(Duration::from_millis(500)).await;
                }

                // Return path to the requested segment
                tx.send(segment_path).unwrap();
            }

            Request::Cancel { video_id, tx } => {
                if let Some(job) = current_job.take() {
                    if job.video_id == video_id {
                        kill_job(job, &transcode_dir).await.unwrap();
                    }
                }

                tx.send(()).unwrap();
            }
        }
    }
}

fn clear_dir(dir: &Path) -> eyre::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let file_type = entry.file_type().unwrap();
        let path = entry.path();

        if file_type.is_file() {
            std::fs::remove_file(&path)?;
        } else if file_type.is_dir() {
            std::fs::remove_dir_all(&path)?;
        }
    }

    Ok(())
}

async fn get_video_path_by_id(conn: &mut SqliteConnection, id: i64) -> Option<String> {
    let res: Option<(String,)> = sqlx::query_as("SELECT path FROM video_files WHERE id = ?")
        .bind(id)
        .fetch_optional(conn)
        .await
        .ok()
        .flatten();

    res.map(|(path,)| path)
}

fn should_restart_job(job: &Job, video_id: i64, segment: u32) -> bool {
    job.video_id != video_id
        || segment < job.start_segment
        || job
            .last_segment()
            .map(|n| segment > n + 15)
            .unwrap_or(false)
}

fn start_job(
    ffmpeg: &Ffmpeg,
    transcode_dir: &Path,
    video_id: i64,
    path: &str,
    segment: u32,
) -> eyre::Result<Job> {
    let segment_name_template = transcode_dir.join("%d.ts");
    let playlist_name = transcode_dir.join("main.m3u8");

    let child = ffmpeg.spawn_transcode(&TranscodeOptions {
        input_path: path,
        start_number: segment,
        segment_time: 3,
        segment_filename: &segment_name_template,
        playlist_filename: &playlist_name,
    })?;

    Ok(Job {
        video_id,
        out_dir: transcode_dir.to_path_buf(),
        start_segment: segment,
        process: child,
    })
}

async fn kill_job(mut job: Job, transcode_dir: &Path) -> eyre::Result<()> {
    log::warn!("killing current transcode job");

    job.process.kill()?;
    job.process.await?;

    clear_dir(&transcode_dir)?;

    Ok(())
}

fn get_segment_number_from_name(name: &str) -> Option<u32> {
    name.split('.').next().and_then(|n| n.parse().ok())
}
