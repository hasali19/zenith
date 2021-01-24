use async_trait::async_trait;

use super::{Endpoint, Request, Response};

#[async_trait]
pub trait Middleware<S>: Send + Sync + 'static {
    async fn handle(&self, state: S, req: Request, next: Next<'_, S>) -> Response;
}

pub struct Next<'a, S> {
    endpoint: &'a dyn Endpoint<S>,
    next_middleware: &'a [Box<dyn Middleware<S>>],
}

impl<'a, S: Send + Sync + 'static> Next<'a, S> {
    pub fn new(endpoint: &'a dyn Endpoint<S>, next: &'a [Box<dyn Middleware<S>>]) -> Next<'a, S> {
        Next {
            endpoint,
            next_middleware: next,
        }
    }

    pub async fn run(mut self, state: S, req: Request) -> Response {
        if let Some((current, next)) = self.next_middleware.split_first() {
            self.next_middleware = next;
            current.handle(state, req, self).await
        } else {
            self.endpoint.call(state, req).await
        }
    }
}
