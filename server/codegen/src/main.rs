#![feature(let_chains)]

use std::collections::HashSet;
use std::time::Instant;

use quote::{format_ident, quote};
use syn::{Expr, ExprLit, FnArg, Lit, Type};
use walkdir::WalkDir;

struct Route {
    name: String,
    path: String,
    method: String,
    src_file: String,
    fn_path: syn::Path,
    param_count: usize,
    path_param_index: Option<usize>,
    query_param_index: Option<usize>,
    request_body_param_index: Option<usize>,
}

fn main() -> eyre::Result<()> {
    let start = Instant::now();

    let mut tokens = quote! {
        use std::borrow::Cow;

        use axum::Router;
        use axum::http::Method;
        use axum::routing::*;
        use speq::{QuerySpec, RequestSpec};
        use speq::reflection::{Reflect, Type, TypeContext};
    };

    let mut routes = vec![];

    for entry in WalkDir::new("server/zenith/src") {
        let Ok(entry) = entry else {
            continue;
        };

        if !entry.file_type().is_file() {
            continue;
        }

        if entry.path().extension().and_then(|it| it.to_str()) != Some("rs") {
            continue;
        }

        let file = syn::parse_file(&std::fs::read_to_string(entry.path())?)?;
        let mod_path = entry
            .path()
            .strip_prefix("server/zenith/src")?
            .with_extension("")
            .to_str()
            .unwrap()
            .replace(['/', '\\'], "::");

        for item in file.items {
            if let syn::Item::Fn(item) = item {
                let fn_name = item.sig.ident.to_string();
                let mut doc = vec![];

                for attr in item.attrs {
                    if let Ok(attr) = attr.meta.require_name_value() {
                        if attr.path.is_ident("doc") {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(str), ..
                            }) = &attr.value
                            {
                                let line = str.value();
                                let line = line.strip_prefix(' ').unwrap_or(&line);
                                doc.push(line.to_owned());
                            }
                        }
                    }
                }

                if let Some((method, path)) = doc.first().and_then(|l| l.split_once(' ')) {
                    let method = method.to_lowercase();
                    let path = path.to_owned();
                    let fn_path = format!("crate::{mod_path}::{fn_name}");

                    let mut path_param_index = None;
                    let mut query_param_index = None;
                    let mut request_body_param_index = None;

                    for (i, arg) in item.sig.inputs.iter().enumerate() {
                        let FnArg::Typed(arg) = arg else {
                            continue;
                        };

                        let Type::Path(ty) = &*arg.ty else {
                            continue;
                        };

                        let Some(segment) = ty.path.segments.last() else {
                            continue;
                        };

                        if segment.ident == "Path" {
                            path_param_index = Some(i);
                        } else if segment.ident == "Query" || segment.ident == "QsQuery" {
                            query_param_index = Some(i);
                        } else if segment.ident == "Json" {
                            request_body_param_index = Some(i);
                        }
                    }

                    routes.push(Route {
                        name: fn_name,
                        path,
                        method,
                        src_file: entry.path().to_str().unwrap().to_owned(),
                        fn_path: syn::parse_str(&fn_path)?,
                        param_count: item.sig.inputs.len(),
                        path_param_index,
                        query_param_index,
                        request_body_param_index,
                    });
                }
            }
        }
    }

    let mut parameter_type_fns = HashSet::new();

    let route_calls = routes.iter().map(|route| {
        let path = &route.path;
        let method = format_ident!("{}", route.method);
        let fn_path = &route.fn_path;
        quote! {
            .route(#path, #method(#fn_path))
        }
    });

    tokens.extend(quote! {
        pub fn router() -> Router<crate::App> {
            Router::new() #(#route_calls)*
        }
    });

    let route_specs = routes.iter().map(|route| {
        let name = &route.name;
        let path = &route.path;
        let method = format_ident!("{}", route.method.to_uppercase());
        let src_file = &route.src_file;
        let fn_path = &route.fn_path;

        let params = if let Some(index) = route.path_param_index {
            let param_count = route.param_count;
            let parameter_type_fn = format_ident!("parameter_type_{param_count}_{index}");

            parameter_type_fns.insert((param_count, index));

            quote! {
                Some(#parameter_type_fn(#fn_path, cx))
            }
        } else {
            quote! { None }
        };

        let query = if let Some(index) = route.query_param_index {
            let param_count = route.param_count;
            let parameter_type_fn = format_ident!("parameter_type_{param_count}_{index}");

            parameter_type_fns.insert((param_count, index));

            quote! {
                Some(QuerySpec {
                    type_desc: #parameter_type_fn(#fn_path, cx),
                })
            }
        } else {
            quote! { None }
        };

        let request = if let Some(index) = route.request_body_param_index {
            let param_count = route.param_count;
            let parameter_type_fn = format_ident!("parameter_type_{param_count}_{index}");

            parameter_type_fns.insert((param_count, index));

            quote! {
                Some(RequestSpec {
                    type_desc: #parameter_type_fn(#fn_path, cx),
                })
            }
        } else {
            quote! { None }
        };

        quote! {
            speq::RouteSpec {
                name: Cow::Borrowed(#name),
                path: speq::PathSpec {
                    value: Cow::Borrowed(#path),
                    params: #params,
                },
                method: Method::#method,
                src_file: Cow::Borrowed(#src_file),
                doc: None,
                query: #query,
                request: #request,
                responses: vec![],
            }
        }
    });

    tokens.extend(quote! {
        pub fn route_specs(cx: &mut TypeContext) -> Vec<speq::RouteSpec> {
            vec![#(#route_specs),*]
        }
    });

    for (params, i) in parameter_type_fns {
        let fn_name = format_ident!("parameter_type_{params}_{i}");
        let target_type_name = format_ident!("T{i}");

        let type_params = (0..params).map(|i| format_ident!("T{i}"));
        let target_fn_params = (0..params).map(|i| format_ident!("T{i}"));

        tokens.extend(quote! {
            fn #fn_name<#(#type_params),*, R>(
                _: fn(#(#target_fn_params),*) -> R,
                cx: &mut TypeContext,
            ) -> Type
            where
                #target_type_name: Reflect
            {
                <#target_type_name as Reflect>::reflect(cx)
            }
        });
    }

    let tokens = prettyplease::unparse(&syn::parse2(tokens)?);

    println!("{tokens}");

    let elapsed = Instant::now() - start;
    eprintln!("completed in {}ms", elapsed.as_millis());

    Ok(())
}
