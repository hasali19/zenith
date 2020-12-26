use std::process::Stdio;

use eyre::eyre;
use tokio::process::Command;

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

pub async fn get_video_info(path: &str) -> eyre::Result<VideoInfo> {
    let output = Command::new("ffprobe")
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
