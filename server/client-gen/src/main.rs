extern crate zenith;

use std::collections::HashMap;
use std::fmt::Write;

use lazy_regex::regex;
use speq::reflection::{EnumTag, EnumVariantKind, PrimitiveType, Type, TypeDecl};

pub fn main() {
    let mut fn_decls = HashMap::new();

    println!("import qs from \"query-string\";");
    println!();

    let spec = speq::spec();

    for route in spec.routes {
        if ["get_events"].contains(&route.name.as_ref()) {
            continue;
        }

        let mut decl = format!("async {}(\n", route.name);

        for param in route.params {
            decl += "      ";
            decl += &param.name;
            decl += ": ";
            decl += &gen_ts_type(&param.type_desc);
            decl += ",\n";
        }

        let mut has_params = false;
        if let Some(query) = route.query {
            decl += "      params: ";
            decl += &gen_ts_type(&query.type_desc);
            decl += ",\n";
            has_params = true;
        }

        decl += "    )";

        let success_response = route
            .responses
            .into_iter()
            .find(|res| res.status.is_success());

        if let Some(type_desc) = success_response.and_then(|res| res.type_desc) {
            decl += ": Promise<";
            decl += &gen_ts_type(&type_desc);
            decl += ">";
        }

        let path = regex!(":([^/]+)").replace_all(&route.path, "$${$1}");

        writeln!(decl, " {{").unwrap();
        writeln!(decl, "      const url = new URL(`/api{path}`);").unwrap();

        if has_params {
            writeln!(
                decl,
                "      url.search = \"?\" + qs.stringify(params, {{ arrayFormat: \"bracket\" }});"
            )
            .unwrap();
        }

        writeln!(decl, "      const res = await fetch(url);").unwrap();
        writeln!(decl, "      return await res.json();").unwrap();
        write!(decl, "    }}").unwrap();

        let file_name = std::path::Path::new(route.src_file.as_ref())
            .file_stem()
            .and_then(|it| it.to_str())
            .unwrap()
            .to_owned();

        fn_decls
            .entry(file_name)
            .or_insert_with(Vec::new)
            .push(decl);
    }

    println!("export default {{");

    for (group_name, decls) in fn_decls {
        println!("  {group_name}: {{");

        for decl in decls {
            println!("    {decl},");
        }

        println!("  }},");
    }

    println!("}};");
    println!();

    for (_, type_decl) in spec.types {
        println!("{}", gen_ts_type_decl(type_decl));
        println!();
    }
}

fn gen_ts_type(type_desc: &Type) -> String {
    match type_desc {
        Type::Primitive(primitive) => match primitive {
            PrimitiveType::Bool => "boolean".to_owned(),
            PrimitiveType::Int(_) => "number".to_owned(),
            PrimitiveType::UInt(_) => "number".to_owned(),
            PrimitiveType::Float(_) => "number".to_owned(),
            PrimitiveType::String => "string".to_owned(),
        },
        Type::Option(inner) => format!("({} | null)", gen_ts_type(flatten_option(inner))),
        Type::Array(inner) => format!("{}[]", gen_ts_type(inner)),
        Type::Tuple(_) => todo!(),
        Type::Map(_) => todo!(),
        Type::Id(id) => id
            .split("::")
            .last()
            .expect("id must not be empty")
            .to_owned(),
    }
}

fn flatten_option(type_desc: &Type) -> &Type {
    if let Type::Option(inner) = type_desc {
        flatten_option(inner)
    } else {
        type_desc
    }
}

fn gen_ts_type_decl(type_decl: TypeDecl) -> String {
    match type_decl {
        TypeDecl::Struct(struct_type) => {
            let mut body = "{\n".to_owned();
            let mut flatten = vec![];

            for field in struct_type.fields {
                if field.flatten {
                    flatten.push(gen_ts_type(&field.type_desc));
                } else {
                    let sep = if field.required { ":" } else { "?:" };
                    writeln!(
                        body,
                        "  {}{sep} {};",
                        field.name,
                        gen_ts_type(&field.type_desc)
                    )
                    .unwrap();
                }
            }

            body += "}";

            if flatten.is_empty() {
                format!("interface {} {}", struct_type.name, body)
            } else {
                format!(
                    "type {} = {} & {};",
                    struct_type.name,
                    body,
                    flatten.join(" & ")
                )
            }
        }
        TypeDecl::Enum(enum_type) => {
            let tag = match enum_type.tag {
                None => todo!(),
                Some(tag) => match tag {
                    EnumTag::External => todo!(),
                    EnumTag::Internal(tag) => tag,
                    EnumTag::Adjacent { .. } => todo!(),
                },
            };

            let mut variants = vec![];
            for variant in enum_type.variants {
                let variant = match variant.kind {
                    EnumVariantKind::Unit => {
                        format!("{{\n  {tag}: \"{}\";\n}}", variant.tag_value)
                    }
                    EnumVariantKind::NewType(inner) => {
                        format!(
                            "({{ {tag}: \"{}\" }} & {})",
                            variant.tag_value,
                            gen_ts_type(&inner)
                        )
                    }
                    EnumVariantKind::Struct(fields) => {
                        let mut body = format!("{{\n  {tag}: \"{}\";\n", variant.tag_value);
                        let mut flatten = vec![];

                        for field in fields {
                            if field.flatten {
                                flatten.push(gen_ts_type(&field.type_desc));
                            } else {
                                let sep = if field.required { ":" } else { "?:" };
                                writeln!(
                                    body,
                                    "  {}{sep} {};",
                                    field.name,
                                    gen_ts_type(&field.type_desc)
                                )
                                .unwrap();
                            }
                        }

                        body += "}";

                        if flatten.is_empty() {
                            body
                        } else {
                            flatten.insert(0, body);
                            format!("({})", flatten.join("&"))
                        }
                    }
                };

                variants.push(variant);
            }

            format!("type {} = {};", enum_type.name, variants.join(" | "))
        }
    }
}
