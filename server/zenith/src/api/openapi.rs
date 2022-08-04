use axum::http::Method;
use axum_codegen::reflection::{
    EnumTag, EnumVariantKind, Field, FloatWidth, PrimitiveType, Type, TypeContext, TypeDecl,
};
use axum_codegen::{ParamLocation, Route};
use indexmap::indexmap;
use markdown::{Block, Span};
use openapiv3::*;

pub fn openapi_spec() -> OpenAPI {
    build_openapi_spec(TypeContext::new())
}

fn build_openapi_spec(mut cx: TypeContext) -> OpenAPI {
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

        let operation_ref = match route.method() {
            Method::GET => &mut item.get,
            Method::POST => &mut item.post,
            Method::PUT => &mut item.put,
            Method::PATCH => &mut item.patch,
            Method::DELETE => &mut item.delete,
            method => panic!("invalid method: {method}"),
        };

        *operation_ref = build_route_spec(route, &mut cx);

        paths
    });

    let mut components = spec.components.unwrap_or_default();

    for (name, schema) in cx.into_types() {
        components
            .schemas
            .insert(name.into_owned(), type_decl_to_schema(schema));
    }

    components.schemas.sort_keys();

    spec.components = Some(components);

    spec
}

fn build_route_spec(route: &'static dyn Route, cx: &mut TypeContext) -> Option<Operation> {
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

    for param in route.params(cx) {
        // serde_qs requires array parameters to be passed in the form 'key[]=a&key[]=b'. While
        // openapi doesn't have a dedicated option for this, we can make it work by appending []
        // to the parameter name.
        let mut name = param.name;
        if matches!(param.location, ParamLocation::Query) {
            if let Type::Array(_) = &param.type_desc {
                name += "[]";
            }
        }

        let parameter_data = ParameterData {
            name,
            required: param.required && !matches!(param.type_desc, Type::Option(_)),
            deprecated: None,
            description: None,
            example: None,
            examples: Default::default(),
            explode: None,
            extensions: Default::default(),
            format: ParameterSchemaOrContent::Schema(type_to_schema(param.type_desc)),
        };

        let param = match param.location {
            ParamLocation::Path => Parameter::Path {
                parameter_data,
                style: PathStyle::Simple,
            },
            ParamLocation::Query => Parameter::Query {
                parameter_data,
                // serde_qs requires the [] for arrays to be passed unencoded when using strict
                // mode: https://docs.rs/serde_qs/latest/serde_qs/#strict-vs-non-strict-modes
                allow_reserved: true,
                style: QueryStyle::Form,
                allow_empty_value: None,
            },
        };

        operation.parameters.push(ReferenceOr::Item(param));
    }

    operation.request_body = route.request(cx).map(|req| {
        let content = indexmap! {
            "application/json".to_owned() => MediaType {
                schema: Some(type_to_schema(req.type_desc)),
                ..Default::default()
            },
        };

        ReferenceOr::Item(RequestBody {
            content,
            ..Default::default()
        })
    });

    for res in route.responses(cx) {
        let status = res.status;
        let mut content = indexmap! {};

        if let Some(schema) = res.type_desc {
            content.insert(
                "application/json".to_owned(),
                MediaType {
                    schema: Some(type_to_schema(schema)),
                    ..Default::default()
                },
            );
        }

        let res = openapiv3::Response {
            content,
            description: res
                .description
                .to_owned()
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

fn type_to_schema(type_desc: Type) -> ReferenceOr<Schema> {
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
            let schema = type_to_schema(*inner);
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
            items: Some(box_schema(type_to_schema(*inner))),
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
            schema.schema_data.title = Some(ty.name);
            schema
        }
        TypeDecl::Enum(ty) => match ty.tag {
            None => todo!(),
            Some(EnumTag::Internal(tag_name)) => {
                let mut one_of = vec![];

                for variant in ty.variants {
                    let schema_data = SchemaData {
                        title: Some(variant.name.to_owned()),
                        ..Default::default()
                    };

                    let tag_schema = Schema {
                        schema_kind: SchemaKind::Type(SchemaType::Object(ObjectType {
                            properties: indexmap! {
                                tag_name.to_owned() => ReferenceOr::Item(Box::new(Schema {
                                    schema_kind: SchemaKind::Type(SchemaType::String(StringType {
                                        enumeration: vec![Some(variant.tag_value.to_owned())],
                                        ..Default::default()
                                    })),
                                    schema_data: Default::default(),
                                })),
                            },
                            required: vec![tag_name.to_owned()],
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
                                all_of: vec![ReferenceOr::Item(tag_schema), type_to_schema(ty)],
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
                        title: Some(ty.name),
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
        let field_schema = type_to_schema(field.type_desc);
        if field.flatten {
            all_of.push(field_schema);
        } else {
            properties.insert(field.name.to_owned(), box_schema(field_schema));
            if !optional {
                required.push(field.name.to_owned());
            }
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
