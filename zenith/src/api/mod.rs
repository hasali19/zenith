mod events;
mod metadata;
mod movies;
mod progress;
mod transcoder;
mod tv;
mod videos;

use actix_web::{web, Scope};

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
