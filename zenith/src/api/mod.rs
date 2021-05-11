mod events;
mod import;
mod metadata;
mod movies;
mod progress;
mod transcoder;
mod tv;
mod videos;

use actix_web::{web, HttpResponse, Scope};

pub type ApiResult<T = HttpResponse> = actix_web::Result<T>;

pub fn service(path: &str) -> Scope {
    web::scope(path)
        .configure(movies::configure)
        .configure(tv::configure)
        .configure(videos::configure)
        .configure(metadata::configure)
        .configure(progress::configure)
        .configure(transcoder::configure)
        .configure(events::configure)
        .default_service(web::route().to(HttpResponse::NotFound))
}
