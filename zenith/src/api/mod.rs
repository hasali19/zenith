mod common;
mod events;
mod ext;
mod files;
mod import;
mod metadata;
mod movies;
mod progress;
mod subtitles;
mod transcoder;
mod tv;
mod videos;

pub mod error;

use atium::headers::AccessControlAllowOrigin;
use atium::query::QueryError;
use atium::respond::RespondRequestExt;
use atium::router::{ParamError, Router};
use atium::{async_trait, endpoint, Handler, Request, StatusCode};
use serde_json::json;

use crate::api::error::ErrorResponse;

use self::error::bad_request;

pub fn handler() -> impl Handler {
    let router = Router::new()
        .with(movies::routes)
        .with(tv::routes)
        .with(import::routes)
        .with(videos::routes)
        .with(subtitles::routes)
        .with(files::routes)
        .with(progress::routes)
        .with(metadata::routes)
        .with(transcoder::routes)
        .with(events::routes);

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

struct ErrorHandler;

#[async_trait]
impl Handler for ErrorHandler {
    async fn run(&self, req: atium::Request, next: &dyn atium::Next) -> atium::Request {
        let mut req = next.run(req).await;

        if let Some(mut e) = req.take_ext::<eyre::Report>() {
            let res: ErrorResponse = if let Some(e) = e.downcast_mut::<ErrorResponse>() {
                std::mem::take(e).into()
            } else if let Some(e) = e.downcast_ref::<ParamError>() {
                bad_request(e.to_string()).into()
            } else if let Some(e) = e.downcast_ref::<QueryError>() {
                bad_request(e.to_string()).into()
            } else {
                ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into()
            };

            req.set_res(res);
        }

        req
    }
}

#[endpoint]
async fn not_found(req: &mut Request) -> eyre::Result<()> {
    req.respond(StatusCode::NOT_FOUND)
        .json(&json!({"error": "invalid request path"}))?;
    Ok(())
}
