use axum::http::Method;
use axum_codegen::openapiv3::*;
use axum_codegen::SchemaGenerator;
use axum_codegen::{indexmap, openapiv3, ParamLocation, Route};
use markdown::{Block, Span};

pub fn openapi_spec() -> OpenAPI {
    build_openapi_spec(SchemaGenerator::new())
}

fn build_openapi_spec(mut schema_gen: SchemaGenerator) -> OpenAPI {
    let mut spec = OpenAPI {
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

        let item = match paths
            .paths
            .entry(format!("/api/{path}"))
            .or_insert_with(|| ReferenceOr::Item(Default::default()))
        {
            ReferenceOr::Reference { .. } => unreachable!(),
            ReferenceOr::Item(item) => item,
        };

        let operation_ref = match method {
            Method::GET => &mut item.get,
            Method::POST => &mut item.post,
            Method::PUT => &mut item.put,
            Method::PATCH => &mut item.patch,
            Method::DELETE => &mut item.delete,
            _ => panic!("invalid method: {method}"),
        };

        *operation_ref = build_route_spec(route, &mut schema_gen);

        paths
    });

    let mut components = spec.components.unwrap_or_default();

    for (name, schema) in schema_gen.into_schemas() {
        components
            .schemas
            .insert(name.clone(), ReferenceOr::Item(schema));
    }

    components.schemas.sort_keys();

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
        let parameter_data = ParameterData {
            name: param.name,
            required: matches!(param.location, ParamLocation::Path),
            deprecated: None,
            description: None,
            example: None,
            examples: Default::default(),
            explode: None,
            extensions: Default::default(),
            format: ParameterSchemaOrContent::Schema(param.schema),
        };

        let param = match param.location {
            ParamLocation::Path => Parameter::Path {
                parameter_data,
                style: PathStyle::Simple,
            },
            ParamLocation::Query => Parameter::Query {
                parameter_data,
                allow_reserved: false,
                style: QueryStyle::Form,
                allow_empty_value: None,
            },
        };

        operation.parameters.push(ReferenceOr::Item(param));
    }

    operation.request_body = route.request(schema_gen).map(|req| {
        let content = indexmap! {
            "application/json".to_owned() => MediaType {
                schema: Some(req.schema),
                ..Default::default()
            },
        };

        ReferenceOr::Item(RequestBody {
            content,
            ..Default::default()
        })
    });

    for res in route.responses(schema_gen) {
        let status = res.status;
        let mut content = indexmap! {};

        if let Some(schema) = res.schema {
            content.insert(
                "application/json".to_owned(),
                MediaType {
                    schema: Some(schema),
                    ..Default::default()
                },
            );
        }

        let res = openapiv3::Response {
            content,
            description: res
                .description
                .or_else(|| status.canonical_reason().map(|reason| reason.to_owned()))
                .unwrap_or_else(|| res.status.to_string()),
            ..Default::default()
        };

        operation.responses.responses.insert(
            openapiv3::StatusCode::Code(status.as_u16()),
            ReferenceOr::Item(res),
        );
    }

    operation.responses.extensions.clear();

    Some(operation)
}
