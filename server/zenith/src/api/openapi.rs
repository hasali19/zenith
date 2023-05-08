use std::borrow::Cow;
use std::collections::HashMap;

use axum::http::Method;
use camino::Utf8Path;
use indexmap::indexmap;
use markdown::{Block, Span};
use openapiv3::*;
use speq::reflection::{
    EnumTag, EnumVariantKind, Field, FloatWidth, PrimitiveType, Type, TypeContext, TypeDecl,
};
use speq::{ApiSpec, RouteSpec};

use super::routing;

pub fn openapi_spec() -> OpenAPI {
    let mut tcx = TypeContext::new();
    let routes = routing::routes::route_specs(&mut tcx);
    let spec = ApiSpec {
        routes,
        types: tcx.into_types(),
    };

    let mut openapi = OpenAPI {
        openapi: "3.0.0".to_owned(),
        info: Info {
            title: "Zenith Media Server API".to_owned(),
            version: "1.0.0".to_owned(),
            ..Default::default()
        },
        ..Default::default()
    };

    openapi.paths = spec
        .routes
        .into_iter()
        .fold(Default::default(), |mut paths, route| {
            let path = route
                .path
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

            let operation_ref = match route.method {
                Method::GET => &mut item.get,
                Method::POST => &mut item.post,
                Method::PUT => &mut item.put,
                Method::PATCH => &mut item.patch,
                Method::DELETE => &mut item.delete,
                method => panic!("invalid method: {method}"),
            };

            *operation_ref = build_route_spec(route, &spec.types);

            paths
        });

    let mut components = openapi.components.unwrap_or_default();

    for (name, schema) in spec.types {
        components
            .schemas
            .insert(name.into_owned(), type_decl_to_schema(schema));
    }

    components.schemas.sort_keys();

    openapi.components = Some(components);

    openapi
}

