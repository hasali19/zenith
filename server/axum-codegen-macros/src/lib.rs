use proc_macro::TokenStream;
use quote::quote;
use syn::AttributeArgs;

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

fn route(method: Method, args: TokenStream, mut item: TokenStream) -> TokenStream {
    let input: syn::ItemFn = match syn::parse(item.clone()) {
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

    TokenStream::from(quote! {
        #vis fn #name(req: axum::http::request::Parts, axum::extract::RawBody(body): axum::extract::RawBody) -> futures::future::BoxFuture<'static, axum::http::Response<axum::body::BoxBody>> {
            #input
            #[linkme::distributed_slice(#mod_path::AXUM_ROUTES)]
            static _handler: (&'static str, axum::http::Method, axum_codegen::RequestHandler) = (#path, #method, self::#name);
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
        pub static AXUM_ROUTES: [(&'static str, axum::http::Method, axum_codegen::RequestHandler)] = [..];

        pub fn axum_routes() -> &'static [(&'static str, axum::http::Method, axum_codegen::RequestHandler)] {
            &AXUM_ROUTES
        }
    })
}
