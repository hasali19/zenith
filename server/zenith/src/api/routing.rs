use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{middleware, Extension, Json};
use serde_qs::axum::QsQueryConfig;
use tower_http::cors::{AllowMethods, AllowOrigin, CorsLayer};

use crate::App;

use super::error::ApiError;

const DOCS_INDEX: &str = include_str!("docs.html");

#[allow(clippy::needless_borrow)]
pub mod routes {
    include!("../../router.rs");
}

pub fn router() -> axum::Router<App> {
    let spec = super::openapi_spec();

    routes::router()
        .route("/", get(|| async move { Html(DOCS_INDEX) }))
        .route("/openapi.json", get(|| async move { Json(spec) }))
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_origin(AllowOrigin::mirror_request())
                .allow_methods(AllowMethods::mirror_request())
                .allow_headers([ACCEPT, CONTENT_TYPE]),
        )
        .layer(middleware::from_fn(error_handler))
        .layer(Extension(QsQueryConfig::new(5, false)))
}

async fn error_handler<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let mut res = next.run(req).await;

    let error = res.extensions_mut().remove::<ApiError>();
    if let Some(error) = error {
        if res.status().is_client_error() {
            tracing::error!(method = %method, uri = %uri, status = %res.status(), "{:#}", error.inner);
        } else {
            tracing::error!("{:?}", error.inner);
        }
    }

    res
}
