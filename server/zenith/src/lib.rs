#![feature(let_chains)]

mod ext;

pub mod api;
pub mod config;
pub mod library;
pub mod metadata;
pub mod subtitles;
pub mod transcoder;
pub mod util;
pub mod utils;
pub mod video_prober;

pub use db::media::MediaItemType;
pub use db::Db;

speq::axum_config!(());
