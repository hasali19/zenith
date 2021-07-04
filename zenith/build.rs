use std::fs::File;
use std::io::Write;
use std::path::Path;

use syn::{Item, Lit, Meta};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src");

    let mut acc = vec![];

    find_handlers("src", &mut acc);

    let out_path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("handlers.rs");
    let mut file = File::create(out_path)?;

    writeln!(
        file,
        "pub fn configure(config: &mut actix_web::web::ServiceConfig) {{"
    )?;

    writeln!(file, "    config")?;

    for (method, path, _, mod_path) in acc {
        let method = match method {
            Method::Get => "get",
            Method::Post => "post",
        };

        writeln!(
            file,
            "        .route(\"{}\", actix_web::web::{}().to({}))",
            path.trim_start_matches("/api"),
            method,
            mod_path
        )?;
    }

    writeln!(file, ";}}")?;

    Ok(())
}

enum Method {
    Get,
    Post,
}

impl Method {
    fn parse(val: &str) -> Option<Method> {
        Some(match val {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => return None,
        })
    }
}

fn find_handlers(path: impl AsRef<Path>, acc: &mut Vec<(Method, String, Vec<String>, String)>) {
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if entry.file_type().unwrap().is_dir() {
            find_handlers(file_path, acc);
            continue;
        }

        let content = std::fs::read_to_string(&file_path).unwrap();
        let ast = syn::parse_file(&content).unwrap();

        for item in ast.items {
            if let Item::Fn(item) = item {
                let mut docs = item.attrs.iter().filter_map(|attr| {
                    let meta = match attr.parse_meta().ok()? {
                        Meta::NameValue(v) => v,
                        _ => return None,
                    };

                    if !matches!(meta.path.get_ident(), Some(v) if *v == "doc") {
                        return None;
                    }

                    let content = match meta.lit {
                        Lit::Str(v) => v.value(),
                        _ => return None,
                    };

                    Some(content)
                });

                let first_doc = match docs.next() {
                    Some(doc) => doc,
                    None => continue,
                };

                println!("{:#?}", first_doc);

                let (method, path) = match first_doc.trim().split_once(' ') {
                    Some(v) => v,
                    None => continue,
                };

                let method = match Method::parse(method) {
                    Some(v) => v,
                    None => continue,
                };

                let docs: Vec<_> = docs.collect();

                let rel_path = file_path
                    .strip_prefix("src")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('/', "::");

                let mod_path = rel_path
                    .trim_end_matches("::mod.rs")
                    .trim_end_matches(".rs");

                let item_path = format!("crate::{}::{}", mod_path, item.sig.ident);

                acc.push((method, path.to_owned(), docs, item_path));
            }
        }
    }
}
