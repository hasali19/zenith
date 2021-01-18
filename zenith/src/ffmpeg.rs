use std::path::Path;
use std::process::Stdio;

use eyre::eyre;
use tokio::process::{Child, Command};

pub struct Ffprobe {
    exe_path: String,
}

#[derive(serde::Deserialize)]
struct FfprobeOutput {
    format: FfprobeFormat,
}

#[derive(serde::Deserialize)]
struct FfprobeFormat {
    duration: String,
}

pub struct VideoInfo {
    pub duration: f64,
}

impl Ffprobe {
    pub fn new(exe_path: impl Into<String>) -> Self {
        Ffprobe {
            exe_path: exe_path.into(),
        }
    }

    pub async fn get_video_info(&self, path: &str) -> eyre::Result<VideoInfo> {
        let output = Command::new(&self.exe_path)
            .arg("-loglevel")
            .arg("error")
            .arg("-print_format")
            .arg("json")
            .arg("-show_format")
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

        Ok(VideoInfo {
            duration: data
                .format
                .duration
                .parse()
                .map_err(|_| eyre!("invalid duration"))?,
        })
    }
}

pub struct Ffmpeg {
    exe_path: String,
}

pub struct TranscodeOptions<'a> {
    pub input_path: &'a str,
    pub start_number: u32,
    pub segment_time: u32,
    pub segment_filename: &'a Path,
    pub playlist_filename: &'a Path,
}

impl Ffmpeg {
    pub fn new(exe_path: impl Into<String>) -> Self {
        Ffmpeg {
            exe_path: exe_path.into(),
        }
    }

    pub fn spawn_transcode(&self, options: &TranscodeOptions) -> eyre::Result<Child> {
        let start_time = options.start_number * options.segment_time;
        let child = Command::new(&self.exe_path)
            .arg("-ss")
            .arg(start_time.to_string())
            .arg("-noaccurate_seek")
            .arg("-i")
            .arg(options.input_path)
            .arg("-map_metadata")
            .arg("-1")
            .arg("-map_chapters")
            .arg("-1")
            .arg("-threads")
            .arg("0")
            .arg("-codec:v")
            .arg("libx264")
            .arg("-preset")
            .arg("veryfast")
            .arg("-force_key_frames:0")
            .arg(format!("expr:gte(t,{}+n_forced*3)", start_time))
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
            .arg(options.start_number.to_string())
            .arg("-hls_segment_filename")
            .arg(options.segment_filename)
            .arg("-hls_playlist_type")
            .arg("vod")
            .arg("-hls_list_size")
            .arg("0")
            .arg("-hls_flags")
            .arg("temp_file")
            .arg("-y")
            .arg(options.playlist_filename)
            .stderr(Stdio::null())
            .spawn()?;

        Ok(child)
    }
}
