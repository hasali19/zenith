use std::collections::VecDeque;
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;

use eyre::{eyre, Context};
use serde::Serialize;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::{broadcast, RwLock, Semaphore};

use crate::config::Config;
use crate::db::Db;
use crate::ext::CommandExt;
use crate::ffprobe::{Ffprobe, VideoInfo};

#[derive(Clone, Serialize)]
pub struct Job {
    pub video_id: i64,
    #[serde(flatten)]
    pub state: JobState,
}

#[derive(Clone, Serialize)]
#[serde(tag = "state")]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    Queued,
    Processing { progress: f64 },
}

impl Job {
    pub fn new(video_id: i64) -> Job {
        Job {
            video_id,
            state: JobState::Queued,
        }
    }

    fn progress(&self, progress: f64) -> Job {
        Job {
            video_id: self.video_id,
            state: JobState::Processing { progress },
        }
    }
}

pub struct Transcoder {
    db: Db,
    config: Arc<Config>,
    sema: Semaphore,
    queue: RwLock<VecDeque<Job>>,
    current: RwLock<Option<Job>>,
    sender: broadcast::Sender<Event>,
}

#[derive(Clone, Debug)]
pub enum Event {
    Queued(i64),
    Started(i64),
    Progress(i64, f64),
    Success(i64),
    Error(i64),
}

