mod entity;
mod generic;
mod source;
mod utils;

use crate::{
    entity::{make_align, make_size, make_type_size, make_load, make_store},
    generic::{make_params, make_where_clause},
    source::make_source,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};
use convert_case::{Case, Casing};


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
        (quote! { <Self as types::Named>::type_tag() }, quote! { "_" })
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

#[proc_macro_derive(DynSizedEntity, attributes(skip))]
pub fn derive_dyn_sized_entity(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    let ty = &input.ident;
    let align = make_align(&input);
    let size = make_size(&input);
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

#[proc_macro_derive(SizedEntity, attributes(skip))]
pub fn derive_sized_entity(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    let ty = &input.ident;
    let align = make_align(&input);
    let type_size = make_type_size(&input);
    let load = make_load(&input);
    let store = make_store(&input);
    let source = make_source(&input);
    let (params, bindings) = make_params(&input);
    let mut where_clause = make_where_clause(&input, quote! { types::SizedEntity + types::Sourced }, None);
    if !where_clause.is_empty() {
        where_clause = quote! { where #where_clause };
    }

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
