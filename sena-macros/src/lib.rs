use proc_macro::{Span, TokenStream};

const CRATE_NAME: &str = "sena";

pub(crate) fn crate_ident() -> syn::Ident {
    syn::Ident::new(CRATE_NAME, Span::mixed_site().into())
}

/// Turns your struct into something that acts like `HList`
#[proc_macro_derive(HList, attributes(tail))]
pub fn derive_hlist(item: TokenStream) -> TokenStream {
    impls::hlist::derive(item)
}

mod impls;
