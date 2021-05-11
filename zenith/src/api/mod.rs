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
        .service(movies::service("/movies"))
        .service(tv::service("/tv"))
        .service(videos::service("/videos"))
        .service(metadata::service("/metadata"))
        .service(progress::service("/progress"))
        .service(transcoder::service("/transcoder"))
        .service(events::service("/events"))
}
