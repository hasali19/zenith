mod common;
mod events;
mod files;
mod import;
mod metadata;
mod movies;
mod progress;
mod subtitles;
mod transcoder;
mod tv;
mod videos;

use actix_http::http::header;
use actix_web::dev::HttpServiceFactory;
use actix_web::middleware::DefaultHeaders;
use actix_web::{web, HttpResponse};

pub type ApiResult<T = HttpResponse> = actix_web::Result<T>;

pub fn service(path: &str) -> impl HttpServiceFactory {
    let default_headers = DefaultHeaders::new()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::CACHE_CONTROL, "no-cache");

    web::scope(path)
        .wrap(default_headers)
        .configure(movies::configure)
        .configure(tv::configure)
        .configure(videos::configure)
        .configure(subtitles::configure)
        .configure(metadata::configure)
        .configure(progress::configure)
        .configure(transcoder::configure)
        .configure(files::configure)
        .configure(import::configure)
        .configure(events::configure)
        .default_service(web::route().to(HttpResponse::NotFound))
}
