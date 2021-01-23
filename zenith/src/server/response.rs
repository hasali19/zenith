use hyper::{header, Body, Response as HyperResponse, StatusCode};
use serde::Serialize;

pub struct Response(HyperResponse<Body>);

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

impl From<StatusCode> for Response {
    fn from(status: StatusCode) -> Self {
        HyperResponse::builder()
            .status(status)
            .body(Body::empty())
            .unwrap()
            .into()
    }
}

pub struct JsonResponse(Response);

impl<T: Serialize> From<T> for JsonResponse {
    fn from(val: T) -> Self {
        let res = match serde_json::to_vec(&val) {
            Ok(json) => HyperResponse::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json))
                .unwrap()
                .into(),
            Err(_) => HyperResponse::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
                .into(),
        };

        JsonResponse(res)
    }
}

impl From<JsonResponse> for Response {
    fn from(JsonResponse(res): JsonResponse) -> Self {
        res
    }
}
