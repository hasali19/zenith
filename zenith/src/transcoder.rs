use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use futures::StreamExt;
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

    pub async fn generate_playlist(&self, item_id: i64) -> Option<String> {
        let mut conn = self.db.acquire().await.ok()?;
        let duration: f64 =
            sqlx::query_scalar("SELECT duration FROM video_files WHERE item_id = ?")
                .bind(item_id)
                .fetch_optional(&mut conn)
                .await
                .ok()??;

        let segments = (duration / 6.0).ceil() as i32;
        let mut playlist = String::new();

        playlist.push_str("#EXTM3U\n");
        playlist.push_str("#EXT-X-PLAYLIST-TYPE:VOD\n");
        playlist.push_str("#EXT-X-VERSION:3\n");
        playlist.push_str("#EXT-X-TARGETDURATION:6\n");
        playlist.push_str("#EXT-X-MEDIA-SEQUENCE:0\n");

        for i in 0..segments {
            let length = if i == segments - 1 {
                6.0 * (1 - segments) as f64 + duration
            } else {
                6.0
            };

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

            // TODO: Make this nicer
            let mut job = match current.take() {
                None => self.spawn_ffmpeg(item_id, segment).await?,
                Some(job) if job.item_id != item_id => self.spawn_ffmpeg(item_id, segment).await?,
                Some(job) => {
                    if let Some(data) = job.segments.get(&segment) {
                        let data = data.clone();
                        *current = Some(job);
                        return Ok(Some(data));
                    }

                    if job.canceller.is_none() {
                        self.spawn_ffmpeg(item_id, segment).await?
                    } else if segment < job.last_requested_segment
                        || segment > job.last_requested_segment + 20
                    {
                        log::warn!("restarting transcode due to out of range seek");
                        self.spawn_ffmpeg(item_id, segment).await?
                    } else {
                        job
                    }
                }
            };

            job.last_requested_segment = segment;

            *current = Some(job);

            drop(current);

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    async fn spawn_ffmpeg(&self, item_id: i64, segment: u32) -> eyre::Result<JobState> {
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
            start_number: segment,
            segment_time: 6,
            use_hw_encoder: config.use_hw_encoder,
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

        Ok(JobState {
            item_id,
            last_requested_segment: segment,
            segments: HashMap::new(),
            canceller: Some(tx),
        })
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

            if segment < last_request + 10 {
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
