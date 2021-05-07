use std::collections::HashMap;
use std::process::Stdio;

use eyre::eyre;
use serde::Deserialize;
use serde_json::Value;
use tokio::process::Command;

use crate::ext::CommandExt;

pub struct Ffprobe {
    path: String,
}

#[derive(Deserialize)]
pub struct Output {
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
    pub codec_name: String,
    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

impl Ffprobe {
    pub fn new(path: impl Into<String>) -> Ffprobe {
        Ffprobe { path: path.into() }
    }

    pub async fn probe(&self, path: &str) -> eyre::Result<Output> {
        let output = Command::new(&self.path)
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

        Ok(serde_json::from_slice(&output.stdout)?)
    }
}