use atium::StatusCode;

use super::error::{self, ErrorResponse};

pub trait OptionExt<T> {
    fn or_bad_request(self, msg: impl Into<String>) -> Result<T, ErrorResponse>;
    fn or_not_found(self, msg: impl Into<String>) -> Result<T, ErrorResponse>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_bad_request(self, msg: impl Into<String>) -> Result<T, ErrorResponse> {
        self.ok_or_else(|| error::bad_request(msg))
    }

    fn or_not_found(self, msg: impl Into<String>) -> Result<T, ErrorResponse> {
        self.ok_or_else(|| ErrorResponse::new(StatusCode::NOT_FOUND, msg.into()))
    }
}
