use std::future::Future;
use std::pin::Pin;

use axum::body::{Body, BoxBody};
use axum::http::request::Parts;
use axum::http::Response;

pub use axum_codegen_macros::*;

pub type RequestHandler =
    fn(Parts, Body) -> Pin<Box<dyn Future<Output = Response<BoxBody>> + Send + 'static>>;
