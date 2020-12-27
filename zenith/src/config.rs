use std::fs::File;
use std::io::BufReader;

#[derive(serde::Deserialize)]
pub struct Config {
    pub movie_path: String,
    pub tv_show_path: String,
    pub tmdb_access_token: String,
    pub db_path: Option<String>,
}

impl Config {
    pub fn load(path: &str) -> eyre::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_yaml::from_reader(reader)?;
        Ok(config)
    }
}
