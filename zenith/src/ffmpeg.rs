use std::ffi::OsStr;
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
            .arg_pair("-loglevel", "error")
            .arg_pair("-print_format", "json")
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
    pub start_time: u64,
    pub use_hw_encoder: bool,
}

impl Ffmpeg {
    pub fn new(exe_path: impl Into<String>) -> Self {
        Ffmpeg {
            exe_path: exe_path.into(),
        }
    }

    pub fn spawn_transcode(&self, options: &TranscodeOptions) -> std::io::Result<Child> {
        let mut cmd = Command::new(&self.exe_path);

        cmd.arg_pair("-ss", options.start_time.to_string())
            .arg_pair("-i", options.input_path);

        if options.use_hw_encoder {
            cmd.arg_pair("-c:v", "h264_nvenc");
        } else {
            cmd.arg_pair("-c:v", "libx264");
            cmd.arg_pair("-preset", "veryfast");
        }

        cmd.arg_pair("-c:a", "libmp3lame")
            .arg_pair("-f", "mp4")
            .arg_pair("-movflags", "frag_keyframe+empty_moov")
            .arg("pipe:1")
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
    }
}
