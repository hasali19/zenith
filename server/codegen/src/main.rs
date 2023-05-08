#![feature(let_chains)]

use std::time::Instant;

use syn::{Expr, ExprLit, Lit};
use walkdir::WalkDir;

struct Route {
    name: String,
    path: String,
    method: String,
    src_file: String,
}

fn main() -> eyre::Result<()> {
    let start = Instant::now();

    println!("use crate::App;");
    println!();
    println!("pub fn router() -> axum::Router<App> {{");
    println!("    axum::Router::new()");

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

                    println!("       .route(\"{path}\", axum::routing::{method}(crate::{mod_path}::{fn_name}))");

                    routes.push(Route {
                        name: fn_name,
                        path,
                        method,
                        src_file: entry.path().to_str().unwrap().to_owned(),
                    });
                }
            }
        }
    }

    println!("}}");
    println!();
    println!(
        "pub fn route_specs(cx: &mut speq::reflection::TypeContext) -> Vec<speq::RouteSpec> {{"
    );
    println!("    vec![");

    for route in routes {
        let name = route.name;
        let path = route.path;
        let method = route.method.to_uppercase();
        let src_file = route.src_file;

        println!(
            "        speq::RouteSpec {{
            name: std::borrow::Cow::Borrowed(\"{name}\"),
            path: std::borrow::Cow::Borrowed(\"{path}\"),
            method: axum::http::Method::{method},
            src_file: std::borrow::Cow::Borrowed(\"{src_file}\"),
            doc: None,
            params: vec![],
            query: None,
            request: None,
            responses: vec![],
        }},"
        );
    }

    println!("    ]");
    println!("}}");

    let elapsed = Instant::now() - start;
    eprintln!("completed in {}ms", elapsed.as_millis());

    Ok(())
}