impl Transcoder {
    pub fn new(db: Db, config: Arc<Config>) -> Arc<Transcoder> {
        let (sender, _) = broadcast::channel(8);

        Arc::new(Transcoder {
            db,
            config,
            sema: Semaphore::new(0),
            queue: RwLock::new(VecDeque::new()),
            current: RwLock::new(None),
            sender,
        })
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    pub async fn enqueue(&self, job: Job) {
        let id = job.video_id;

        // Skip if we are already transcoding this id
        if self.current().await.iter().any(|j| j.video_id == id) {
            return;
        }

        let mut queue = self.queue.write().await;

        // Skip if this id is already in the queue
        for existing in queue.iter() {
            if existing.video_id == id {
                return;
            }
        }

        queue.push_back(job);

        self.sema.add_permits(1);
        self.sender.send(Event::Queued(id)).ok();
    }

    pub async fn enqueue_all(&self) {
        let ids: Vec<i64> = {
            let mut conn = self.db.acquire().await.unwrap();

            sqlx::query_scalar("SELECT item_id FROM video_files")
                .fetch_all(&mut conn)
                .await
                .unwrap()
        };

        for id in ids {
            self.enqueue(Job::new(id)).await;
        }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(self.run());
    }

    /// Returns the job currently being processed, if any.
    pub async fn current(&self) -> Option<Job> {
        self.current.read().await.as_ref().cloned()
    }

    /// Returns the queue of jobs.
    ///
    /// This includes the job currently being processed as the first item in the queue.
    pub async fn queue(&self) -> Vec<Job> {
        let mut queue = vec![];

        if let Some(job) = self.current().await {
            queue.push(job);
        }

        queue.extend(self.queue.read().await.iter().cloned());

        queue
    }

    async fn set_current(&self, value: Option<Job>) {
        *self.current.write().await = value;
    }

    #[tracing::instrument(skip(self))]
    async fn run(self: Arc<Self>) {
        if !Path::new("data/reports").is_dir() {
            std::fs::create_dir_all("data/reports").expect("failed to create report directory");
        }

        loop {
            let job = self.dequeue_job().await;
            let id = job.video_id;

            self.set_current(Some(job.progress(0.0))).await;
            self.sender.send(Event::Started(id)).ok();

            match self.process_job(job).await {
                Ok(_) => {
                    self.set_current(None).await;
                    self.sender.send(Event::Success(id)).ok();
                }
                Err(_) => {
                    self.set_current(None).await;
                    self.sender.send(Event::Error(id)).ok();
                }
            };
        }
    }

    async fn process_job(&self, job: Job) -> eyre::Result<()> {
        let id = job.video_id;

        tracing::info!("starting transcode for video (id: {})", id);

        let path = self
            .get_video_path(id)
            .await
            .wrap_err("failed to get video path")?
            .ok_or_else(|| eyre!("no video found with id: {}", id))?;

        let info = Ffprobe::new(&self.config.transcoding.ffprobe_path)
            .probe(&path)
            .await
            .wrap_err("ffprobe failed to get video info")?;

        self.convert_video(&job, &path, &info).await?;

        Ok(())
    }

    async fn convert_video(&self, job: &Job, path: &str, info: &VideoInfo) -> eyre::Result<()> {
        let id = job.video_id;
        let mut cmd = Command::new(&self.config.transcoding.ffmpeg_path);

        cmd.arg_pair("-i", path);

        let mut transcode_any = false;

        for stream in &info.streams {
            match stream.codec_type.as_str() {
                // Transcode audio stream if not already aac
                "audio" => {
                    cmd.arg_pair("-map", format!("0:{}", stream.index));

                    if stream.codec_name == "aac" {
                        cmd.arg_pair(format!("-c:{}", stream.index), "copy");
                    } else {
                        cmd.arg_pair(format!("-c:{}", stream.index), "aac");
                        cmd.arg_pair(format!("-ac:{}", stream.index), "2");
                        transcode_any = true;
                    }
                }
                // Copy all other streams
                _ => {
                    cmd.arg_pair("-map", format!("0:{}", stream.index));
                    cmd.arg_pair(format!("-c:{}", stream.index), "copy");
                }
            }
        }

        if !transcode_any {
            tracing::info!("skipping {} - no streams to transcode", id);
            return Ok(());
        }

        cmd.arg_pair("-f", "matroska");

        let output = Path::new(path).with_extension("mkv.temp");

        cmd.arg(&output);

        cmd.arg_pair("-progress", "-");
        cmd.arg("-y");
        cmd.stdout(Stdio::piped());

        cmd.env("FFREPORT", "file=data/reports/%p-%t.log:level=32");

        let mut child = cmd.spawn().wrap_err("failed to spawn ffmpeg")?;

        let duration = info.format.duration.parse::<f64>().unwrap() * 1000f64;
        let stderr = child.stdout.take().unwrap();
        let mut reader = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            if let Some((key, value)) = line.split_once('=') {
                match key {
                    "progress" if value == "end" => break,
                    "out_time_us" => {
                        if let Ok(time) = value.parse::<u64>() {
                            let progress = (time as f64 / 1000f64) / duration;
                            self.update_job_progress(job, progress).await;
                        }
                    }
                    _ => {}
                }
            }
        }

        tracing::info!("finished reading ffmpeg progress");

        if !child.wait().await?.success() {
            return Err(eyre!("ffmpeg terminated unsuccessfully"));
        }

        std::fs::remove_file(path).wrap_err("failed to remove old video file")?;

        let path = output.with_extension("");

        std::fs::rename(&output, &path).wrap_err("failed to rename new video file")?;

        self.update_video_path(id, &path).await;

        Ok(())
    }

    async fn update_job_progress(&self, job: &Job, progress: f64) {
        self.set_current(Some(job.progress(progress))).await;
        self.sender
            .send(Event::Progress(job.video_id, progress))
            .ok();
    }

    async fn get_video_path(&self, id: i64) -> eyre::Result<Option<String>> {
        let mut conn = self.db.acquire().await?;

        let path = sqlx::query_scalar("SELECT path FROM video_files WHERE item_id = ?")
            .bind(id)
            .fetch_optional(&mut conn)
            .await?;

        Ok(path)
    }

    async fn update_video_path(&self, id: i64, path: &Path) {
        let mut conn = self.db.acquire().await.unwrap();

        sqlx::query("UPDATE video_files SET path = ? WHERE item_id = ?")
            .bind(path.to_str().unwrap())
            .bind(id)
            .execute(&mut conn)
            .await
            .unwrap();
    }

    async fn dequeue_job(&self) -> Job {
        // Wait for the semaphore to signal that there is an item
        // in the queue
        self.sema.acquire().await.unwrap().forget();

        let mut queue = self.queue.write().await;

        // At this point it should always be safe to pop
        queue.pop_front().unwrap()
    }
}
