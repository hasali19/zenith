use std::path::Path;
use std::sync::Arc;

use tokio::process::Command;
use tokio::sync::{RwLock, Semaphore};

use crate::db::Db;
use crate::ext::CommandExt;
use crate::ffprobe::Ffprobe;

pub struct Job {
    pub video_id: i64,
}

pub struct Transcoder {
    db: Db,
    sema: Semaphore,
    queue: RwLock<Vec<Job>>,
    current: RwLock<Option<Job>>,
}

impl Transcoder {
    pub fn new(db: Db) -> Arc<Transcoder> {
        Arc::new(Transcoder {
            db,
            sema: Semaphore::new(0),
            queue: RwLock::new(Vec::new()),
            current: RwLock::new(None),
        })
    }

    pub async fn enqueue(&self, job: Job) {
        // Skip if we are already transcoding this id
        if self
            .current()
            .await
            .iter()
            .any(|j| j.video_id == job.video_id)
        {
            return;
        }

        let mut queue = self.queue.write().await;

        // Skip if this id is already in the queue
        for existing in queue.iter() {
            if existing.video_id == job.video_id {
                return;
            }
        }

        queue.push(job);

        self.sema.add_permits(1);
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(self.run());
    }

    pub async fn current(&self) -> Option<Job> {
        self.current.read().await.as_ref().map(|j| Job {
            video_id: j.video_id,
        })
    }

    async fn set_current(&self, value: Option<Job>) {
        *self.current.write().await = value;
    }

    #[tracing::instrument(skip(self))]
    async fn run(self: Arc<Self>) {
        loop {
            let job = self.dequeue_job().await;
            let id = job.video_id;

            self.set_current(Some(job)).await;

            tracing::info!("starting transcode for video (id: {})", id);

            let path = self.get_video_path(id).await;
            let info = Ffprobe::new("ffprobe").probe(&path).await.unwrap();

            let mut cmd = Command::new("ffmpeg");

            cmd.arg_pair("-i", &path);

            for stream in info.streams {
                // Transcode video stream if not already in h264
                if stream.codec_type == "video" {
                    cmd.arg_pair("-map", format!("0:{}", stream.index));

                    if stream.codec_name == "h264" {
                        cmd.arg_pair(format!("-c:{}", stream.index), "copy");
                    } else {
                        cmd.arg_pair(format!("-c:{}", stream.index), "libx264");
                    }
                }

                // Transcode audio stream if not already in aac
                if stream.codec_type == "audio" {
                    cmd.arg_pair("-map", format!("0:{}", stream.index));

                    if stream.codec_name == "aac" {
                        cmd.arg_pair(format!("-c:{}", stream.index), "copy");
                    } else {
                        cmd.arg_pair(format!("-c:{}", stream.index), "aac");
                        cmd.arg_pair(format!("-ac:{}", stream.index), "2");
                    }
                }

                // Copy all subtitle streams
                if stream.codec_type == "subtitle" {
                    cmd.arg_pair("-map", format!("0:{}", stream.index));
                    cmd.arg_pair(format!("-c:{}", stream.index), "copy");
                }
            }

            cmd.arg_pair("-f", "matroska");
            cmd.arg("-y");

            let output = Path::new(&path).with_extension("mkv.temp");

            cmd.arg(&output);

            match cmd.status().await {
                Ok(status) => {
                    if !status.success() {
                        tracing::error!("ffmpeg terminated unsuccessfully");
                        self.set_current(None).await;
                        continue;
                    }
                }
                Err(e) => {
                    tracing::error!("{}", e);
                    self.set_current(None).await;
                    continue;
                }
            }

            if let Err(e) = std::fs::remove_file(&path) {
                tracing::error!("failed to remove file: {}", e);
                self.set_current(None).await;
                continue;
            }

            let path = output.with_extension("");

            if let Err(e) = std::fs::rename(&output, &path) {
                tracing::error!("failed to rename file: {}", e);
                self.set_current(None).await;
                continue;
            }

            self.update_video_path(id, &path).await;
            self.set_current(None).await;
        }
    }

    async fn get_video_path(&self, id: i64) -> String {
        let mut conn = self.db.acquire().await.unwrap();

        sqlx::query_scalar("SELECT path FROM video_files WHERE item_id = ?")
            .bind(id)
            .fetch_one(&mut conn)
            .await
            .unwrap()
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
        queue.pop().unwrap()
    }
}
