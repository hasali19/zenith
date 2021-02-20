use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use futures::StreamExt;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::{oneshot, Mutex};
use zenith_http::Body;

use crate::config::Config;
use crate::db::Db;
use crate::ffmpeg::{Ffmpeg, HlsTranscodeOptions};

pub struct HlsTranscoder {
    db: Db,
    config: Arc<Config>,
    current: Mutex<Option<JobState>>,
}

struct JobState {
    item_id: i64,
    playlist: Vec<(f64, f64)>,
    segments: HashMap<u32, Bytes>,
    last_requested_segment: u32,
    canceller: Option<oneshot::Sender<()>>,
}

impl HlsTranscoder {
    pub fn new(config: Arc<Config>, db: Db) -> Self {
        HlsTranscoder {
            db,
            config,
            current: Mutex::new(None),
        }
    }

    pub async fn create_session(&self, item_id: i64) -> eyre::Result<()> {
        let mut current = self.current.lock().await;

        if let Some(current) = current.as_ref() {
            if current.item_id == item_id {
                return Ok(());
            }
        }

        let mut conn = self.db.acquire().await?;

        let (path, duration): (String, f64) =
            sqlx::query_as("SELECT path, duration FROM video_files WHERE item_id = ?")
                .bind(item_id)
                .fetch_optional(&mut conn)
                .await?
                .ok_or_else(|| eyre::eyre!("not found"))?;

        let mut child = tokio::process::Command::new("ffprobe")
            .arg("-select_streams")
            .arg("v")
            .arg("-skip_frame")
            .arg("nokey")
            .arg("-show_entries")
            .arg("frame=pkt_pts_time")
            .arg("-print_format")
            .arg("csv")
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut playlist = vec![];

        let stdout = child.stdout.as_mut().unwrap();
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        let line = lines.next_line().await.unwrap().unwrap();

        let mut start_time: f64 = line.split(',').nth(1).unwrap().parse().unwrap();

        while let Ok(Some(v)) = lines.next_line().await {
            let frame_time: f64 = v.split(',').nth(1).unwrap().parse().unwrap();
            let duration = frame_time - start_time;
            if duration >= 6.0 {
                playlist.push((start_time, duration));
                start_time = frame_time;
            }
        }

        let final_segment = duration - start_time;
        if final_segment > 0.0 {
            playlist.push((start_time, final_segment));
        }

        if let Some(JobState {
            canceller: Some(canceller),
            ..
        }) = current.take()
        {
            canceller.send(()).ok();
        }

        *current = Some(JobState {
            item_id,
            playlist,
            segments: HashMap::new(),
            last_requested_segment: 0,
            canceller: None,
        });

        Ok(())
    }

    pub async fn generate_playlist(&self, item_id: i64) -> Option<String> {
        let job = self.current.lock().await;
        let job = job.as_ref()?;

        if job.item_id != item_id {
            return None;
        }

        let mut playlist = String::new();

        playlist.push_str("#EXTM3U\n");
        playlist.push_str("#EXT-X-PLAYLIST-TYPE:VOD\n");
        playlist.push_str("#EXT-X-VERSION:3\n");
        playlist.push_str("#EXT-X-TARGETDURATION:6\n");
        playlist.push_str("#EXT-X-MEDIA-SEQUENCE:0\n");

        for (i, (_, length)) in job.playlist.iter().enumerate() {
            playlist.push_str(&format!("#EXTINF:{:.8},\n", length));
            playlist.push_str(&format!("{}.ts\n", i));
        }

        playlist.push_str("#EXT-X-ENDLIST\n");

        Some(playlist)
    }

    pub async fn request_segment(&self, item_id: i64, segment: u32) -> eyre::Result<Option<Bytes>> {
        log::info!("requesting segment {}", segment);

        loop {
            let mut current = self.current.lock().await;

            let mut job = match current.as_mut() {
                None => return Ok(None),
                Some(job) if job.item_id != item_id => return Ok(None),
                Some(job) => {
                    if let Some(data) = job.segments.get(&segment) {
                        return Ok(Some(data.clone()));
                    }

                    if job.canceller.is_none() {
                        self.spawn_ffmpeg(job, item_id, segment).await?;
                        job
                    } else if segment < job.last_requested_segment
                        || segment > job.last_requested_segment + 20
                    {
                        log::warn!("restarting transcode due to out of range seek");
                        self.spawn_ffmpeg(job, item_id, segment).await?;
                        job
                    } else {
                        job
                    }
                }
            };

            job.last_requested_segment = segment;

            if let Some(data) = job.segments.get(&segment) {
                return Ok(Some(data.clone()));
            }

            drop(current);

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    async fn spawn_ffmpeg(
        &self,
        job: &mut JobState,
        item_id: i64,
        segment: u32,
    ) -> eyre::Result<()> {
        log::info!(
            "starting transcode for item_id: {}, segment: {}",
            item_id,
            segment
        );

        let config = &self.config.transcoding;
        let ffmpeg = Ffmpeg::new(&config.ffmpeg_path);

        let mut conn = self.db.acquire().await?;
        let path: String = sqlx::query_scalar("SELECT path FROM video_files WHERE item_id = ?")
            .bind(item_id)
            .fetch_one(&mut conn)
            .await?;

        let mut child = ffmpeg.transcode_hls(&HlsTranscodeOptions {
            input_path: &path,
            start_time: job.playlist[segment as usize].0,
            start_number: segment,
            segment_time: 6,
        })?;

        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            tokio::select! {
                _ = child.wait() => {}
                _ = rx => {
                    child.kill().await.unwrap();
                }
            }
        });

        job.canceller = Some(tx);

        Ok(())
    }

    pub async fn receive_segment(&self, segment: u32, mut stream: Body) {
        loop {
            let last_request = self
                .current
                .lock()
                .await
                .as_ref()
                .unwrap()
                .last_requested_segment;

            if segment <= last_request + 10 {
                break;
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        log::info!("receiving segment {}", segment);

        let mut data = Vec::new();

        while let Some(Ok(bytes)) = stream.next().await {
            data.extend_from_slice(&bytes);
        }

        self.current
            .lock()
            .await
            .as_mut()
            .unwrap()
            .segments
            .insert(segment, Bytes::from(data));
    }
}
