use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Stdio;

use eyre::eyre;
use tokio::process::{Child, Command};

trait CommandExt {
    fn arg_pair(&mut self, arg1: impl AsRef<OsStr>, arg2: impl AsRef<OsStr>) -> &mut Self;
}

impl CommandExt for Command {
    fn arg_pair(&mut self, arg1: impl AsRef<OsStr>, arg2: impl AsRef<OsStr>) -> &mut Self {
        self.arg(arg1);
        self.arg(arg2);
        self
    }
}

pub struct Ffprobe {
    exe_path: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct FfprobeOutput {
    pub format: FfprobeFormat,
    pub streams: Vec<FfprobeStream>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FfprobeFormat {
    pub duration: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct FfprobeStream {
    pub index: u32,
    pub codec_type: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl Ffprobe {
    pub fn new(exe_path: impl Into<String>) -> Self {
        Ffprobe {
            exe_path: exe_path.into(),
        }
    }

    pub async fn get_video_info(&self, path: &str) -> eyre::Result<FfprobeOutput> {
        let output = Command::new(&self.exe_path)
            .arg_pair("-loglevel", "error")
            .arg_pair("-print_format", "json")
            .arg("-show_format")
            .arg("-show_streams")
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()
            .await?;

        if !output.status.success() {
            return Err(eyre!("ffprobe terminated unsuccessfully"));
        }

        let stdout = String::from_utf8(output.stdout)?;
        let data: FfprobeOutput = serde_json::from_str(&stdout)?;

        Ok(data)
    }
}

pub struct VideoInfo {
    pub duration: f64,
}

#[async_trait::async_trait]
pub trait VideoInfoProvider {
    async fn get_video_info(&self, path: &str) -> eyre::Result<VideoInfo>;
}

#[async_trait::async_trait]
impl VideoInfoProvider for Ffprobe {
    async fn get_video_info(&self, path: &str) -> eyre::Result<VideoInfo> {
        let info = match self.get_video_info(path).await {
            Err(e) => return Err(e),
            Ok(info) => VideoInfo {
                duration: info.format.duration.parse()?,
            },
        };

        Ok(info)
    }
}

pub struct Ffmpeg {
    exe_path: String,
}

pub struct TranscodeOptions<'a> {
    pub input_path: &'a str,
    pub start_time: u64,
}

pub struct HlsTranscodeOptions<'a> {
    pub input_path: &'a str,
    pub start_number: u32,
    pub segment_time: u32,
    pub use_hw_encoder: bool,
}

pub struct SubtitleOptions<'a> {
    pub input_path: &'a str,
    pub start_time: u64,
    pub stream_index: u32,
}

impl Ffmpeg {
    pub fn new(exe_path: impl Into<String>) -> Self {
        Ffmpeg {
            exe_path: exe_path.into(),
        }
    }

    pub fn transcode(&self, options: &TranscodeOptions) -> std::io::Result<Child> {
        let log_dir = Path::new("transcode-logs");
        if !log_dir.is_dir() {
            std::fs::create_dir(log_dir)?;
        }

        let ffreport = format!("file={}/%p-%t.log:level=32", log_dir.to_string_lossy());

        Command::new(&self.exe_path)
            .env("FFREPORT", ffreport)
            .arg("-noaccurate_seek")
            .arg_pair("-ss", options.start_time.to_string())
            .arg_pair("-i", options.input_path)
            .arg_pair("-c:v", "copy")
            .arg_pair("-c:a", "libmp3lame")
            .arg_pair("-f", "mp4")
            .arg_pair("-movflags", "frag_keyframe+empty_moov")
            .arg("pipe:1")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
    }

    pub fn transcode_hls(&self, options: &HlsTranscodeOptions) -> eyre::Result<Child> {
        let log_dir = Path::new("transcode-logs");
        if !log_dir.is_dir() {
            std::fs::create_dir(log_dir)?;
        }

        let ffreport = format!("file={}/%p-%t.log:level=32", log_dir.to_string_lossy());

        let start_time = options.start_number * options.segment_time;
        let segment_time = options.segment_time.to_string();
        let key_frames = format!("expr:gte(t,{}+n_forced*{})", start_time, segment_time);

        // TODO: Remove hardcoded urls
        let segment_filename = "http://localhost:8000/api/hls/receiver/%d.ts";
        let playlist_filename = "http://localhost:8000/api/hls/receiver/main.m3u8";

        let mut cmd = Command::new(&self.exe_path);

        cmd.env("FFREPORT", ffreport)
            .arg_pair("-ss", start_time.to_string())
            .arg_pair("-i", options.input_path)
            .arg_pair("-map_metadata", "-1")
            .arg_pair("-map_chapters", "-1")
            .arg_pair("-threads", "0")
            .arg("-sn");

        if options.use_hw_encoder {
            cmd.arg_pair("-c:v", "h264_nvenc");
            cmd.arg_pair("-profile", "high");
        } else {
            cmd.arg_pair("-c:v", "libx264");
            cmd.arg_pair("-preset", "veryfast");
        }

        let child = cmd
            .arg_pair("-force_key_frames:0", key_frames)
            .arg_pair("-g", "72")
            .arg_pair("-keyint_min", "72")
            .arg_pair("-sc_threshold", "0")
            .arg("-start_at_zero")
            .arg_pair("-vsync", "-1")
            .arg_pair("-codec:a", "libmp3lame")
            .arg("-copyts")
            .arg_pair("-avoid_negative_ts", "disabled")
            .arg_pair("-f", "hls")
            .arg_pair("-max_delay", "5000000")
            .arg_pair("-hls_time", segment_time)
            .arg_pair("-individual_header_trailer", "0")
            .arg_pair("-hls_segment_type", "mpegts")
            .arg_pair("-start_number", options.start_number.to_string())
            .arg_pair("-hls_segment_filename", segment_filename)
            .arg_pair("-hls_playlist_type", "vod")
            .arg_pair("-hls_list_size", "0")
            .arg_pair("-http_persistent", "1")
            .arg_pair("-method", "PUT")
            .arg(playlist_filename)
            .stderr(Stdio::null())
            .spawn()?;

        Ok(child)
    }

    pub fn extract_subtitles(&self, options: &SubtitleOptions) -> std::io::Result<Child> {
        Command::new(&self.exe_path)
            .arg_pair("-ss", options.start_time.to_string())
            .arg_pair("-i", options.input_path)
            .arg_pair("-map", format!("0:{}", options.stream_index))
            .arg_pair("-c:s", "webvtt")
            .arg_pair("-f", "webvtt")
            .arg("pipe:1")
            .stdout(Stdio::piped())
            .spawn()
    }
}
