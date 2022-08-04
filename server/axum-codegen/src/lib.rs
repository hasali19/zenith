pub mod reflection;

use axum::http::{Method, StatusCode};
pub use axum_codegen_macros::*;

use reflection::{Type, TypeContext};

pub enum ParamLocation {
    Path,
    Query,
}

pub struct ParamSpec {
    pub location: ParamLocation,
    pub name: String,
    pub required: bool,
    pub type_desc: Type,
}

pub struct RequestSpec {
    pub type_desc: Type,
}

#[derive(Debug)]
pub struct ResponseSpec {
    pub status: StatusCode,
    pub description: Option<String>,
    pub type_desc: Option<Type>,
}

pub trait Route: Send + Sync {
    fn name(&self) -> &'static str;
    fn path(&self) -> &'static str;
    fn method(&self) -> Method;

    fn src_file(&self) -> &'static str;

    fn doc(&self) -> Option<&'static str> {
        None
    }

    fn params(&self, cx: &mut TypeContext) -> Vec<ParamSpec>;
    fn request(&self, cx: &mut TypeContext) -> Option<RequestSpec>;
    fn responses(&self, cx: &mut TypeContext) -> Vec<ResponseSpec>;

    fn register(&self, router: axum::Router) -> axum::Router;
}

pub mod inventory {
    pub use inventory::submit;
}

#[macro_export]
macro_rules! submit {
    ($e:expr) => {
        axum_codegen::inventory::submit! {
            $e as &'static dyn axum_codegen::Route
        }
    };
}

::inventory::collect!(&'static dyn Route);

pub fn routes() -> impl Iterator<Item = &'static dyn Route> {
    ::inventory::iter::<&'static dyn Route>.into_iter().copied()
}

pub fn query_params_from_reflected_type<T: reflection::Reflect>(
    cx: &mut TypeContext,
) -> impl Iterator<Item = ParamSpec> + '_ {
    let struct_type = match T::reflect(cx).as_id().map(|id| cx.get(id).unwrap()) {
        Some(reflection::TypeDecl::Struct(inner)) => inner,
        _ => panic!("query model type must be a struct or map"),
    };

    struct_type.fields.iter().map(|field| ParamSpec {
        location: ParamLocation::Query,
        name: field.name.clone(),
        required: !field.has_default,
        type_desc: field.type_desc.clone(),
    })
}
