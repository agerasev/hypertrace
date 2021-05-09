use crate::utils::fields_iter;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::iter::Iterator;
use syn::{self, Data, DeriveInput, Fields, GenericParam};

fn make_where_clause_fields(fields: &Fields, bound: &TokenStream2, last_bound: Option<&TokenStream2>) -> TokenStream2 {
    let iter = fields_iter(fields);
    let len = iter.len();
    iter.enumerate().fold(quote! {}, |accum, (index, field)| {
        let ty = &field.ty;
        let b = if index + 1 < len { bound } else { last_bound.unwrap_or(bound) };
        quote! {
            #accum
            #ty: #b,
        }
    })
}

pub fn make_where_clause(input: &DeriveInput, bound: TokenStream2, last_bound: Option<TokenStream2>) -> TokenStream2 {
    match &input.data {
        Data::Struct(struct_data) => make_where_clause_fields(&struct_data.fields, &bound, last_bound.as_ref()),
        Data::Enum(enum_data) => enum_data.variants.iter().fold(quote! {}, |accum, variant| {
            let variant_clause = make_where_clause_fields(&variant.fields, &bound, last_bound.as_ref());
            quote! {
                #accum
                #variant_clause
            }
        }),
        Data::Union(_) => panic!("Union derive is not supported yet"),
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
