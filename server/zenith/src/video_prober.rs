use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;

use async_trait::async_trait;
use eyre::{eyre, Context};
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
    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

#[derive(Deserialize)]
pub struct Stream {
    pub index: u32,
    pub codec_type: String,
    pub codec_name: Option<String>,
    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

impl Ffprobe {
    pub fn new(path: impl Into<String>) -> Ffprobe {
        Ffprobe { path: path.into() }
    }

    async fn probe(&self, path: &str) -> eyre::Result<VideoInfo> {
        let json_path = Path::new(path).with_extension("ffprobe.json");

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

#[async_trait]
pub trait VideoProber: Send + Sync {
    async fn probe(&self, path: &str) -> eyre::Result<VideoInfo>;
}

#[async_trait::async_trait]
impl VideoProber for Ffprobe {
    async fn probe(&self, path: &str) -> eyre::Result<VideoInfo> {
        Ffprobe::probe(self, path).await
    }
}
