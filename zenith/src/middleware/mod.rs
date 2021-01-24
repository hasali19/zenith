use std::time::Instant;

use async_trait::async_trait;

use zenith_server::Middleware;

pub struct Logger;

#[async_trait]
impl<S: Send + Sync + 'static> Middleware<S> for Logger {
    async fn handle(
        &self,
        state: S,
        req: zenith_server::Request,
        next: zenith_server::Next<'_, S>,
    ) -> zenith_server::Response {
        let method = req.method().clone();
        let path = req.uri().path();
        let qs = req.uri().query();

        let path = match qs {
            Some(qs) => format!("{}?{}", path, qs),
            None => path.to_string(),
        };

        let start = Instant::now();
        let res = next.run(state, req).await;
        let duration = start.elapsed();
        let status = res.status();

        log::info!("{} {} -> {} - {:?}", method, path, status, duration);

        res
    }
}
