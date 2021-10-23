mod error;
mod ext;
mod files;
mod import;
mod items;
mod metadata;
mod movies;
mod progress;
mod scanner;
mod subtitles;
mod transcoder;
mod tv;
mod videos;

use std::future::Future;

use actix_web::body::AnyBody;
use actix_web::dev::{HttpServiceFactory, Service, ServiceResponse};
use actix_web::http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE};
use actix_web::http::HeaderValue;
use actix_web::middleware::{DefaultHeaders, NormalizePath, TrailingSlash};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse};
use serde_json::json;

use self::error::{not_found, ApiError};

pub type ApiResult<T> = Result<T, ApiError>;

pub fn service() -> impl HttpServiceFactory {
    web::scope("/api")
        .wrap(DefaultHeaders::new().header(ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .wrap(NormalizePath::new(TrailingSlash::Trim))
        .wrap_fn(|req, srv| error_handler(srv.call(req)))
        .configure(configure_services)
        .default_service(web::route().to(handle_not_found))
}

fn configure_services(config: &mut ServiceConfig) {
    config
        .service(items::get_items)
        .service(items::get_item)
        .service(items::update_user_data)
        .service(movies::get_movies)
        .service(movies::get_recent_movies)
        .service(movies::get_movie)
        .service(tv::get_shows)
        .service(tv::get_recently_updated_shows)
        .service(tv::get_show)
        .service(tv::get_seasons)
        .service(tv::get_season)
        .service(tv::get_episodes)
        .service(tv::get_episode)
        .service(videos::get_video_content)
        .service(subtitles::get_subtitle)
        .service(subtitles::delete_subtitle)
        .service(import::get_import_queue)
        .service(import::import_subtitle)
        .service(import::import_movie)
        .service(import::import_show)
        .service(import::import_episode)
        .service(scanner::start_scan)
        .service(scanner::scan_item)
        .service(transcoder::get_state)
        .service(transcoder::get_events)
        .service(transcoder::transcode)
        .service(progress::update_progress)
        .service(metadata::refresh_metadata)
        .service(files::get_files);
}

async fn error_handler(
    res: impl Future<Output = actix_web::Result<ServiceResponse>>,
) -> actix_web::Result<ServiceResponse> {
    match res.await {
        Err(e) => Err(e),
        Ok(res) => {
            let err = match res.response().error() {
                Some(err) => err,
                None => return Ok(res),
            };

            let content_type = HeaderValue::from_str("application/json").unwrap();
            let message = err.to_string();

            match serde_json::to_vec(&json!({ "message": message })) {
                Err(e) => {
                    tracing::error!("{}", e);
                    Ok(res)
                }
                Ok(v) => Ok(res.map_body(|head, _| {
                    head.headers_mut().insert(CONTENT_TYPE, content_type);
                    AnyBody::from(v)
                })),
            }
        }
    }
}

async fn handle_not_found() -> ApiResult<HttpResponse> {
    Err(not_found("invalid request path/method"))
}
