pub mod library;
pub mod movies;
pub mod stream;
pub mod tv_shows;

use hyper::{Body, StatusCode};

use crate::server::{App, Response};
use crate::AppState;

pub fn configure(app: &mut App<AppState>) {
    app.configure(library::configure);
    app.configure(stream::configure);
    app.configure(movies::configure);
    app.configure(tv_shows::configure);
}

type ApiResult<T = Response> = Result<T, ApiError>;

#[derive(Debug)]
struct ApiError {
    status: StatusCode,
    body: Body,
}

impl ApiError {
    pub fn new(status: StatusCode, body: Body) -> Self {
        ApiError { status, body }
    }

    pub fn from_status(status: StatusCode) -> Self {
        ApiError::new(status, Body::empty())
    }

    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.body = body.into();
        self
    }

    pub fn bad_request() -> Self {
        ApiError::from_status(StatusCode::BAD_REQUEST)
    }

    pub fn internal_server_error() -> Self {
        ApiError::from_status(StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn not_found() -> Self {
        ApiError::from_status(StatusCode::NOT_FOUND)
    }
}

impl From<ApiError> for crate::server::Response {
    fn from(e: ApiError) -> Self {
        hyper::Response::builder()
            .status(e.status)
            .body(e.body)
            .unwrap()
            .into()
    }
}

impl From<eyre::Report> for ApiError {
    fn from(e: eyre::Report) -> Self {
        ApiError::internal_server_error().body(e.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError::internal_server_error().body(e.to_string())
    }
}
