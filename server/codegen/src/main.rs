#![feature(let_chains)]

use std::time::Instant;

use syn::{Expr, ExprLit, Lit};
use walkdir::WalkDir;

fn main() -> eyre::Result<()> {
    let start = Instant::now();

    println!("use crate::App;");
    println!();
    println!("pub fn router() -> axum::Router<App> {{");
    println!("    axum::Router::new()");

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
                    println!("       .route(\"{path}\", axum::routing::{method}(crate::{mod_path}::{fn_name}))");
                }
            }
        }
    }

    println!("}}");

    let elapsed = Instant::now() - start;
    eprintln!("completed in {}ms", elapsed.as_millis());

    Ok(())
}
