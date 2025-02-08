use std::fs::File;
use std::io::BufReader;

use camino::{Utf8Path, Utf8PathBuf};
use eyre::{Context, eyre};
use regex::Regex;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub logging: Logging,
    #[serde(default)]
    pub http: Http,
    pub libraries: Libraries,
    #[serde(default)]
    pub paths: Paths,
    pub tmdb: Tmdb,
    #[serde(default)]
    pub transcoding: Transcoding,
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub import: Import,
    #[serde(default)]
    pub subtitles: Subtitles,
    #[serde(default)]
    pub watcher: Watcher,
    #[serde(default)]
    pub cast: Cast,
}

#[derive(Default, Deserialize)]
pub struct Logging {
    #[serde(default)]
    pub format: LogFormat,
    pub filter: Option<String>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    #[cfg_attr(not(debug_assertions), default)]
    Compact,
    #[cfg_attr(debug_assertions, default)]
    Pretty,
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
    #[serde(deserialize_with = "deserialize_lib_path")]
    pub movies: Utf8PathBuf,
    #[serde(deserialize_with = "deserialize_lib_path")]
    pub tv_shows: Utf8PathBuf,
}

#[derive(Deserialize)]
pub struct Paths {
    #[serde(default = "Paths::default_cache")]
    pub cache: Utf8PathBuf,
    #[serde(default = "Paths::default_data")]
    pub data: Utf8PathBuf,
}

impl Paths {
    fn default_cache() -> Utf8PathBuf {
        "cache".into()
    }

    fn default_data() -> Utf8PathBuf {
        "data".into()
    }
}

impl Default for Paths {
    fn default() -> Self {
        Paths {
            cache: Self::default_cache(),
            data: Self::default_data(),
        }
    }
}

#[derive(Deserialize)]
pub struct Tmdb {
    pub api_key: String,
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
    #[serde(default)]
    pub matchers: Vec<ImportMatcher>,
}

#[derive(Clone, Deserialize)]
pub struct ImportMatcher {
    pub target: ImportMatcherTarget,
    #[serde(deserialize_with = "deserialize_regex")]
    pub regex: Regex,
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ImportMatcherTarget {
    Movie,
    Episode,
}

#[derive(Deserialize)]
pub struct Subtitles {
    #[serde(default = "Subtitles::default_path")]
    pub path: Utf8PathBuf,
}

#[derive(Deserialize)]
pub struct Watcher {
    #[serde(default = "Watcher::default_enabled")]
    pub enabled: bool,
}

#[derive(Default, Deserialize)]
pub struct Cast {
    pub app_id: Option<String>,
}

impl Config {
    pub fn load(path: &str) -> eyre::Result<Self> {
        let file =
            File::open(path).wrap_err_with(|| eyre!("failed to read config file: {path}"))?;
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
    fn default_path() -> Utf8PathBuf {
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

impl Watcher {
    fn default_enabled() -> bool {
        true
    }
}

impl Default for Watcher {
    fn default() -> Self {
        Watcher {
            enabled: Watcher::default_enabled(),
        }
    }
}

fn deserialize_regex<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Regex, D::Error> {
    struct RegexVisitor;

    impl Visitor<'_> for RegexVisitor {
        type Value = Regex;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid regex")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Regex::new(v).map_err(E::custom)
        }
    }

    deserializer.deserialize_str(RegexVisitor)
}

fn deserialize_lib_path<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Utf8PathBuf, D::Error> {
    struct PathBufVisitor;

    impl Visitor<'_> for PathBufVisitor {
        type Value = Utf8PathBuf;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid path")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Utf8Path::new(v).canonicalize_utf8().map_err(E::custom)
        }
    }

    deserializer.deserialize_str(PathBufVisitor)
}
