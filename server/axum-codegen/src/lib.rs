use std::future::Future;
use std::pin::Pin;

use axum::extract::RawBody;
use axum::http::request::Parts;
use axum::http::{Method, StatusCode};
use axum::response::Response;
use okapi::schemars::gen::SchemaGenerator;
use okapi::schemars::schema::Schema;

pub use axum_codegen_macros::*;

pub enum ParamLocation {
    Path,
    Query,
}

pub struct ParamSpec {
    pub location: ParamLocation,
    pub name: String,
    pub schema: Schema,
}

pub struct RequestSpec {
    pub schema: Schema,
}

#[derive(Debug)]
pub struct ResponseSpec {
    pub status: StatusCode,
    pub description: Option<String>,
    pub schema: Option<Schema>,
}

pub trait Route: Send + Sync {
    fn path(&self) -> &'static str;
    fn method(&self) -> Method;

    fn src_file(&self) -> &'static str;

    fn handle(
        &self,
        parts: Parts,
        body: RawBody,
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;

    fn doc(&self) -> Option<&'static str> {
        None
    }

    fn params(&self, schema_gen: &mut SchemaGenerator) -> Vec<ParamSpec>;
    fn request(&self, schema_gen: &mut SchemaGenerator) -> Option<RequestSpec>;
    fn responses(&self, schema_gen: &mut SchemaGenerator) -> Vec<ResponseSpec>;
}
