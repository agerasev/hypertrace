mod entity;
mod generic;
mod source;
mod utils;

use crate::{
    entity::{
        make_align, make_is_dyn_sized, make_load, make_min_size, make_size, make_store,
        make_type_size,
    },
    generic::{make_params, make_where_clause},
    source::make_source,
};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_derive(Named)]
pub fn derive_named(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    let ty = &input.ident;
    let (params, bindings) = make_params(&input);

    let name = ty.to_string();
    let prefix = name.to_case(Case::Snake);

    let (tag, div) = if input.generics.params.is_empty() {
        (quote! { "" }, quote! { "" })
    } else {
        (
            quote! { <Self as types::Named>::type_tag() },
            quote! { "_" },
        )
    };

    TokenStream::from(quote! {
        impl<#bindings> types::Named for #ty<#params> {
            fn type_name(cfg: &types::Config) -> String {
                format!("{}{}", #name, #tag)
            }
            fn type_prefix(cfg: &types::Config) -> String {
                format!("{}{}{}", #prefix, #div, #tag)
            }
        }
    })
}

#[proc_macro_derive(Entity, attributes(getter))]
pub fn derive_entity(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    let ty = &input.ident;
    let align = make_align(&input);
    let size = make_size(&input);
    let min_size = make_min_size(&input);
    let is_dyn_sized = make_is_dyn_sized(&input);
    let load = make_load(&input);
    let store = make_store(&input);
    let source = make_source(&input);
    let (params, bindings) = make_params(&input);
    let mut where_clause = make_where_clause(
        &input,
        quote! { types::SizedEntity + types::Sourced },
        Some(quote! { types::Entity + types::Sourced }),
    );
    if !where_clause.is_empty() {
        where_clause = quote! { where #where_clause };
    }

    let expanded = quote! {
        impl<#bindings> types::Entity for #ty<#params> #where_clause {
            fn align(cfg: &types::Config) -> usize {
                #align
            }

            fn size(&self, cfg: &types::Config) -> usize {
                #size
            }

            fn min_size(cfg: &types::Config) -> usize {
                #min_size
            }

            fn is_dyn_sized() -> bool {
                #is_dyn_sized
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
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(SizedEntity)]
pub fn derive_sized_entity(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    let ty = &input.ident;
    let type_size = make_type_size(&input);
    let (params, bindings) = make_params(&input);
    let mut where_clause =
        make_where_clause(&input, quote! { types::SizedEntity + types::Sourced }, None);
    if !where_clause.is_empty() {
        where_clause = quote! { where #where_clause };
    }

    let expanded = quote! {
        impl<#bindings> types::SizedEntity for #ty<#params> #where_clause {
            fn type_size(cfg: &types::Config) -> usize {
                #type_size
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Sourced)]
pub fn derive_sourced(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    let ty = &input.ident;
    let (params, bindings) = make_params(&input);

    TokenStream::from(quote! {
        impl<#bindings> types::Sourced for #ty<#params> where Self: types::Entity {
            fn source(cfg: &types::Config) -> types::source::SourceTree {
                <Self as types::Entity>::type_source(cfg)
            }
        }
    })
}
