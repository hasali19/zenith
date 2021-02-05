mod library;
mod metadata;
mod movies;
mod progress;
mod stream;
mod tv;

use zenith_server::{App, Body, Response, StatusCode};

use crate::AppState;

pub fn configure(app: &mut App<AppState>) {
    app.configure(library::configure);
    app.configure(movies::configure);
    app.configure(tv::configure);
    app.configure(stream::configure);
    app.configure(progress::configure);
    app.configure(metadata::configure);
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

impl From<ApiError> for Response {
    fn from(e: ApiError) -> Self {
        Response::new().with_status(e.status).with_body(e.body)
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

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError::internal_server_error().body(e.to_string())
    }
}
