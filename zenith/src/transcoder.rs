use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{self, Ready};
use tokio::process::{Child, Command};
use tokio::sync::{mpsc, oneshot};

use crate::db::Db;

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
    pub fn new(db: Db, temp_dir: &str) -> Self {
        let (tx, rx) = mpsc::channel(1);
        tokio::spawn(transcoder(rx, db, temp_dir.to_string()));
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

async fn transcoder(mut rx: mpsc::Receiver<Request>, db: Db, temp_dir: String) {
    let temp_dir = PathBuf::from(temp_dir);

    // Delete anything in the temp dir from a previous run
    std::fs::read_dir(&temp_dir)
        .unwrap()
        .for_each(|e| std::fs::remove_file(e.unwrap().path()).unwrap());

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

                let (path,): (String,) =
                    sqlx::query_as("SELECT path FROM video_files WHERE id = ?")
                        .bind(video_id)
                        .fetch_optional(&mut conn)
                        .await
                        .unwrap()
                        .unwrap();

                let job = match current_job.take() {
                    Some(mut job) => {
                        let should_restart = job.video_id != video_id
                            || segment < job.start_segment
                            || job
                                .last_segment()
                                .map(|n| segment > n + 15)
                                .unwrap_or(false);

                        if should_restart {
                            log::warn!("killing current transcode job");

                            job.process.kill().unwrap();
                            job.process.await.unwrap();

                            std::fs::read_dir(&temp_dir)
                                .unwrap()
                                .for_each(|e| std::fs::remove_file(e.unwrap().path()).unwrap());

                            Job {
                                video_id,
                                out_dir: temp_dir.clone(),
                                start_segment: segment,
                                process: start_job(&temp_dir, &path, segment),
                            }
                        } else {
                            job
                        }
                    }
                    None => Job {
                        video_id,
                        out_dir: temp_dir.clone(),
                        start_segment: segment,
                        process: start_job(&temp_dir, &path, segment),
                    },
                };

                let segment_path = temp_dir.join(format!("{}.ts", segment));

                // Wait until the file is ready
                while !segment_path.is_file() {
                    tokio::time::delay_for(Duration::from_millis(500)).await;
                }

                tx.send(segment_path).unwrap();

                current_job = Some(job);
            }

            Request::Cancel { video_id, tx } => {
                if let Some(job) = &current_job {
                    if job.video_id == video_id {
                        log::warn!("killing current transcode job");

                        let mut job = current_job.take().unwrap();

                        job.process.kill().unwrap();
                        job.process.await.unwrap();

                        std::fs::read_dir(&temp_dir)
                            .unwrap()
                            .for_each(|e| std::fs::remove_file(e.unwrap().path()).unwrap());
                    }
                }

                tx.send(()).unwrap();
            }
        }
    }
}

fn get_segment_number_from_name(name: &str) -> Option<u32> {
    name.split('.').next().and_then(|n| n.parse().ok())
}

fn start_job(temp_dir: &Path, path: &str, segment: u32) -> Child {
    let segment_name_template = temp_dir.join("%d.ts");
    let playlist_name = temp_dir.join("main.m3u8");
    let process = Command::new("ffmpeg")
        .arg("-ss")
        .arg((segment * 3).to_string())
        .arg("-noaccurate_seek")
        .arg("-i")
        .arg(path)
        .arg("-map_metadata")
        .arg("-1")
        .arg("-map_chapters")
        .arg("-1")
        .arg("-threads")
        .arg("0")
        .arg("-codec:v")
        .arg("libx264")
        .arg("-force_key_frames:0")
        .arg(format!("expr:gte(t,{}+n_forced*3)", segment * 3))
        .arg("-g")
        .arg("72")
        .arg("-keyint_min")
        .arg("72")
        .arg("-sc_threshold")
        .arg("0")
        .arg("-start_at_zero")
        .arg("-vsync")
        .arg("-1")
        .arg("-codec:a")
        .arg("libmp3lame")
        .arg("-copyts")
        .arg("-avoid_negative_ts")
        .arg("disabled")
        .arg("-f")
        .arg("hls")
        .arg("-max_delay")
        .arg("5000000")
        .arg("-hls_time")
        .arg("3")
        .arg("-individual_header_trailer")
        .arg("0")
        .arg("-hls_segment_type")
        .arg("mpegts")
        .arg("-start_number")
        .arg(segment.to_string())
        .arg("-hls_segment_filename")
        .arg(segment_name_template)
        .arg("-hls_playlist_type")
        .arg("vod")
        .arg("-hls_list_size")
        .arg("0")
        .arg("-hls_flags")
        .arg("temp_file")
        .arg("-y")
        .arg(playlist_name)
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    process
}
