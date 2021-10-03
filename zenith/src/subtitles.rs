use std::process::Stdio;

use eyre::eyre;
use tokio::io::AsyncWrite;
use tokio::process::Command;
use tokio_stream::Stream;

use crate::config::Config;
use crate::ext::CommandExt;
use crate::util;

pub async fn convert<I, E>(
    config: &Config,
    input: I,
    output: impl AsyncWrite + Unpin,
) -> eyre::Result<()>
where
    I: Stream<Item = Result<bytes::Bytes, E>> + Unpin,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut child = Command::new(&config.transcoding.ffmpeg_path)
        .arg_pair("-f", "srt")
        .arg_pair("-i", "-")
        .arg_pair("-f", "webvtt")
        .arg("pipe:1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.take().unwrap();
    let stdout = util::to_byte_stream(child.stdout.take().unwrap());

    // Copy input to ffmpeg stdin
    let input_fut = util::copy_stream(input, stdin);
    // Copy ffmpeg output to file
    let output_fut = util::copy_stream(stdout, output);

    // Wait for all futures to complete
    let (.., res) = tokio::join!(input_fut, output_fut, child.wait());

    if !res?.success() {
        return Err(eyre!("failed to convert subtitles"));
    }

    Ok(())
}

pub async fn extract_embedded(config: &Config, path: &str, index: u32) -> std::io::Result<Vec<u8>> {
    Command::new(&config.transcoding.ffmpeg_path)
        .arg_pair("-i", &path)
        .arg_pair("-map", format!("0:{}", index))
        .arg_pair("-c:s", "webvtt")
        .arg_pair("-f", "webvtt")
        .arg("pipe:1")
        .stdout(Stdio::piped())
        .output()
        .await
        .map(|output| output.stdout)
}