fn build_route_spec(
    route: RouteSpec,
    types: &HashMap<Cow<'static, str>, TypeDecl>,
) -> Option<Operation> {
    let summary = route
        .doc
        .map(|doc| markdown::tokenize(&doc))
        .unwrap_or_default()
        .into_iter()
        .by_ref()
        .find_map(|block| {
            let spans = match block {
                Block::Paragraph(spans) => spans,
                _ => return None,
            };

            match spans.into_iter().next() {
                Some(Span::Text(text)) => Some(text),
                _ => None,
            }
        });

    let mut operation = Operation::default();

    if operation.summary.is_none() {
        operation.summary = summary;
    }

    if operation.tags.is_empty() {
        if let Some(file_stem) = Utf8Path::new(route.src_file.as_ref()).file_stem() {
            operation.tags = vec![file_stem.to_owned()]
        }
    }

    for param in route.params {
        let parameter_data = ParameterData {
            name: param.name.into_owned(),
            required: true,
            deprecated: None,
            description: None,
            example: None,
            examples: Default::default(),
            explode: None,
            extensions: Default::default(),
            format: ParameterSchemaOrContent::Schema(type_to_schema(&param.type_desc)),
        };

        operation
            .parameters
            .push(ReferenceOr::Item(Parameter::Path {
                parameter_data,
                style: PathStyle::Simple,
            }));
    }

    if let Some(query) = route.query {
        let struct_type = query
            .type_desc
            .as_id()
            .and_then(|id| types.get(id))
            .and_then(|decl| decl.as_struct())
            .unwrap_or_else(|| panic!("unsupported query model type: {:?}", query.type_desc));

        for field in &struct_type.fields {
            // serde_qs requires array parameters to be passed in the form 'key[]=a&key[]=b'. While openapi doesn't have
            // a dedicated option for this, we can make it work by appending [] to the parameter name.
            let mut name = field.name.clone();
            if let Type::Array(_) = &field.type_desc {
                name += "[]";
            }

            let parameter_data = ParameterData {
                name: name.into_owned(),
                required: field.required && !matches!(field.type_desc, Type::Option(_)),
                deprecated: None,
                description: None,
                example: None,
                examples: Default::default(),
                explode: None,
                extensions: Default::default(),
                format: ParameterSchemaOrContent::Schema(type_to_schema(&field.type_desc)),
            };

            operation
                .parameters
                .push(ReferenceOr::Item(Parameter::Query {
                    parameter_data,
                    // serde_qs requires the [] for arrays to be passed unencoded when using strict mode:
                    // https://docs.rs/serde_qs/latest/serde_qs/#strict-vs-non-strict-modes
                    allow_reserved: true,
                    style: QueryStyle::Form,
                    allow_empty_value: None,
                }));
        }
    }

    operation.request_body = route.request.map(|req| {
        let content = indexmap! {
            "application/json".to_owned() => MediaType {
                schema: Some(type_to_schema(&req.type_desc)),
                ..Default::default()
            },
        };

        ReferenceOr::Item(RequestBody {
            content,
            ..Default::default()
        })
    });

    for res in route.responses {
        let status = res.status;
        let mut content = indexmap! {};

        if let Some(schema) = res.type_desc {
            content.insert(
                "application/json".to_owned(),
                MediaType {
                    schema: Some(type_to_schema(&schema)),
                    ..Default::default()
                },
            );
        }

        let res = openapiv3::Response {
            content,
            description: res
                .description
                .map(|it| it.into_owned())
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

fn type_to_schema(type_desc: &Type) -> ReferenceOr<Schema> {
    use openapiv3::Type as SchemaType;
    let schema = match type_desc {
        Type::Primitive(ty) => match ty {
            PrimitiveType::Bool => type_schema(SchemaType::Boolean {}),
            PrimitiveType::Int(width) => type_schema(SchemaType::Integer(IntegerType {
                format: VariantOrUnknownOrEmpty::Unknown(format!("int{}", width.as_u8())),
                ..Default::default()
            })),
            PrimitiveType::UInt(width) => type_schema(SchemaType::Integer(IntegerType {
                format: VariantOrUnknownOrEmpty::Unknown(format!("uint{}", width.as_u8())),
                minimum: Some(0),
                ..Default::default()
            })),
            PrimitiveType::Float(width) => type_schema(SchemaType::Number(NumberType {
                format: VariantOrUnknownOrEmpty::Item(match width {
                    FloatWidth::F32 => NumberFormat::Float,
                    FloatWidth::F64 => NumberFormat::Double,
                }),
                ..Default::default()
            })),
            PrimitiveType::String => type_schema(SchemaType::String(StringType {
                ..Default::default()
            })),
        },
        Type::Option(inner) => {
            let schema = type_to_schema(inner);
            if let ReferenceOr::Item(mut schema) = schema {
                schema.schema_data.nullable = true;
                schema
            } else {
                Schema {
                    schema_kind: SchemaKind::AllOf {
                        all_of: vec![schema],
                    },
                    schema_data: SchemaData {
                        nullable: true,
                        ..Default::default()
                    },
                }
            }
        }
        Type::Array(inner) => type_schema(SchemaType::Array(ArrayType {
            items: Some(box_schema(type_to_schema(inner))),
            min_items: None,
            max_items: None,
            unique_items: false,
        })),
        Type::Map(_) => todo!(),
        Type::Id(id) => {
            return ReferenceOr::Reference {
                reference: format!("#/components/schemas/{id}"),
            }
        }
    };

    ReferenceOr::Item(schema)
}

fn type_decl_to_schema(type_decl: TypeDecl) -> ReferenceOr<Schema> {
    use openapiv3::Type as SchemaType;
    let schema = match type_decl {
        TypeDecl::Struct(ty) => {
            let mut schema = schema_for_fields(ty.fields);
            schema.schema_data.title = Some(ty.name.into_owned());
            schema
        }
        TypeDecl::Enum(ty) => match ty.tag {
            None => todo!(),
            Some(EnumTag::Adjacent { .. }) => todo!(),
            Some(EnumTag::External) => Schema {
                schema_kind: SchemaKind::Type(SchemaType::String(StringType {
                    enumeration: ty
                        .variants
                        .into_iter()
                        .map(|variant| match variant.kind {
                            EnumVariantKind::Unit => Some(variant.tag_value.into_owned()),
                            EnumVariantKind::NewType(_) => todo!(),
                            EnumVariantKind::Struct(_) => todo!(),
                        })
                        .collect(),
                    ..Default::default()
                })),
                schema_data: SchemaData {
                    title: Some(ty.name.into_owned()),
                    ..Default::default()
                },
            },
            Some(EnumTag::Internal(tag_name)) => {
                let mut one_of = vec![];

                for variant in ty.variants {
                    let schema_data = SchemaData {
                        title: Some(variant.name.into_owned()),
                        ..Default::default()
                    };

                    let tag_schema = Schema {
                        schema_kind: SchemaKind::Type(SchemaType::Object(ObjectType {
                            properties: indexmap! {
                                tag_name.clone().into_owned() => ReferenceOr::Item(Box::new(Schema {
                                    schema_kind: SchemaKind::Type(SchemaType::String(StringType {
                                        enumeration: vec![Some(variant.tag_value.into_owned())],
                                        ..Default::default()
                                    })),
                                    schema_data: Default::default(),
                                })),
                            },
                            required: vec![tag_name.clone().into_owned()],
                            ..Default::default()
                        })),
                        schema_data: Default::default(),
                    };

                    let variant_schema = match variant.kind {
                        EnumVariantKind::Unit => Schema {
                            schema_data,
                            ..tag_schema
                        },
                        EnumVariantKind::NewType(ty) => Schema {
                            schema_kind: SchemaKind::AllOf {
                                all_of: vec![ReferenceOr::Item(tag_schema), type_to_schema(&ty)],
                            },
                            schema_data,
                        },
                        EnumVariantKind::Struct(fields) => Schema {
                            schema_kind: SchemaKind::AllOf {
                                all_of: vec![
                                    ReferenceOr::Item(tag_schema),
                                    ReferenceOr::Item(schema_for_fields(fields)),
                                ],
                            },
                            schema_data,
                        },
                    };

                    one_of.push(ReferenceOr::Item(variant_schema));
                }

                Schema {
                    schema_kind: SchemaKind::OneOf { one_of },
                    schema_data: SchemaData {
                        title: Some(ty.name.into_owned()),
                        ..Default::default()
                    },
                }
            }
        },
    };
    ReferenceOr::Item(schema)
}

fn type_schema(ty: openapiv3::Type) -> Schema {
    Schema {
        schema_kind: SchemaKind::Type(ty),
        schema_data: Default::default(),
    }
}

fn box_schema(schema: ReferenceOr<Schema>) -> ReferenceOr<Box<Schema>> {
    match schema {
        ReferenceOr::Reference { reference } => ReferenceOr::Reference { reference },
        ReferenceOr::Item(schema) => ReferenceOr::Item(Box::new(schema)),
    }
}

fn schema_for_fields(fields: Vec<Field>) -> Schema {
    let mut all_of = vec![];
    let mut properties = indexmap! {};
    let mut required = vec![];

    for field in fields {
        let optional = matches!(field.type_desc, Type::Option(_));
        let field_schema = type_to_schema(&field.type_desc);
        if field.flatten {
            all_of.push(field_schema);
        } else {
            if !optional {
                required.push(field.name.clone().into_owned());
            }
            properties.insert(field.name.into_owned(), box_schema(field_schema));
        }
    }

    Schema {
        schema_kind: SchemaKind::Any(AnySchema {
            typ: Some("object".to_owned()),
            properties,
            all_of,
            required,
            ..Default::default()
        }),
        schema_data: Default::default(),
    }
}
