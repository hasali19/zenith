use std::fs::File;
use std::io::BufReader;

#[derive(serde::Deserialize)]
pub struct Config {
    pub movie_path: String,
    pub tv_show_path: String,
    pub tmdb_access_token: String,
    pub transcode_dir: String,
    #[serde(default)]
    pub use_hw_encoder: bool,
    db_path: Option<String>,
    ffprobe_path: Option<String>,
    ffmpeg_path: Option<String>,
}

impl Config {
    pub fn load(path: &str) -> eyre::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_yaml::from_reader(reader)?)
    }

    pub fn db_path(&self) -> &str {
        self.db_path.as_deref().unwrap_or("zenith.db")
    }

    pub fn ffprobe_path(&self) -> &str {
        self.ffprobe_path.as_deref().unwrap_or("ffprobe")
    }

    pub fn ffmpeg_path(&self) -> &str {
        self.ffmpeg_path.as_deref().unwrap_or("ffmpeg")
    }
}
