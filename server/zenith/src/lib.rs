#![feature(let_chains)]

mod ext;
mod password_utils;

pub mod api;
pub mod config;
pub mod library;
pub mod metadata;
pub mod subtitles;
pub mod transcoder;
pub mod util;
pub mod utils;
pub mod video_prober;

use std::marker::PhantomData;

use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
pub use db::media::MediaItemType;
pub use db::Db;
use serde_qs::axum::QsQuery;
use speq::reflection::{Reflect, TypeContext};
use speq::{QuerySpec, RequestSpec, RouteSpec};

#[derive(Clone)]
pub struct App {
    pub key: Key,
}

impl FromRef<App> for Key {
    fn from_ref(input: &App) -> Self {
        input.key.clone()
    }
}

speq::axum_config!(App);

pub trait DefaultRouteFnParam {
    fn process_spec(&self, spec: RouteSpec, cx: &mut TypeContext) -> RouteSpec;
}

impl<T> DefaultRouteFnParam for &T {
    fn process_spec(&self, spec: RouteSpec, _cx: &mut TypeContext) -> RouteSpec {
        spec
    }
}

pub trait ActualRouteFnParam {
    fn process_spec(&self, spec: RouteSpec, cx: &mut TypeContext) -> RouteSpec;
}

impl<T: RouteFnParam> ActualRouteFnParam for PhantomData<T> {
    fn process_spec(&self, spec: RouteSpec, cx: &mut TypeContext) -> RouteSpec {
        <T as RouteFnParam>::process_spec(spec, cx)
    }
}

pub trait RouteFnParam {
    fn process_spec(spec: RouteSpec, cx: &mut TypeContext) -> RouteSpec;
}

impl<T: Reflect> RouteFnParam for axum::extract::Path<T> {
    fn process_spec(mut spec: RouteSpec, cx: &mut TypeContext) -> RouteSpec {
        spec.path.params = Some(T::reflect(cx));
        spec
    }
}

impl<T: Reflect> RouteFnParam for axum::extract::Json<T> {
    fn process_spec(mut spec: RouteSpec, cx: &mut TypeContext) -> RouteSpec {
        spec.request = Some(RequestSpec {
            type_desc: T::reflect(cx),
        });
        spec
    }
}

impl<T: Reflect> RouteFnParam for QsQuery<T> {
    fn process_spec(mut spec: RouteSpec, cx: &mut TypeContext) -> RouteSpec {
        spec.query = Some(QuerySpec {
            type_desc: T::reflect(cx),
        });
        spec
    }
}
