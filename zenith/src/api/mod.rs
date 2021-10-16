mod events;
mod ext;
mod files;
mod import;
mod items;
mod metadata;
mod movies;
mod progress;
mod subtitles;
mod transcoder;
mod tv;
mod videos;

mod error;
mod scanner;

use atium::headers::AccessControlAllowOrigin;
use atium::router::Router;
use atium::{async_trait, endpoint, Handler, Request};

use crate::api::error::ErrorHandler;

pub fn handler() -> impl Handler {
    let router = Router::new()
        .with(items::routes)
        .with(movies::routes)
        .with(tv::routes)
        .with(import::routes)
        .with(videos::routes)
        .with(subtitles::routes)
        .with(files::routes)
        .with(progress::routes)
        .with(metadata::routes)
        .with(transcoder::routes)
        .with(events::routes)
        .with(scanner::routes);

    atium::compose!(DefaultHeaders, ErrorHandler, router, not_found)
}

struct DefaultHeaders;

#[async_trait]
impl Handler for DefaultHeaders {
    async fn run(&self, req: atium::Request, next: &dyn atium::Next) -> atium::Request {
        let mut req = next.run(req).await;

        if let Some(res) = req.res_mut() {
            res.set_header(AccessControlAllowOrigin::ANY);
        }

        req
    }
}

#[endpoint]
async fn not_found(req: &mut Request) -> eyre::Result<()> {
    req.set_res(error::not_found("invalid request path"));
    Ok(())
}
