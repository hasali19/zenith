#![feature(let_chains)]

use std::collections::HashSet;
use std::time::Instant;

use quote::{format_ident, quote};
use syn::{Expr, ExprLit, Lit};
use walkdir::WalkDir;

struct Route {
    name: String,
    path: String,
    method: String,
    src_file: String,
    fn_path: syn::Path,
    param_count: usize,
}

fn main() -> eyre::Result<()> {
    let start = Instant::now();

    let mut tokens = quote! {
        use std::borrow::Cow;

        use axum::Router;
        use axum::http::Method;
        use axum::routing::*;
        use speq::reflection::{TypeContext};

        use crate::{DefaultRouteFnParam, ActualRouteFnParam};
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

                    routes.push(Route {
                        name: fn_name,
                        path,
                        method,
                        src_file: entry.path().to_str().unwrap().to_owned(),
                        fn_path: syn::parse_str(&fn_path)?,
                        param_count: item.sig.inputs.len(),
                    });
                }
            }
        }
    }

    let mut arg_fns = HashSet::new();

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

        let stmts = (0..route.param_count).map(|i| {
            let arg_fn = format_ident!("arg_{}_{i}", route.param_count);
            arg_fns.insert((route.param_count, i));
            quote! {
                spec = (&#arg_fn(f)).process_spec(spec, cx);
            }
        });

        quote! {
            {
                use #fn_path as f;
                let mut spec = default_spec(#name, #path, Method::#method, #src_file);
                #(#stmts)*
                spec
            }
        }
    });

    let count = routes.len();

    tokens.extend(quote! {
        pub fn route_specs(cx: &mut TypeContext) -> Vec<speq::RouteSpec> {
            fn default_spec(
                name: &'static str,
                path: &'static str,
                method: Method,
                src_file: &'static str,
            ) -> speq::RouteSpec {
                speq::RouteSpec {
                    name: Cow::Borrowed(name),
                    path: speq::PathSpec {
                        value: Cow::Borrowed(path),
                        params: None,
                    },
                    method,
                    src_file: Cow::Borrowed(src_file),
                    doc: None,
                    query: None,
                    request: None,
                    responses: vec![],
                }
            }

            let mut routes = Vec::with_capacity(#count);

            #(routes.push(#route_specs);)*

            routes
        }
    });

    for (params, i) in arg_fns {
        let fn_name = format_ident!("arg_{params}_{i}");
        let target_type_name = format_ident!("T{i}");

        let type_params = (0..params)
            .map(|i| format_ident!("T{i}"))
            .collect::<Vec<_>>();

        let target_fn_params = (0..params)
            .map(|i| format_ident!("T{i}"))
            .collect::<Vec<_>>();

        tokens.extend(quote! {
            fn #fn_name<#(#type_params),*, R>(
                _: fn(#(#target_fn_params),*) -> R,
            ) -> std::marker::PhantomData<#target_type_name> {
                std::marker::PhantomData
            }
        });
    }

    let tokens = prettyplease::unparse(&syn::parse2(tokens)?);

    println!("{tokens}");

    let elapsed = Instant::now() - start;
    eprintln!("completed in {}ms", elapsed.as_millis());

    Ok(())
}
