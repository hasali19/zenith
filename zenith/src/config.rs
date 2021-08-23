use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub http: Http,
    pub libraries: Libraries,
    pub tmdb: Tmdb,
    #[serde(default)]
    pub transcoding: Transcoding,
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub import: Import,
    #[serde(default)]
    pub subtitles: Subtitles,
}

#[derive(Deserialize)]
pub struct Http {
    #[serde(default = "Http::default_host")]
    pub host: String,
    #[serde(default = "Http::default_port")]
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Libraries {
    pub movies: String,
    pub tv_shows: String,
}

#[derive(Deserialize)]
pub struct Tmdb {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct Transcoding {
    #[serde(default = "Transcoding::default_ffprobe_path")]
    pub ffprobe_path: String,
    #[serde(default = "Transcoding::default_ffmpeg_path")]
    pub ffmpeg_path: String,
}

#[derive(Deserialize)]
pub struct Database {
    #[serde(default = "Database::default_path")]
    pub path: String,
}

#[derive(Default, Deserialize)]
pub struct Import {
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Deserialize)]
pub struct Subtitles {
    #[serde(default = "Subtitles::default_path")]
    pub path: PathBuf,
}

impl Config {
    pub fn load(path: &str) -> eyre::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_yaml::from_reader(reader)?)
    }
}

impl Http {
    fn default_host() -> String {
        "0.0.0.0".into()
    }

    fn default_port() -> u16 {
        8000
    }
}

impl Default for Http {
    fn default() -> Self {
        Http {
            host: Http::default_host(),
            port: Http::default_port(),
        }
    }
}

impl Transcoding {
    fn default_ffprobe_path() -> String {
        "ffprobe".into()
    }

    fn default_ffmpeg_path() -> String {
        "ffmpeg".into()
    }
}

impl Default for Transcoding {
    fn default() -> Self {
        Transcoding {
            ffprobe_path: Transcoding::default_ffprobe_path(),
            ffmpeg_path: Transcoding::default_ffmpeg_path(),
        }
    }
}

impl Database {
    fn default_path() -> String {
        "zenith.db".into()
    }
}

impl Default for Database {
    fn default() -> Self {
        Database {
            path: Database::default_path(),
        }
    }
}

impl Subtitles {
    fn default_path() -> PathBuf {
        "data/subtitles".into()
    }
}

impl Default for Subtitles {
    fn default() -> Self {
        Subtitles {
            path: Subtitles::default_path(),
        }
    }
}
