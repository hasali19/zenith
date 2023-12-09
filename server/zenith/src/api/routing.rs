use axum::body::Body;
use axum::extract::Request;
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Extension, Json};
use serde_qs::axum::QsQueryConfig;
use tower::ServiceBuilder;
use tower_http::cors::{AllowMethods, AllowOrigin, CorsLayer};

use crate::App;

use super::auth;
use super::error::ApiError;

const DOCS_INDEX: &str = include_str!("docs.html");

pub fn router(state: App) -> axum::Router<()> {
    let spec = super::openapi_spec();

    let auth_middleware = middleware::from_fn_with_state(state.clone(), auth::middleware);

    speq::axum_router!()
        .route("/", get(|| async move { Html(DOCS_INDEX) }))
        .route("/openapi.json", get(|| async move { Json(spec) }))
        .route_layer(auth_middleware)
        .layer(
            ServiceBuilder::new()
                .layer(Extension(QsQueryConfig::new(5, false)))
                .layer(middleware::from_fn(error_handler))
                .layer(
                    CorsLayer::new()
                        .allow_credentials(true)
                        .allow_origin(AllowOrigin::mirror_request())
                        .allow_methods(AllowMethods::mirror_request())
                        .allow_headers([ACCEPT, CONTENT_TYPE]),
                ),
        )
        .with_state(state)
}

async fn error_handler(req: Request<Body>, next: Next) -> impl IntoResponse {
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
