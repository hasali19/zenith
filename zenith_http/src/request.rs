use std::mem;

use headers::{Header, HeaderMapExt};
use hyper::http::request::Parts;
use hyper::{Body, Method, Request as HyperRequest, Uri};
use route_recognizer::Params;
use serde::de::DeserializeOwned;

pub struct Request {
    parts: Parts,
    params: Params,
    body: Body,
}

pub enum Error {
    HyperError(hyper::Error),
    JsonParseError(serde_json::Error),
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

    pub fn uri(&self) -> &Uri {
        &self.parts.uri
    }

    pub fn method(&self) -> &Method {
        &self.parts.method
    }

    pub fn param(&self, key: &str) -> Option<&str> {
        self.params.find(key)
    }

    pub fn header<T: Header>(&self) -> Option<T> {
        self.parts.headers.typed_get()
    }

    pub fn query<T: DeserializeOwned>(&self) -> Result<T, serde_qs::Error> {
        let qs = self.parts.uri.query().unwrap_or("");
        serde_qs::from_str(qs)
    }

    pub fn body(&mut self) -> Body {
        mem::replace(&mut self.body, Body::empty())
    }

    pub async fn body_json<T: DeserializeOwned>(&mut self) -> Result<T, Error> {
        let body = self.body();
        let bytes = hyper::body::to_bytes(body)
            .await
            .map_err(Error::HyperError)?;

        Ok(serde_json::from_slice(&bytes).map_err(Error::JsonParseError)?)
    }
}
