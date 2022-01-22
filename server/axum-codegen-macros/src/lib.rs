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

    let vis = input.vis.clone();
    let name = input.sig.ident.clone();

    let args = input.sig.inputs.iter().map(|_| {
        quote! {
            match axum::extract::FromRequest::from_request(&mut parts).await {
                Ok(v) => v,
                Err(e) => return axum::response::IntoResponse::into_response(e).map(axum::body::boxed),
            }
        }
    });

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

    let mod_path = std::env::var("AXUM_CODEGEN_MODULE").unwrap_or_else(|_| "crate".to_owned());
    let mod_path: syn::Path =
        syn::parse_str(&mod_path).unwrap_or_else(|_| panic!("invalid module path: {}", mod_path));

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

    let route_impl = quote! {
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

            fn handle(&self, req: axum::http::request::Parts, body: axum::extract::RawBody) -> futures::future::BoxFuture<'static, axum::http::Response<axum::body::BoxBody>> {
                self::#name(req, body)
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
        }
    };

    TokenStream::from(quote! {
        #vis fn #name(req: axum::http::request::Parts, axum::extract::RawBody(body): axum::extract::RawBody) -> futures::future::BoxFuture<'static, axum::http::Response<axum::body::BoxBody>> {
            #input
            #route_impl

            #[linkme::distributed_slice(#mod_path::AXUM_ROUTES)]
            static _route: &dyn axum_codegen::Route = &Route;

            Box::pin(async move {
                let req = axum::http::Request::from_parts(req, body);
                let mut parts = axum::extract::RequestParts::new(req);
                let res = #name(#(#args),*).await;
                axum::response::IntoResponse::into_response(res).map(axum::body::boxed)
            })
        }
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

#[proc_macro]
pub fn routes(_: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        #[linkme::distributed_slice]
        pub static AXUM_ROUTES: [&'static dyn axum_codegen::Route] = [..];

        pub fn axum_routes() -> &'static [&'static dyn axum_codegen::Route] {
            &AXUM_ROUTES
        }
    })
}
