mod error;
mod ext;
mod import;
mod items;
mod metadata;
mod movies;
mod openapi;
mod progress;
mod routing;
mod scanner;
mod subtitles;
mod transcoder;
mod tv;
mod videos;

use self::error::ApiError;

pub use self::openapi::openapi_spec;
pub use self::routing::router;

pub type ApiResult<T> = Result<T, ApiError>;
