use std::mem;

use hyper::http::request::Parts;
use hyper::{Body, Request as HyperRequest};
use route_recognizer::Params;
use serde::de::DeserializeOwned;

pub struct Request {
    parts: Parts,
    params: Params,
    body: Body,
}

impl<'a> Request {
    pub fn new(req: HyperRequest<Body>, params: Params) -> Self {
        let (parts, body) = req.into_parts();
        Request {
            parts,
            params,
            body,
        }
    }

    pub fn param(&self, key: &str) -> Option<&str> {
        self.params.find(key)
    }

    pub fn query<T: DeserializeOwned>(&self) -> Result<T, serde_qs::Error> {
        let qs = self.parts.uri.query().unwrap_or("");
        serde_qs::from_str(qs)
    }

    pub async fn body_json<T: DeserializeOwned>(&mut self) -> eyre::Result<T> {
        let body = mem::replace(&mut self.body, Body::empty());
        let bytes = hyper::body::to_bytes(body).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}
