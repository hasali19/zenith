use std::any::type_name;
use std::future::{ready, Ready};
use std::ops::Deref;

use actix_web::error::ErrorInternalServerError;
use actix_web::FromRequest;

mod ext;

pub mod api;
pub mod config;
pub mod db;
pub mod ffprobe;
pub mod library;
pub mod metadata;
pub mod subtitles;
pub mod tmdb;
pub mod transcoder;
pub mod util;
pub mod utils;

#[derive(Clone)]
pub struct Ext<T>(pub T);

impl<T> Deref for Ext<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone + 'static> FromRequest for Ext<T> {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(
            req.app_data()
                .cloned()
                .map(|val: T| Ext(val))
                .ok_or_else(|| {
                    let type_name = type_name::<Self>();
                    let message = format!("failed to extract {} from request", type_name);
                    ErrorInternalServerError(message)
                }),
        )
    }
}
