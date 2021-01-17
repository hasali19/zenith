use std::process::Stdio;

use eyre::eyre;
use tokio::process::Command;

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
