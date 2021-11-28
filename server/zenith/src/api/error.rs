use axum::body::{Full, HttpBody};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use bytes::Bytes;
use eyre::eyre;
use serde_json::json;

pub struct ApiError {
    status: StatusCode,
    inner: eyre::Report,
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> Self {
        ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            inner: e.into(),
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            inner: e.into(),
        }
    }
}

impl From<eyre::Error> for ApiError {
    fn from(e: eyre::Error) -> Self {
        ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            inner: e,
        }
    }
}

pub fn bad_request(msg: impl ToString) -> ApiError {
    ApiError {
        status: StatusCode::BAD_REQUEST,
        inner: eyre!(msg.to_string()),
    }
}

pub fn not_found(msg: impl ToString) -> ApiError {
    ApiError {
        status: StatusCode::NOT_FOUND,
        inner: eyre!(msg.to_string()),
    }
}

impl IntoResponse for ApiError {
    type Body = Full<Bytes>;
    type BodyError = <Self::Body as HttpBody>::Error;

    fn into_response(self) -> Response<Self::Body> {
        let mut res = IntoResponse::into_response(Json(json!({"message": self.inner.to_string()})));
        *res.status_mut() = self.status;
        res
    }
}
