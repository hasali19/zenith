use std::io::BufRead;
use std::process::Stdio;

use eyre::eyre;
use once_cell::sync::Lazy;
use regex::Regex;
use tokio::process::Command;

use crate::ext::CommandExt;

pub struct CropDetect<'a> {
    ffmpeg_path: &'a str,
    duration: f64,
}

pub struct CropResult {
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub y2: u32,
}

impl<'a> CropDetect<'a> {
    pub fn new(ffmpeg_path: &'a str) -> Self {
        CropDetect {
            ffmpeg_path,
            duration: 10.0,
        }
    }

    pub async fn run(&self, input: &str) -> eyre::Result<CropResult> {
        let output = Command::new(self.ffmpeg_path)
            .arg_pair("-i", input)
            .arg_pair("-t", &self.duration.to_string())
            .arg_pair("-vf", "cropdetect")
            .arg_pair("-f", "null")
            .arg("-")
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            return Err(eyre!("ffmpeg terminated unsuccessfully: {}", output.status));
        }

        let stderr = output.stderr.as_slice();

        stderr
            .lines()
            .flatten()
            .filter_map(|line| parse_log_line(&line))
            .last()
            .ok_or_else(|| eyre!("failed to find any crop results in ffmpeg output"))
    }
}

fn parse_log_line(line: &str) -> Option<CropResult> {
    static LINE_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"x1:(\d+) x2:(\d+) y1:(\d+) y2:(\d+) w:\d+ h:\d+ x:\d+ y:\d+ pts:\d+ t:\S+ crop=\d+:\d+:\d+:\d+").unwrap()
    });

    let captures = LINE_REGEX.captures(line)?;

    let capture = |index| captures.get(index).unwrap().as_str().parse().ok();

    Some(CropResult {
        x1: capture(1)?,
        x2: capture(2)?,
        y1: capture(3)?,
        y2: capture(4)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ffmpeg_output() {
        let line = "[Parsed_cropdetect_0 @ 0x55c7297fd1c0] x1:0 x2:1919 y1:77 y2:1079 w:1920 h:992 x:0 y:84 pts:1063 t:1.063000 crop=1920:992:0:84";
        assert!(matches!(
            parse_log_line(line),
            Some(CropResult {
                x1: 0,
                x2: 1919,
                y1: 77,
                y2: 1079
            })
        ));
    }

    #[test]
    fn parse_other_output_fails() {
        let line = "frame=   24 fps=0.0 q=-0.0 Lsize=N/A time=00:00:01.00 bitrate=N/A speed=  17x";
        assert!(parse_log_line(line).is_none());
    }
}
