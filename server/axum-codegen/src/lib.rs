use std::future::Future;
use std::pin::Pin;

use axum::extract::RawBody;
use axum::http::request::Parts;
use axum::response::Response;

pub use axum_codegen_macros::*;

pub type RequestHandler =
    fn(Parts, RawBody) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
