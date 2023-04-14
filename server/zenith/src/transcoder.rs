use std::collections::VecDeque;
use std::process::Stdio;
use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use eyre::{eyre, Context};
use serde::Serialize;
use speq::Reflect;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::{broadcast, RwLock, Semaphore};

use crate::config::Config;
use crate::db::subtitles::{Subtitle, UpdateSubtitle};
use crate::db::videos::UpdateVideo;
use crate::db::{self, Db};
use crate::ext::CommandExt;
use crate::library::MediaLibrary;
use crate::video_prober::{VideoInfo, VideoProber};

#[derive(Clone, Serialize, Reflect)]
pub struct Job {
    pub video_id: i64,
    #[serde(flatten)]
    pub state: JobState,
}

#[derive(Clone, Serialize, Reflect)]
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
    library: Arc<MediaLibrary>,
    video_prober: Arc<dyn VideoProber>,
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
    pub fn new(
        db: Db,
        config: Arc<Config>,
        library: Arc<MediaLibrary>,
        video_prober: Arc<dyn VideoProber>,
    ) -> Arc<Transcoder> {
        let (sender, _) = broadcast::channel(8);

        Arc::new(Transcoder {
            db,
            config,
            library,
            video_prober,
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
            db::videos::get_all_ids(&mut conn).await.unwrap()
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
        if !Utf8Path::new("data/reports").is_dir() {
            tokio::fs::create_dir_all("data/reports")
                .await
                .expect("failed to create report directory");
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
                Err(e) => {
                    tracing::error!("{e:?}");
                    self.set_current(None).await;
                    self.sender.send(Event::Error(id)).ok();
                }
            };
        }
    }

    #[tracing::instrument(skip(self, job), fields(video_id = job.video_id))]
    async fn process_job(&self, job: Job) -> eyre::Result<()> {
        let id = job.video_id;

        tracing::info!("processing new job");

        let path = self
            .get_video_path(id)
            .await
            .wrap_err("failed to get video path")?
            .ok_or_else(|| eyre!("no video found with id: {id}"))?;

        let info = self
            .video_prober
            .probe(&path)
            .await
            .wrap_err("failed to probe video info")?;

        self.process_video(&job, &path, &info).await?;
        self.library.rescan_video(&path).await?;

        Ok(())
    }

    async fn process_video(
        &self,
        job: &Job,
        path: &Utf8Path,
        info: &VideoInfo,
    ) -> eyre::Result<()> {
        let id = job.video_id;
        let output = Utf8Path::new(path).with_extension("mkv.temp");

        enum StreamMapping {
            Copy(u32),
            ConvertAudio(u32),
        }

        // Build list of mappings for each stream in the file
        let mut transcode_any = false;
        let mut mappings = vec![];
        for stream in &info.streams {
            if stream.codec_type == "audio" {
                // Transcode audio stream if not already aac
                if stream.codec_name.as_deref() == Some("aac") {
                    mappings.push(StreamMapping::Copy(stream.index));
                } else {
                    transcode_any = true;
                    mappings.push(StreamMapping::ConvertAudio(stream.index));
                }
            } else {
                // Copy all other streams
                mappings.push(StreamMapping::Copy(stream.index));
            }
        }

        let mut cmd = Command::new(&self.config.transcoding.ffmpeg_path);

        cmd.arg_pair("-i", path);

        // Generate ffmpeg args for all the mappings, but only if we're actually
        // transcoding at least one stream
        if transcode_any {
            for mapping in mappings {
                match mapping {
                    StreamMapping::Copy(index) => {
                        cmd.arg_pair("-map", format!("0:{index}"));
                        cmd.arg_pair(format!("-c:{index}"), "copy");
                    }
                    StreamMapping::ConvertAudio(index) => {
                        tracing::info!(stream = index, "converting audio stream");
                        cmd.arg_pair("-map", format!("0:{index}"));
                        cmd.arg_pair(format!("-c:{index}"), "aac");
                        cmd.arg_pair(format!("-ac:{index}"), "2");
                    }
                }
            }

            cmd.arg_pair("-f", "matroska");
            cmd.arg(&output);
        }

        // Extract subtitles
        let subtitles_dir = self.config.subtitles.path.join(id.to_string());
        let existing_subtitles = self.get_video_subtitles(id).await?;
        let mut subtitle_tmps = vec![];
        for stream in &info.streams {
            if stream.codec_type.as_str() == "subtitle" {
                if existing_subtitles.iter().any(|s| {
                    matches!(s.stream_index, Some(i) if i == stream.index) && s.path.is_some()
                }) {
                    // Skip if the subtitle has already been extracted
                    // TODO: Option to re-extract subtitles
                    continue;
                }

                let index = stream.index;
                let output = subtitles_dir.join(format!("{index}.extracted.vtt.tmp"));

                cmd.arg_pair("-map", format!("0:{index}"));
                cmd.arg_pair(format!("-c:{index}"), "copy");
                cmd.arg_pair("-f", "webvtt");
                cmd.arg(&output);

                subtitle_tmps.push((output, stream.index));
            }
        }

        // Finish if no streams need transcoding and no subtitles
        // need extracting
        if !transcode_any && subtitle_tmps.is_empty() {
            tracing::info!("skipping - nothing to do");
            return Ok(());
        }

        // Ensure the subtitle directory exists
        if !subtitle_tmps.is_empty() {
            tracing::info!("extracting {} subtitle streams", subtitle_tmps.len());
            tokio::fs::create_dir_all(subtitles_dir)
                .await
                .wrap_err("failed to create subtitles directory")?;
        }

        cmd.arg_pair("-progress", "-");
        cmd.arg("-y");
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::null());

        cmd.env("FFREPORT", "file=data/reports/%p-%t.log:level=32");

        let mut child = cmd
            .spawn()
            .wrap_err_with(|| eyre!("failed to spawn command: {:?}", cmd.as_std()))?;

        // Monitor progress from the ffmpeg process
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

        if !child.wait().await?.success() {
            return Err(eyre!("ffmpeg terminated unsuccessfully"));
        }

        if transcode_any {
            tokio::fs::remove_file(path)
                .await
                .wrap_err("failed to remove original video file")?;

            self.rename_tmp_file(&output).await?;
            self.update_video_path(id, &output.with_extension(""))
                .await?;
        }

        for (path, stream_index) in subtitle_tmps {
            self.rename_tmp_file(&path).await?;
            self.update_subtitle_path(id, stream_index, &path.with_extension(""))
                .await?;
        }

        tracing::info!("finished processing job");

        Ok(())
    }

    async fn rename_tmp_file(&self, path: &Utf8Path) -> eyre::Result<()> {
        tokio::fs::rename(path, path.with_extension(""))
            .await
            .wrap_err("failed to rename new video file")
    }

    async fn update_job_progress(&self, job: &Job, progress: f64) {
        self.set_current(Some(job.progress(progress))).await;
        self.sender
            .send(Event::Progress(job.video_id, progress))
            .ok();
    }

    async fn get_video_path(&self, id: i64) -> eyre::Result<Option<Utf8PathBuf>> {
        let mut conn = self.db.acquire().await?;

        let path = db::videos::get_basic_info(&mut conn, id)
            .await?
            .map(|info| info.path);

        Ok(path)
    }

    async fn get_video_subtitles(&self, id: i64) -> eyre::Result<Vec<Subtitle>> {
        let mut conn = self.db.acquire().await?;
        let subtitles = db::subtitles::get_for_video(&mut conn, id).await?;
        Ok(subtitles)
    }

    async fn update_subtitle_path(
        &self,
        video_id: i64,
        stream_index: u32,
        path: &Utf8Path,
    ) -> eyre::Result<()> {
        let mut conn = self.db.acquire().await?;

        let data = UpdateSubtitle {
            path: Some(path),
            title: None,
            language: None,
        };

        db::subtitles::update_embedded(&mut conn, video_id, stream_index, data).await?;

        Ok(())
    }

    async fn update_video_path(&self, id: i64, path: &Utf8Path) -> eyre::Result<()> {
        let mut conn = self.db.acquire().await?;

        let data = UpdateVideo {
            path: Some(path),
            duration: None,
            format_name: None,
        };

        db::videos::update(&mut conn, id, data).await?;

        Ok(())
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
