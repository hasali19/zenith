mod auth;
mod cast;
mod collections;
mod dto;
mod error;
mod ext;
mod images;
mod import;
mod items;
mod metadata;
mod movies;
mod openapi;
mod progress;
mod routing;
mod scanner;
mod server;
mod subtitles;
mod trakt;
mod transcoder;
mod tv;
mod users;
mod videos;

use self::error::ApiError;

pub use self::openapi::openapi_spec;
pub use self::routing::router;

pub type ApiResult<T> = Result<T, ApiError>;
