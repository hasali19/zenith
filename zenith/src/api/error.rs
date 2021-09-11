use std::error::Error as StdError;
use std::fmt::Display;

use atium::headers::ContentType;
use atium::{Response, StatusCode};

#[derive(Debug, Default)]
pub struct ErrorResponse {
    status: StatusCode,
    message: String,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, message: String) -> ErrorResponse {
        ErrorResponse { status, message }
    }
}

impl From<ErrorResponse> for Response {
    fn from(e: ErrorResponse) -> Self {
        Response::new()
            .with_status(e.status)
            .with_header(ContentType::json())
            .with_body(serde_json::to_vec(&serde_json::json!({"error": e.message})).unwrap())
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error({}): {}", self.status, self.message)
    }
}

impl StdError for ErrorResponse {}

pub fn bad_request(msg: impl Into<String>) -> ErrorResponse {
    ErrorResponse::new(StatusCode::BAD_REQUEST, msg.into())
}
