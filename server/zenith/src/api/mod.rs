mod error;
mod ext;
mod import;
mod items;
mod metadata;
mod movies;
mod progress;
mod routing;
mod scanner;
mod subtitles;
mod transcoder;
mod tv;
mod videos;

use self::error::ApiError;

pub use self::routing::{openapi_spec, router};

pub type ApiResult<T> = Result<T, ApiError>;
