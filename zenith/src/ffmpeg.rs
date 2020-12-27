use std::process::Stdio;

use actix_web::web::Bytes;
use eyre::eyre;
use futures::{Stream, StreamExt};
use tokio::process::Command;
use tokio_util::codec::{BytesCodec, FramedRead};

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

pub fn begin_transcode(
    start_time: f64,
    path: String,
) -> impl Stream<Item = Result<Bytes, actix_web::Error>> {
    let (mut tx, rx) = tokio::sync::mpsc::channel(1);

    actix_web::rt::spawn(async move {
        let mut child = tokio::process::Command::new("ffmpeg")
            .arg("-ss")
            .arg(start_time.to_string())
            .arg("-i")
            .arg(path)
            .arg("-c:v")
            .arg("copy")
            .arg("-c:a")
            .arg("aac")
            .arg("-f")
            .arg("matroska")
            .arg("pipe:1")
            .stdout(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .unwrap();

        let stdout = child.stdout.as_mut().unwrap();
        let mut stream = FramedRead::new(stdout, BytesCodec::new());

        while let Some(v) = stream.next().await {
            let v = v.map(Bytes::from).map_err(actix_web::Error::from);
            if tx.send(v).await.is_err() {
                child.kill().unwrap();
                break;
            }
        }

        child.await.unwrap();
    });

    rx
}
