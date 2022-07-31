use std::fmt::Write;

use proc_macro::TokenStream;
use quote::quote;
use structmeta::StructMeta;
use syn::{AttributeArgs, Lit, LitInt, LitStr, Meta};

enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Patch,
    Trace,
}

#[derive(StructMeta)]
struct PathArgs {
    name: LitStr,
    model: syn::Path,
}

#[derive(StructMeta)]
struct QueryArgs {
    name: LitStr,
    model: syn::Path,
}

#[derive(StructMeta)]
struct RequestArgs {
    model: Option<syn::Path>,
}

#[derive(StructMeta)]
struct ResponseArgs {
    status: Option<LitInt>,
    description: Option<LitStr>,
    model: Option<syn::Path>,
}

fn route(method: Method, args: TokenStream, mut item: TokenStream) -> TokenStream {
    let mut input: syn_mid::ItemFn = match syn::parse(item.clone()) {
        Ok(input) => input,
        Err(e) => {
            item.extend(TokenStream::from(e.into_compile_error()));
            return item;
        }
    };

    let args = syn::parse_macro_input!(args as AttributeArgs);
    let path = match args.first().unwrap() {
        syn::NestedMeta::Meta(_) => panic!(),
        syn::NestedMeta::Lit(lit) => match lit {
            syn::Lit::Str(path) => path.value(),
            _ => {
                item.extend(TokenStream::from(
                    quote! {compile_error("Invalid path in macro arguments")},
                ));
                return item;
            }
        },
    };

    let name = input.sig.ident.clone();

    let method = match method {
        Method::Get => quote! { axum::http::Method::GET },
        Method::Post => quote! { axum::http::Method::POST },
        Method::Put => quote! { axum::http::Method::PUT },
        Method::Delete => quote! { axum::http::Method::DELETE },
        Method::Head => quote! { axum::http::Method::HEAD },
        Method::Options => quote! { axum::http::Method::OPTIONS },
        Method::Connect => quote! { axum::http::Method::CONNECT },
        Method::Patch => quote! { axum::http::Method::PATCH },
        Method::Trace => quote! { axum::http::Method::TRACE },
    };

    let mut doc = String::new();
    let mut params = vec![];
    let mut request = quote! { None };
    let mut responses = vec![];

    for attr in &input.attrs {
        if attr.path.is_ident("doc") {
            let meta = attr.parse_meta().unwrap();
            let meta = match meta {
                Meta::NameValue(val) => val,
                _ => unreachable!(),
            };

            let val = match meta.lit {
                Lit::Str(str) => str.value(),
                _ => unreachable!(),
            };

            writeln!(doc, "{}", val).unwrap();
        } else if attr.path.is_ident("path") {
            let args = attr.parse_args::<PathArgs>().unwrap();
            let name = args.name;
            let model = args.model;

            let param_spec = quote! {
                axum_codegen::ParamSpec {
                    location: axum_codegen::ParamLocation::Path,
                    name: #name.to_owned(),
                    schema: schema_gen.subschema_for::<#model>(),
                }
            };

            params.push(param_spec);
        } else if attr.path.is_ident("query") {
            let args = attr.parse_args::<QueryArgs>().unwrap();
            let name = args.name;
            let model = args.model;

            let param_spec = quote! {
                axum_codegen::ParamSpec {
                    location: axum_codegen::ParamLocation::Query,
                    name: #name.to_owned(),
                    schema: schema_gen.subschema_for::<#model>(),
                }
            };

            params.push(param_spec);
        } else if attr.path.is_ident("request") {
            let args = attr.parse_args::<RequestArgs>().unwrap();
            let model = args.model;

            request = quote! {
                Some(
                    axum_codegen::RequestSpec {
                        schema: schema_gen.subschema_for::<#model>(),
                    }
                )
            };
        } else if attr.path.is_ident("response") {
            let args = attr.parse_args::<ResponseArgs>().unwrap();
            let status = args
                .status
                .map(|v| v.base10_parse().unwrap())
                .unwrap_or(200u16);

            let description = match args.description {
                None => quote! { None },
                Some(description) => {
                    quote! { Some(#description.to_owned()) }
                }
            };

            let schema = match args.model {
                None => {
                    quote! { None }
                }
                Some(model) => {
                    quote! { Some(schema_gen.subschema_for::<#model>()) }
                }
            };

            let response_spec = quote! {
                axum_codegen::ResponseSpec {
                    status: axum::http::StatusCode::from_u16(#status).unwrap(),
                    description: #description,
                    schema: #schema,
                }
            };

            responses.push(response_spec);
        }
    }

    input.attrs.retain(|attr| {
        ["path", "query", "request", "response"]
            .iter()
            .all(|ident| !attr.path.is_ident(ident))
    });

    TokenStream::from(quote! {
        #input

        const _: () = {
            struct Route;

            impl axum_codegen::Route for Route {
                fn path(&self) -> &'static str {
                    #path
                }

                fn method(&self) -> axum::http::Method {
                    #method
                }

                fn src_file(&self) -> &'static str {
                    file!()
                }

                fn doc(&self) -> Option<&'static str> {
                    Some(#doc)
                }

                fn params(&self, schema_gen: &mut okapi::schemars::gen::SchemaGenerator) -> Vec<axum_codegen::ParamSpec> {
                    vec![#(#params),*]
                }

                fn request(&self, schema_gen: &mut okapi::schemars::gen::SchemaGenerator) -> Option<axum_codegen::RequestSpec> {
                    #request
                }

                fn responses(&self, schema_gen: &mut okapi::schemars::gen::SchemaGenerator) -> Vec<axum_codegen::ResponseSpec> {
                    vec![#(#responses),*]
                }

                fn register(&self, router: axum::Router) -> axum::Router {
                    router.route(self.path(), match self.method() {
                        axum::http::Method::GET => axum::routing::get(#name),
                        axum::http::Method::POST => axum::routing::post(#name),
                        axum::http::Method::PUT => axum::routing::put(#name),
                        axum::http::Method::DELETE => axum::routing::delete(#name),
                        axum::http::Method::HEAD => axum::routing::head(#name),
                        axum::http::Method::OPTIONS => axum::routing::options(#name),
                        axum::http::Method::PATCH => axum::routing::patch(#name),
                        axum::http::Method::TRACE => axum::routing::trace(#name),
                        method => panic!("Unsupported method: {}", method),
                    })
                }
            }

            inventory::submit! {
                &Route as &'static dyn axum_codegen::Route
            }
        };
    })
}

macro_rules! method_attr {
    ($name:ident, $method:ident) => {
        #[proc_macro_attribute]
        pub fn $name(args: TokenStream, item: TokenStream) -> TokenStream {
            route(Method::$method, args, item)
        }
    };
}

method_attr!(get, Get);
method_attr!(post, Post);
method_attr!(put, Put);
method_attr!(delete, Delete);
method_attr!(head, Head);
method_attr!(options, Options);
method_attr!(connect, Connect);
method_attr!(patch, Patch);
method_attr!(trace, Trace);
