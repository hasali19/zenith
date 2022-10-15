use axum::body::Body;
use axum::http::{Request, Response, StatusCode};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{middleware, Json};
use tower_http::cors::{self, CorsLayer};

use super::error::ApiError;

const DOCS_INDEX: &str = include_str!("docs/docs.html");
const RAPIDOC_JS: &str = include_str!("docs/rapidoc-min.js");

pub fn router() -> axum::Router {
    let spec = super::openapi_spec();

    speq::axum::router()
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
        .layer(CorsLayer::new().allow_origin(cors::Any))
        .layer(middleware::from_fn(error_handler))
}

async fn error_handler<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let mut res = next.run(req).await;
    let error = res.extensions_mut().remove::<ApiError>();
    if let Some(error) = error {
        tracing::error!("{:?}", error.inner);
    }
    res
}
