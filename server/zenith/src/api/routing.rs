use std::collections::BTreeMap;

use axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use axum::http::{HeaderValue, Method};
use axum::response::Html;
use axum::routing::{get, MethodRouter};
use axum::{Json, Router};
use axum_codegen::{ParamLocation, Route};
use markdown::{Block, Span};
use okapi::openapi3::{
    Info, MediaType, OpenApi, Operation, Parameter, ParameterValue, RefOr, RequestBody,
};
use schemars::gen::{SchemaGenerator, SchemaSettings};
use tower_http::set_header::SetResponseHeaderLayer;

const REDOC_INDEX: &str = include_str!("redoc.html");

pub fn router() -> axum::Router {
    let spec = openapi_spec();

    axum_codegen::routes()
        .fold(Router::new(), |router, route| {
            router.route(route.path(), method_service(route))
        })
        .route("/", get(|| async move { Html(REDOC_INDEX) }))
        .route("/openapi.json", get(|| async move { Json(spec) }))
        .layer(SetResponseHeaderLayer::overriding(
            ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ))
}

pub fn openapi_spec() -> OpenApi {
    build_openapi_spec(&mut SchemaSettings::openapi3().into_generator())
}

fn build_openapi_spec(schema_gen: &mut SchemaGenerator) -> OpenApi {
    let mut spec = OpenApi {
        openapi: "3.0.0".to_owned(),
        info: Info {
            title: "Zenith Media Server API".to_owned(),
            version: "1.0.0".to_owned(),
            ..Default::default()
        },
        ..Default::default()
    };

    spec.paths = axum_codegen::routes().fold(Default::default(), |mut paths, route| {
        let method = route.method();
        let path = route
            .path()
            .trim_start_matches('/')
            .split('/')
            .map(|segment| {
                if segment.starts_with(':') {
                    format!("{{{}}}", segment.trim_start_matches(':'))
                } else {
                    segment.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("/");

        let item = paths.entry(format!("/api/{path}")).or_default();
        let operation_ref = match method {
            Method::GET => &mut item.get,
            Method::POST => &mut item.post,
            Method::PUT => &mut item.put,
            Method::PATCH => &mut item.patch,
            Method::DELETE => &mut item.delete,
            _ => panic!("invalid method: {method}"),
        };

        *operation_ref = build_route_spec(route, schema_gen).map(|mut op| {
            for param in &mut op.parameters {
                if let RefOr::Object(param) = param {
                    param.extensions.clear();
                }
            }

            op.responses.extensions.clear();

            op
        });

        paths
    });

    let mut components = spec.components.unwrap_or_default();

    for (name, schema) in schema_gen.take_definitions() {
        components
            .schemas
            .insert(name.clone(), schema.into_object());
    }

    spec.components = Some(components);

    spec
}

fn build_route_spec(
    route: &'static dyn Route,
    schema_gen: &mut SchemaGenerator,
) -> Option<Operation> {
    let doc = route.doc()?;
    let md = markdown::tokenize(doc);
    let mut blocks = md.into_iter();

    let summary = blocks
        .by_ref()
        .filter_map(|block| {
            let spans = match block {
                Block::Paragraph(spans) => spans,
                _ => return None,
            };

            match spans.into_iter().next() {
                Some(Span::Text(text)) => Some(text),
                _ => None,
            }
        })
        .next();

    let mut operation = Operation::default();

    if operation.summary.is_none() {
        operation.summary = summary;
    }

    if operation.tags.is_empty() {
        if let Some(file_stem) = std::path::Path::new(route.src_file()).file_stem() {
            operation.tags = vec![file_stem.to_str().unwrap().to_owned()]
        }
    }

    for param in route.params(schema_gen) {
        let (location, required) = match param.location {
            ParamLocation::Path => ("path", true),
            ParamLocation::Query => ("query", false),
        };

        operation.parameters.push(RefOr::Object(Parameter {
            location: location.to_owned(),
            name: param.name,
            required,
            allow_empty_value: false,
            deprecated: false,
            description: None,
            extensions: Default::default(),
            value: ParameterValue::Schema {
                allow_reserved: false,
                example: None,
                examples: None,
                explode: None,
                style: None,
                schema: param.schema.into_object(),
            },
        }));
    }

    operation.request_body = route.request(schema_gen).map(|req| {
        let mut content = BTreeMap::new();

        content.insert(
            "application/json".to_owned(),
            MediaType {
                schema: Some(req.schema.into_object()),
                ..Default::default()
            },
        );

        RefOr::Object(RequestBody {
            content,
            ..Default::default()
        })
    });

    for res in route.responses(schema_gen) {
        let status = res.status;
        let mut content = BTreeMap::new();

        if let Some(schema) = res.schema {
            content.insert(
                "application/json".to_owned(),
                MediaType {
                    schema: Some(schema.into_object()),
                    ..Default::default()
                },
            );
        }

        let res = okapi::openapi3::Response {
            content,
            description: res
                .description
                .or_else(|| status.canonical_reason().map(|reason| reason.to_owned()))
                .unwrap_or_else(|| res.status.to_string()),
            ..Default::default()
        };

        operation
            .responses
            .responses
            .insert(status.as_str().to_owned(), RefOr::Object(res));
    }

    operation.responses.extensions.clear();

    Some(operation)
}

fn method_service(route: &'static dyn Route) -> MethodRouter {
    let method = route.method();
    let handler = |parts, body| route.handle(parts, body);
    match method {
        Method::GET => axum::routing::get(handler),
        Method::POST => axum::routing::post(handler),
        Method::PUT => axum::routing::put(handler),
        Method::DELETE => axum::routing::delete(handler),
        Method::HEAD => axum::routing::head(handler),
        Method::OPTIONS => axum::routing::options(handler),
        Method::PATCH => axum::routing::patch(handler),
        Method::TRACE => axum::routing::trace(handler),
        _ => panic!("Unsupported method: {method}"),
    }
}
