pub mod library;
pub mod movies;
pub mod stream;
pub mod tv_shows;

use hyper::{Body, Response, StatusCode};

use crate::server::App;
use crate::AppState;

pub fn configure(app: &mut App<AppState>) {
    app.configure(library::configure);
    app.configure(stream::configure);
    app.configure(movies::configure);
}

type ApiResponse = Response<Body>;
type ApiResult<T = ApiResponse> = Result<T, ApiError>;

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
        Response::builder()
            .status(e.status)
            .body(e.body)
            .unwrap()
            .into()
    }
}

impl From<eyre::Report> for ApiError {
    fn from(e: eyre::Report) -> Self {
        ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: Body::from(e.to_string()),
        }
    }
}
