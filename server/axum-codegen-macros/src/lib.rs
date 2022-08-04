use std::fmt::Write;

use proc_macro::TokenStream;
use quote::quote;
use structmeta::StructMeta;
use syn::{AttributeArgs, DeriveInput, Lit, LitInt, LitStr, Meta};
use syn_mid::FnArg;

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
    #[struct_meta(unnamed)]
    model: Vec<syn::Path>,
}

#[derive(StructMeta)]
struct QueryArgs {
    #[struct_meta(unnamed)]
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
            let model = args.model;

            let param_specs = path
                .split('/')
                .filter(|it| it.starts_with(':'))
                .enumerate()
                .map(|(i, name)| {
                    let name = name.trim_start_matches(':');
                    let model = model
                        .get(i)
                        .expect("number of path parameters must match path string");
                    quote! {
                        params.push(axum_codegen::ParamSpec {
                            location: axum_codegen::ParamLocation::Path,
                            name: #name.to_owned(),
                            type_desc: <#model as axum_codegen::reflection::Reflect>::reflect(cx),
                        });
                    }
                });

            params.extend(param_specs);
        } else if attr.path.is_ident("query") {
            let args = attr.parse_args::<QueryArgs>().unwrap();
            let model = args.model;

            let param_spec = quote! {
                params.extend(axum_codegen::query_params_from_reflected_type::<#model>(cx));
            };

            params.push(param_spec);
        } else if attr.path.is_ident("request") {
            let args = attr.parse_args::<RequestArgs>().unwrap();
            let model = args.model;

            request = quote! {
                Some(
                    axum_codegen::RequestSpec {
                        type_desc: <#model as axum_codegen::reflection::Reflect>::reflect(cx),
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

            let type_desc = match args.model {
                None => {
                    quote! { None }
                }
                Some(model) => {
                    quote! { Some(<#model as axum_codegen::reflection::Reflect>::reflect(cx)) }
                }
            };

            let response_spec = quote! {
                axum_codegen::ResponseSpec {
                    status: axum::http::StatusCode::from_u16(#status).unwrap(),
                    description: #description,
                    type_desc: #type_desc,
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

    let query_param = input.sig.inputs.iter().find_map(|input| match input {
        FnArg::Receiver(_) => None,
        FnArg::Typed(param) => param
            .attrs
            .iter()
            .find(|it| it.path.is_ident("query"))
            .map(|attr| (param, attr)),
    });

    if let Some((param, _)) = query_param {
        let model = &param.ty;

        let param_spec = quote! {
            params.extend(axum_codegen::query_params_from_reflected_type::<#model>(cx));
        };

        params.push(param_spec);
    }

    for input in input.sig.inputs.iter_mut() {
        if let FnArg::Typed(param) = input {
            param.attrs.retain(|attr| !attr.path.is_ident("query"));
        }
    }

    TokenStream::from(quote! {
        #input

        const _: () = {
            struct Route;

            impl axum_codegen::Route for Route {
                fn name(&self) -> &'static str {
                    stringify!(#name)
                }

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

                fn params(&self, cx: &mut axum_codegen::reflection::TypeContext) -> Vec<axum_codegen::ParamSpec> {
                    let mut params = vec![];
                    #(#params)*
                    params
                }

                fn request(&self, cx: &mut axum_codegen::reflection::TypeContext) -> Option<axum_codegen::RequestSpec> {
                    #request
                }

                fn responses(&self, cx: &mut axum_codegen::reflection::TypeContext) -> Vec<axum_codegen::ResponseSpec> {
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

            axum_codegen::submit!(&Route);
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

#[proc_macro_derive(Reflect, attributes(serde))]
pub fn derive_reflect(input: TokenStream) -> TokenStream {
    use serde_derive_internals::{ast as serde_ast, Derive};
    let input = syn::parse_macro_input!(input as DeriveInput);
    let cx = serde_derive_internals::Ctxt::new();
    let container = serde_ast::Container::from_ast(&cx, &input, Derive::Serialize).unwrap();
    cx.check().unwrap();

    let ident = container.ident;
    let expr = match container.data {
        serde_ast::Data::Enum(variants) => {
            let tag = match container.attrs.tag() {
                serde_derive_internals::attr::TagType::External => todo!("external tag"),
                serde_derive_internals::attr::TagType::Internal { tag } => tag,
                serde_derive_internals::attr::TagType::Adjacent { .. } => todo!("adjacent tag"),
                serde_derive_internals::attr::TagType::None => todo!("untagged"),
            };

            let variants = variants.into_iter().map(|variant| {
                let name = variant.ident.to_string();
                let serialize_name = variant.attrs.name().serialize_name();
                let kind = match variant.style {
                    serde_ast::Style::Struct => {
                        let fields = variant.fields.into_iter().map(build_field);
                        quote! {
                            EnumVariantKind::Struct(vec![#(#fields),*])
                        }
                    }
                    serde_ast::Style::Tuple => todo!("tuple enum"),
                    serde_ast::Style::Newtype => {
                        let ty = variant.fields[0].ty;
                        quote! {
                            EnumVariantKind::NewType(<#ty as Reflect>::reflect(cx))
                        }
                    }
                    serde_ast::Style::Unit => quote! {
                        EnumVariantKind::Unit
                    },
                };
                quote! {
                    EnumVariant {
                        name: #name.to_owned(),
                        tag_value: #serialize_name.to_owned(),
                        kind: #kind,
                    }
                }
            });

            quote! {
                TypeDecl::Enum(EnumType {
                    name: stringify!(#ident).to_string(),
                    tag: Some(EnumTag::Internal(#tag.to_owned())),
                    variants: vec![#(#variants),*],
                })
            }
        }
        serde_ast::Data::Struct(style, fields) => match style {
            serde_ast::Style::Struct => {
                let fields = fields.into_iter().map(build_field);
                quote! {
                    TypeDecl::Struct(StructType {
                        name: stringify!(#ident).to_string(),
                        fields: vec![#(#fields),*],
                    })
                }
            }
            serde_ast::Style::Tuple => todo!(),
            serde_ast::Style::Newtype => todo!(),
            serde_ast::Style::Unit => todo!(),
        },
    };

    TokenStream::from(quote! {
        impl axum_codegen::reflection::Reflect for #ident {
            fn type_id() -> Option<std::borrow::Cow<'static, str>> {
                Some(concat!(module_path!(), "::", stringify!(#ident)).into())
            }

            fn reflect(cx: &mut axum_codegen::reflection::TypeContext) -> axum_codegen::reflection::Type {
                use axum_codegen::reflection::*;
                let id = Self::type_id().unwrap();
                cx.insert_with(id.clone(), |cx| #expr);
                Type::Id(id)
            }
        }
    })
}

fn build_field(field: serde_derive_internals::ast::Field) -> proc_macro2::TokenStream {
    let name = field.attrs.name().serialize_name();
    let ty = field.ty;
    let flatten = field.attrs.flatten();
    quote! {
        Field {
            name: #name.to_owned(),
            flatten: #flatten,
            type_desc: <#ty as Reflect>::reflect(cx),
        }
    }
}
