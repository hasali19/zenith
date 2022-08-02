use axum::body::Body;
use axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use axum::http::{HeaderValue, Response, StatusCode};
use axum::response::Html;
use axum::routing::get;
use axum::{Json, Router};
use tower_http::set_header::SetResponseHeaderLayer;

const DOCS_INDEX: &str = include_str!("docs/docs.html");
const RAPIDOC_JS: &str = include_str!("docs/rapidoc-min.js");

pub fn router() -> axum::Router {
    let spec = super::openapi_spec();

    axum_codegen::routes()
        .fold(Router::new(), |router, route| route.register(router))
        .route("/", get(|| async move { Html(DOCS_INDEX) }))
        .route(
            "/rapidoc-min.js",
            get(|| async move {
                Response::builder()
                    .header("content-type", "application/javascript; charset=utf-8")
                    .body(Body::from(RAPIDOC_JS))
                    .map_err(|e| {
                        tracing::error!("{e}");
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
            }),
        )
        .route("/openapi.json", get(|| async move { Json(spec) }))
        .layer(SetResponseHeaderLayer::overriding(
            ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ))
}
