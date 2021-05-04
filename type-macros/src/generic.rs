use crate::utils::fields_iter;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::iter::Iterator;
use syn::{self, Data, DeriveInput, Fields, GenericParam};

fn make_where_clause_fields(fields: &Fields) -> TokenStream2 {
    fields_iter(fields).fold(quote! {}, |accum, field| {
        let ty = &field.ty;
        quote! {
            #accum
            #ty: types::SizedEntity + types::Sourced,
        }
    })
}

pub fn make_where_clause(input: &DeriveInput) -> TokenStream2 {
    let clause = match &input.data {
        Data::Struct(struct_data) => make_where_clause_fields(&struct_data.fields),
        Data::Enum(enum_data) => enum_data.variants.iter().fold(quote! {}, |accum, variant| {
            let variant_clause = make_where_clause_fields(&variant.fields);
            quote! {
                #accum
                #variant_clause
            }
        }),
        Data::Union(_) => panic!("Union derive is not supported yet"),
    };
    if clause.is_empty() {
        quote! {}
    } else {
        quote! { where #clause }
    }
}

pub fn make_params(input: &DeriveInput) -> (TokenStream2, TokenStream2) {
    assert!(input.generics.where_clause.is_none());

    let params = input
        .generics
        .params
        .iter()
        .fold(quote! {}, |accum, param| {
            let param = match param {
                GenericParam::Type(type_param) => &type_param.ident,
                GenericParam::Lifetime(lifetime_def) => &lifetime_def.lifetime.ident,
                GenericParam::Const(const_param) => &const_param.ident,
            };
            quote! {
                #accum
                #param,
            }
        });

    (params, input.generics.params.to_token_stream())
}
