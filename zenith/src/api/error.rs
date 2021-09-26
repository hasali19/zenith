use std::error::Error as StdError;
use std::fmt::Display;

use async_trait::async_trait;
use atium::headers::ContentType;
use atium::query::QueryError;
use atium::router::ParamError;
use atium::{Handler, Response, StatusCode};

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

impl<T, E: From<ErrorResponse>> From<ErrorResponse> for Result<T, E> {
    fn from(val: ErrorResponse) -> Self {
        Err(val.into())
    }
}

pub fn bad_request(msg: impl ToString) -> ErrorResponse {
    ErrorResponse::new(StatusCode::BAD_REQUEST, msg.to_string())
}

pub fn not_found(msg: impl ToString) -> ErrorResponse {
    ErrorResponse::new(StatusCode::NOT_FOUND, msg.to_string())
}

pub struct ErrorHandler;

#[async_trait]
impl Handler for ErrorHandler {
    async fn run(&self, req: atium::Request, next: &dyn atium::Next) -> atium::Request {
        let mut req = next.run(req).await;

        if let Some(mut e) = req.take_ext::<eyre::Report>() {
            let res: ErrorResponse = if let Some(e) = e.downcast_mut::<ErrorResponse>() {
                std::mem::take(e)
            } else if let Some(e) = e.downcast_ref::<ParamError>() {
                bad_request(e.to_string())
            } else if let Some(e) = e.downcast_ref::<QueryError>() {
                bad_request(e.to_string())
            } else {
                ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            };

            req.set_res(res);
        }

        req
    }
}
