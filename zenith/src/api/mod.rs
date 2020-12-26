pub mod movies;
pub mod stream;
pub mod tv_shows;

use actix_web::dev::HttpServiceFactory;
use actix_web::http::StatusCode;
use actix_web::{web, ResponseError};

pub type ApiResult<T> = Result<T, ApiError>;

pub fn service(path: &str) -> impl HttpServiceFactory {
    web::scope(path)
        .service(movies::service("/movies"))
        .service(tv_shows::service("/tv_shows"))
        .service(stream::service("/stream"))
}

#[derive(Debug, derive_more::Display)]
pub enum ApiError {
    NotFound,
    InternalError(Box<dyn std::error::Error>),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl<T: std::error::Error + 'static> From<T> for ApiError {
    fn from(e: T) -> Self {
        ApiError::InternalError(Box::new(e))
    }
}
