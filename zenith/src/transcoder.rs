use std::fs::File;
use std::path::PathBuf;
use std::process::Stdio;

use tokio::process::Command;

pub struct Transcoder {
    temp_dir: PathBuf,
    current_job: Option<Job>,
}

pub struct Job {
    video_id: i64,
    start_segment: i32,
    process_handle: tokio::sync::oneshot::Sender<()>,
}

impl Job {
    pub fn video_id(&self) -> i64 {
        self.video_id
    }

    pub fn start_segment(&self) -> i32 {
        self.start_segment
    }
}

impl Transcoder {
    #[allow(clippy::clippy::new_without_default)]
    pub fn new(temp_dir: &str) -> Self {
        Transcoder {
            current_job: None,
            temp_dir: PathBuf::from(temp_dir),
        }
    }

    pub fn is_transcoding(&self, video_id: i64) -> bool {
        matches!(self.current_job, Some(Job { video_id: id, .. }) if id == video_id)
    }

    pub fn get_last_segment_number(&self, video_id: i64) -> Option<i32> {
        std::fs::read_dir(&self.temp_dir)
            .ok()?
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_str().unwrap().to_string())
            .filter(|n| n.starts_with(&video_id.to_string()) && n.ends_with(".ts"))
            .filter_map(|n| get_segment_number_from_name(&n))
            .max()
    }

    pub fn get_segment(&self, video_id: i64, segment: i32) -> Option<(PathBuf, File)> {
        let filename = format!("{}__{}.ts", video_id, segment);
        let path = self.temp_dir.join(filename);
        File::open(&path).ok().map(|v| (path, v))
    }

    pub async fn begin_transcode(&mut self, video_id: i64, path: &str) {
        if let Some(job) = self.current_job.take() {
            log::warn!("killing existing transcode job");
            job.process_handle.send(()).unwrap();
        }

        log::info!("starting transcode");

        let segment_name_template = self.temp_dir.join(format!("{}__%d.ts", video_id));
        let playlist_name = self.temp_dir.join(format!("{}.m3u8", video_id));

        let (tx, rx) = tokio::sync::oneshot::channel();

        let mut cmd = Command::new("ffmpeg");

        cmd.arg("-noaccurate_seek")
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
            .arg("expr:gte(t,0+n_forced*3)")
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
            .arg("0")
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
            .stderr(Stdio::null());

        tokio::spawn(async move {
            let mut child = cmd.spawn().unwrap();

            // Wait for the child to finish, or a signal from the channel to kill it
            tokio::select! {
                _ = &mut child => {},
                Ok(_) = rx => {
                    child.kill().unwrap();
                    child.await.unwrap();
                },
            }
        });

        self.current_job = Some(Job {
            video_id,
            start_segment: 0,
            process_handle: tx,
        });
    }
}

fn get_segment_number_from_name(name: &str) -> Option<i32> {
    name.split("__")
        .nth(1)
        .and_then(|n| n.split('.').next())
        .map(|n| n.parse().unwrap())
}
