use std::collections::HashMap;
use std::process::Stdio;

use async_trait::async_trait;
use camino::Utf8Path;
use eyre::{Context, eyre};
use serde::Deserialize;
use serde_json::Value;
use tokio::process::Command;

use crate::ext::CommandExt;

pub struct Ffprobe {
    path: String,
}

#[derive(Deserialize)]
pub struct VideoInfo {
    pub format: Format,
    pub streams: Vec<Stream>,
}

#[derive(Deserialize)]
pub struct Format {
    pub format_name: String,
    pub duration: String,
}

#[derive(Deserialize)]
pub struct Stream {
    pub index: u32,
    pub codec_type: String,
    pub codec_name: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub channels: Option<u32>,
    pub channel_layout: Option<String>,
    pub tags: Option<StreamTags>,
    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

#[derive(Deserialize)]
pub struct StreamTags {
    pub title: Option<String>,
    pub language: Option<String>,
}

impl Ffprobe {
    pub fn new(path: impl Into<String>) -> Ffprobe {
        Ffprobe { path: path.into() }
    }

    async fn probe(&self, path: &Utf8Path) -> eyre::Result<VideoInfo> {
        let json_path = path.with_extension("ffprobe.json");

        let output = if let Ok(bytes) = tokio::fs::read(&json_path).await {
            bytes
        } else {
            let mut ffprobe = Command::new(&self.path);

            ffprobe
                .arg_pair("-loglevel", "error")
                .arg_pair("-print_format", "json")
                .arg("-show_format")
                .arg("-show_streams")
                .arg(path)
                .stdout(Stdio::piped())
                .stderr(Stdio::null());

            let output = ffprobe
                .spawn()
                .wrap_err_with(|| eyre!("failed to spawn command: {:?}", ffprobe.as_std()))?
                .wait_with_output()
                .await?;

            if !output.status.success() {
                return Err(eyre!("ffprobe terminated unsuccessfully"));
            }

            output.stdout
        };

        Ok(serde_json::from_slice(&output)?)
    }
}

#[cfg_attr(any(test, feature = "mocks"), mockall::automock)]
#[async_trait]
pub trait VideoProber: Send + Sync {
    async fn probe(&self, path: &Utf8Path) -> eyre::Result<VideoInfo>;
}

#[async_trait]
impl VideoProber for Ffprobe {
    async fn probe(&self, path: &Utf8Path) -> eyre::Result<VideoInfo> {
        Ffprobe::probe(self, path).await
    }
}
