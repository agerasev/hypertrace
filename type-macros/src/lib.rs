mod entity;
mod generic;
mod source;
mod utils;

use crate::{
    entity::{make_align, make_load, make_store, make_type_size},
    generic::{make_params, make_where_clause},
    source::make_source,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Named)]
pub fn derive_named(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    let ty = &input.ident;
    let (params, bindings) = make_params(&input);

    let (name, prefix) = match &input.data {
        Data::Struct(_) => (quote! { "Struct" }, quote! { "struct" }),
        Data::Enum(_) => (quote! { "Enum" }, quote! { "enum" }),
        Data::Union(_) => (quote! { "Union" }, quote! { "union" }),
    };

    TokenStream::from(quote! {
        impl<#bindings> types::Named for #ty<#params> {
            fn type_name(cfg: &types::Config) -> String {
                format!("{}{}", #name, <Self as types::Named>::type_tag())
            }
            fn type_prefix(cfg: &types::Config) -> String {
                format!("{}_{}", #prefix, <Self as types::Named>::type_tag())
            }
        }
    })
}

#[proc_macro_derive(SizedEntity, attributes(getter))]
pub fn derive_sized_entity(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    let ty = &input.ident;
    let align = make_align(&input);
    let type_size = make_type_size(&input);
    let load = make_load(&input);
    let store = make_store(&input);
    let source = make_source(&input);
    let (params, bindings) = make_params(&input);
    let where_clause = make_where_clause(&input);

    let expanded = quote! {
        impl<#bindings> types::Entity for #ty<#params> #where_clause {
            fn align(cfg: &types::Config) -> usize {
                #align
            }

            fn size(&self, cfg: &types::Config) -> usize {
                <Self as types::SizedEntity>::type_size(cfg)
            }

            fn load<R: types::io::CntRead>(cfg: &types::Config, src: &mut R) -> std::io::Result<Self> {
                #load
            }

            fn store<W: types::io::CntWrite>(&self, cfg: &types::Config, dst: &mut W) -> std::io::Result<()> {
                #store
            }

            fn type_source(cfg: &types::Config) -> types::SourceTree {
                #source
            }
        }

        impl<#bindings> types::SizedEntity for #ty<#params> #where_clause {
            fn type_size(cfg: &types::Config) -> usize {
                #type_size
            }
        }
    };

    TokenStream::from(expanded)
}
