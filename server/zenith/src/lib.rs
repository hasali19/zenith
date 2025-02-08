#![feature(let_chains)]

mod cropdetect;
mod ext;
mod password_utils;

pub mod api;
pub mod config;
pub mod library;
pub mod metadata;
pub mod server;
pub mod subtitles;
pub mod transcoder;
pub mod util;
pub mod video_prober;

use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
pub use db::Db;
pub use db::media::MediaItemType;

#[derive(Clone)]
pub struct App {
    pub key: Key,
}

impl FromRef<App> for Key {
    fn from_ref(input: &App) -> Self {
        input.key.clone()
    }
}

speq::axum_config!(App);
