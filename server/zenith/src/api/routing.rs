use axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use axum::http::{HeaderValue, Method};
use axum::routing::MethodRouter;
use axum::Router;
use axum_codegen::RequestHandler;
use tower_http::set_header::SetResponseHeaderLayer;

axum_codegen::routes!();

pub fn router() -> axum::Router {
    axum_routes()
        .iter()
        .fold(Router::new(), |router, (path, method, handler)| {
            router.route(path, method_service(method.clone(), *handler))
        })
        .layer(SetResponseHeaderLayer::<_, ()>::overriding(
            ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ))
}

fn method_service(method: Method, handler: RequestHandler) -> MethodRouter {
    match method {
        Method::GET => axum::routing::get(handler),
        Method::POST => axum::routing::post(handler),
        Method::PUT => axum::routing::put(handler),
        Method::DELETE => axum::routing::delete(handler),
        Method::HEAD => axum::routing::head(handler),
        Method::OPTIONS => axum::routing::options(handler),
        Method::PATCH => axum::routing::patch(handler),
        Method::TRACE => axum::routing::trace(handler),
        _ => panic!("Unsupported method: {}", method),
    }
}
