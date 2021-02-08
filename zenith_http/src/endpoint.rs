use async_trait::async_trait;

use super::{Request, Response};

#[async_trait]
pub trait Endpoint<S>: Send + Sync + 'static {
    async fn call(&self, state: S, req: Request) -> Response;
}

#[async_trait]
impl<S, F, Fut, Res> Endpoint<S> for F
where
    S: Send + Sync + 'static,
    F: Send + Sync + 'static + Fn(S, Request) -> Fut,
    Fut: std::future::Future<Output = Res> + Send + 'static,
    Res: Into<Response> + 'static,
{
    async fn call(&self, state: S, req: Request) -> Response {
        (self)(state, req).await.into()
    }
}
