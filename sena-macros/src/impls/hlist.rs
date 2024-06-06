use proc_macro::TokenStream;

use darling::ast;
use darling::{FromDeriveInput, FromField};

use quote::quote;

use crate::crate_ident;

#[derive(FromField)]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

#[derive(FromDeriveInput)]
#[darling(supports(struct_any), attributes(tail))]
struct Input {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<(), Field>,
}

fn zero_ty() -> syn::Type {
    let root = crate_ident();
    syn::parse_quote! { ::#root::pipeline::picker::Z }
}

fn succ_ty(to: syn::Type) -> syn::Type {
    let root = crate_ident();
    syn::parse_quote! { ::#root::pipeline::picker::S<#to> }
}

fn to_hlist_ty(fields: &ast::Fields<Field>) -> syn::Type {
    let root = crate_ident();
    let mut ty: syn::Type = syn::parse_quote!(::#root::pipeline::hlist::HNil);
    for field in fields.iter().rev() {
        let field_ty = &field.ty;

        ty = syn::parse_quote!(::#root::pipeline::hlist::HCons<#field_ty, #ty>);
    }

    ty
}
fn to_hlist_value(fields: &ast::Fields<Field>) -> syn::Expr {
    let root = crate_ident();
    let mut value: syn::Expr = syn::parse_quote!(::#root::pipeline::hlist::HNil);
    for (idx, field) in fields.iter().enumerate().rev() {
        let field_access = field.ident.as_ref().map_or(quote!(#idx), |i| quote!(#i));
        value = syn::parse_quote!(::#root::pipeline::hlist::HCons(this.#field_access, #value));
    }

    value
}

fn impl_hlist(
    ident: syn::Ident,
    fields: ast::Fields<Field>,
    generics: syn::Generics,
) -> proc_macro2::TokenStream {
    let root = crate_ident();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let hlist_ty = to_hlist_ty(&fields);
    let hlist_val = to_hlist_value(&fields);

    let mut picker_impls: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields.len());
    let mut index_ty = zero_ty();

    for (idx, field) in fields.iter().enumerate() {
        let field_ty = &field.ty;
        let field_access = if let Some(ref ident) = field.ident {
            quote!(#ident)
        } else {
            quote!(#idx)
        };

        picker_impls.push(quote! {
            impl #impl_generics ::#root::pipeline::picker::ByRefPicker<#field_ty, #index_ty> for #ident #ty_generics #where_clause {
                fn pick_ref(&self) -> &#field_ty {
                    &self.#field_access
                }

                fn pick_mut(&mut self) -> &mut #field_ty {
                    &mut self.#field_access
                }
            }
        });
        index_ty = succ_ty(index_ty);
    }

    quote! {
        impl #impl_generics ::#root::pipeline::hlist::HList for #ident #ty_generics #where_clause {
            fn prepend<K>(self, item: K) -> ::#root::pipeline::hlist::HCons<K, Self> {
                ::#root::pipeline::hlist::HCons(item, self)
            }
        }
        impl #impl_generics From<#ident #ty_generics> for #hlist_ty #where_clause {
            fn from(this: #ident #ty_generics) -> Self {
                #hlist_val
            }
        }
        impl #impl_generics #ident #ty_generics #where_clause {
            // Rust has skill issue determining whether any destructor here run
            // so we can't place `const` here
            pub fn into_hlist(self) -> #hlist_ty {
                let this = self;
                #hlist_val
            }
        }

        #(#picker_impls)*
    }
}

pub fn derive(stream: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(stream as syn::DeriveInput);
    let input = match Input::from_derive_input(&derive_input) {
        Ok(r) => r,
        Err(e) => return e.write_errors().into(),
    };
    let fields = input.data.take_struct().unwrap();

    impl_hlist(input.ident, fields, input.generics).into()
}
