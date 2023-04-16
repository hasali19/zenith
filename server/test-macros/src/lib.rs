use proc_macro::TokenStream;
use quote::quote;
use structmeta::StructMeta;
use syn::ItemFn;

#[derive(StructMeta)]
struct TestAttr {
    #[struct_meta(unnamed)]
    runner: Option<syn::Ident>,
}

#[proc_macro_attribute]
pub fn test(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr = syn::parse::<TestAttr>(attr).unwrap();
    let item = syn::parse_macro_input!(input as ItemFn);
    let ident = item.sig.ident.clone();

    if let Some(runner) = attr.runner {
        TokenStream::from(quote! {
            #item

            const _: () = {
                fn __runner() -> futures::future::LocalBoxFuture<'static, ()> {
                    Box::pin(#runner(#ident))
                }

                inventory::submit!(crate::TestCase(concat!(module_path!(), "::", stringify!(#ident)), __runner));
            };
        })
    } else {
        TokenStream::from(quote! {
            #item

            const _: () = {
                fn __runner() -> futures::future::LocalBoxFuture<'static, ()> {
                    Box::pin(#ident())
                }

                inventory::submit!(crate::TestCase(concat!(module_path!(), "::", stringify!(#ident)), __runner));
            };
        })
    }
}
