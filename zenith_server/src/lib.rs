mod endpoint;
mod middleware;
mod request;
mod response;

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{service, Server as HyperServer};
use route_recognizer::{Match, Router};

pub use hyper::{Body, Method, StatusCode};

pub use self::endpoint::Endpoint;
pub use self::middleware::{Middleware, Next};
pub use self::request::Request;
pub use self::response::Response;

pub struct App<S> {
    state: S,
    router: HashMap<Method, Router<Box<dyn Endpoint<S>>>>,
    middleware: Vec<Box<dyn Middleware<S>>>,
}

macro_rules! method_fn {
    ($name:ident, $method:ident) => {
        pub fn $name(&mut self, path: &str, to: impl Endpoint<S>) {
            self.route(Method::$method, path, to);
        }
    };
}

impl<S: Clone + Send + Sync + 'static> App<S> {
    pub fn new(state: S) -> Self {
        App {
            state,
            router: HashMap::new(),
            middleware: vec![],
        }
    }

    pub fn configure(&mut self, f: impl Fn(&mut Self)) {
        f(self);
    }

    pub fn route(&mut self, method: Method, path: &str, to: impl Endpoint<S>) {
        self.router
            .entry(method)
            .or_insert_with(Router::new)
            .add(path, Box::new(to));
    }

    method_fn!(connect, CONNECT);
    method_fn!(delete, DELETE);
    method_fn!(get, GET);
    method_fn!(head, HEAD);
    method_fn!(options, OPTIONS);
    method_fn!(patch, PATCH);
    method_fn!(post, POST);
    method_fn!(put, PUT);
    method_fn!(trace, TRACE);

    pub fn wrap(&mut self, middleware: impl Middleware<S>) {
        self.middleware.push(Box::new(middleware));
    }

    pub async fn run(self, addr: SocketAddr) -> hyper::Result<()> {
        let app = Arc::new(self);

        let make_svc = service::make_service_fn(|_| {
            let app = app.clone();
            async { Ok::<_, Infallible>(service::service_fn(move |req| service(app.clone(), req))) }
        });

        log::info!("starting server at http://{}:{}", addr.ip(), addr.port());

        let server = HyperServer::bind(&addr).serve(make_svc);
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install ctrl+c handler");
        };

        tokio::select! {
            _ = server => {},
            _ = ctrl_c => {},
        };

        log::info!("shutting down");

        Ok(())
    }
}

async fn service<S: Clone + Send + Sync + 'static>(
    app: Arc<App<S>>,
    req: hyper::Request<Body>,
) -> Result<hyper::Response<Body>, hyper::http::Error> {
    let method = req.method();
    let path = req.uri().path();

    let route = app
        .router
        .get(method)
        .and_then(|r| r.recognize(path).ok())
        .map(|Match { handler, params }| (handler, params));

    let (handler, params) = match route {
        Some(route) => route,
        None => {
            return hyper::Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
        }
    };

    let req = Request::new(req, params);
    let res = Next::new(handler.as_ref(), &app.middleware)
        .run(app.state.clone(), req)
        .await;

    Ok(res.into())
}
