
mod utils;
mod entity;
mod source;
mod generic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};
use crate::{
    entity::{make_align, make_type_size, make_load, make_store},
    source::make_source,
    generic::{make_params, make_where_clause},
};

#[proc_macro_derive(SizedEntity)]
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

            fn entity_source(cfg: &types::Config) -> types::SourceInfo {
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
