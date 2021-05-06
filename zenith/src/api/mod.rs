mod metadata;
mod movies;
mod progress;
mod stream;
mod tv;

use actix_web::{web, Scope};

pub fn service(path: &str) -> Scope {
    web::scope(path)
        .service(movies::service("/movies"))
        .service(tv::service("/tv"))
        .service(stream::service("/stream"))
        .service(metadata::service("/metadata"))
        .service(progress::service("/progress"))
}
