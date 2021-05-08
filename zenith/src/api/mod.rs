mod events;
mod metadata;
mod movies;
mod progress;
mod stream;
mod transcoder;
mod tv;
mod videos;

use actix_web::{web, Scope};

pub fn service(path: &str) -> Scope {
    web::scope(path)
        .service(movies::service("/movies"))
        .service(tv::service("/tv"))
        .service(stream::service("/stream"))
        .service(videos::service("/videos"))
        .service(metadata::service("/metadata"))
        .service(progress::service("/progress"))
        .service(transcoder::service("/transcoder"))
        .service(events::service("/events"))
}
