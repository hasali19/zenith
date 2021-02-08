use headers::{ContentType, Header, HeaderMapExt};
use hyper::{Body, Response as HyperResponse, StatusCode};
use serde::Serialize;

#[derive(Default)]
pub struct Response(HyperResponse<Body>);

impl Response {
    pub fn new() -> Self {
        Response(HyperResponse::new(Body::empty()))
    }

    pub fn status(&self) -> StatusCode {
        self.0.status()
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        *self.0.status_mut() = status;
        self
    }

    pub fn with_header<H: Header>(mut self, header: H) -> Self {
        self.0.headers_mut().typed_insert(header);
        self
    }

    pub fn body(&self) -> &Body {
        self.0.body()
    }

    pub fn with_body(mut self, body: Body) -> Self {
        *self.0.body_mut() = body;
        self
    }

    pub fn json<T: Serialize>(self, val: &T) -> serde_json::Result<Self> {
        Ok(self
            .with_header(ContentType::json())
            .with_body(Body::from(serde_json::to_vec(&val)?)))
    }
}

impl From<Response> for HyperResponse<Body> {
    fn from(Response(res): Response) -> Self {
        res
    }
}

impl From<HyperResponse<Body>> for Response {
    fn from(res: HyperResponse<Body>) -> Self {
        Response(res)
    }
}

impl<T, E> From<Result<T, E>> for Response
where
    T: Into<Response>,
    E: Into<Response>,
{
    fn from(res: Result<T, E>) -> Self {
        match res {
            Ok(res) => res.into(),
            Err(e) => e.into(),
        }
    }
}

impl From<()> for Response {
    fn from(_: ()) -> Self {
        Response::new()
    }
}

impl From<StatusCode> for Response {
    fn from(status: StatusCode) -> Self {
        HyperResponse::builder()
            .status(status)
            .body(Body::empty())
            .unwrap()
            .into()
    }
}
