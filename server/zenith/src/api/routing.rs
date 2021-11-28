use axum::http::Method;
use axum::routing::MethodRouter;
use axum::Router;
use axum_codegen::RequestHandler;

axum_codegen::routes!();

pub fn router() -> axum::Router {
    axum_routes()
        .iter()
        .fold(Router::new(), |router, (path, method, handler)| {
            router.route(path, method_service(method.clone(), *handler))
        })
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
