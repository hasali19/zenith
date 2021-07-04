mod events;
mod handlers;
mod import;
mod metadata;
mod movies;
mod progress;
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
        .configure(handlers::configure)
        .default_service(web::route().to(HttpResponse::NotFound))
}
