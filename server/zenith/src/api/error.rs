use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct ApiError(actix_web::Error);

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.0.as_response_error().status_code()
    }
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> Self {
        ApiError(ErrorInternalServerError(e))
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError(ErrorInternalServerError(e))
    }
}

impl From<eyre::Error> for ApiError {
    fn from(e: eyre::Error) -> Self {
        ApiError(ErrorInternalServerError(e))
    }
}

pub fn bad_request(msg: impl ToString) -> ApiError {
    ApiError(ErrorBadRequest(msg.to_string()))
}

pub fn not_found(msg: impl ToString) -> ApiError {
    ApiError(ErrorNotFound(msg.to_string()))
}
