use crate::utils::{fields_iter, make_bindings};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::iter::Iterator;
use syn::{self, Data, DeriveInput, Fields, Index};

fn make_align_fields(fields: &Fields) -> TokenStream2 {
    fields_iter(fields).fold(quote! { 1usize }, |accum, field| {
        let ty = &field.ty;
        quote! { types::math::lcm(#accum, <#ty as types::Entity>::align(cfg)) }
    })
}

pub fn make_align(input: &DeriveInput) -> TokenStream2 {
    match &input.data {
        Data::Struct(struct_data) => make_align_fields(&struct_data.fields),
        Data::Enum(enum_data) => {
            enum_data
                .variants
                .iter()
                .fold(quote! { usize::align(cfg) }, |accum, variant| {
                    let variant_align = make_align_fields(&variant.fields);
                    quote! { types::math::lcm(#accum, #variant_align) }
                })
        }
        Data::Union(_) => panic!("Union derive is not supported yet"),
    }
}


fn make_size_fields(fields: &Fields, prefix: TokenStream2, init: TokenStream2) -> TokenStream2 {
    fields_iter(fields)
        .enumerate()
        .fold(init, |accum, (index, field)| {
            let ty = &field.ty;
            let tuple_index = Index {
                index: index as u32,
                span: Span::call_site(),
            };
            let field_name = match &field.ident {
                Some(ident) => quote! { #ident },
                None => quote! { #tuple_index },
            };
            quote! {
                types::math::upper_multiple(#accum, <#ty as types::Entity>::align(cfg)) +
                    <#ty as types::Entity>::size(#prefix #field_name, cfg)
            }
        })
}

pub fn make_size(input: &DeriveInput) -> TokenStream2 {
    let ty = &input.ident;
    let unaligned_size =
        match &input.data {
            Data::Struct(struct_data) => make_size_fields(
                &struct_data.fields,
                quote! { &self. },
                quote!{ 0usize },
            ),
            Data::Enum(enum_data) => {
                let matches = enum_data.variants.iter().fold(
                    quote! {},
                    |accum, variant| {
                        let var = &variant.ident;
                        let bs = make_bindings(&variant.fields);
                        let (bindings, wrapper) = (bs.bindings, bs.wrapper);
                        let variant_size = make_size_fields(
                            &variant.fields,
                            bs.prefix,
                            quote! { types::math::upper_multiple(
                                <usize as types::SizedEntity>::type_size(cfg),
                                <Self as types::Entity>::align(cfg),
                            ) },
                        );
                        quote! {
                            #accum
                            #ty::#var #bindings => {
                                #wrapper
                                #variant_size
                            },
                        }
                    },
                );
                quote! {
                    match self {
                        #matches
                    }
                }
            }
            Data::Union(_) => panic!("Union derive is not supported yet"),
        };
    quote! { types::math::upper_multiple(
        #unaligned_size,
        <Self as types::Entity>::align(cfg),
    ) }
}

fn make_type_size_fields(fields: &Fields, init: TokenStream2) -> TokenStream2 {
    fields_iter(fields).fold(init, |accum, field| {
        let ty = &field.ty;
        quote! {
            types::math::upper_multiple(#accum, <#ty as types::Entity>::align(cfg)) +
                <#ty as types::SizedEntity>::type_size(cfg)
        }
    })
}

pub fn make_type_size(input: &DeriveInput) -> TokenStream2 {
    let unaligned_type_size = match &input.data {
        Data::Struct(struct_data) => make_type_size_fields(&struct_data.fields, quote! { 0usize }),
        Data::Enum(enum_data) => {
            enum_data
                .variants
                .iter()
                .fold(quote! { 0usize }, |accum, variant| {
                    let variant_type_size = make_type_size_fields(
                        &variant.fields,
                        quote! { <usize as types::SizedEntity>::type_size(cfg) },
                    );
                    quote! { std::cmp::max(#accum, #variant_type_size) }
                })
        }
        Data::Union(_) => panic!("Union derive is not supported yet"),
    };
    quote! { types::math::upper_multiple(
        #unaligned_type_size,
        <Self as types::Entity>::align(cfg),
    ) }
}

fn make_load_fields(fields: &Fields, ident: TokenStream2) -> TokenStream2 {
    let field_values = fields_iter(fields).fold(quote! {}, |accum, field| {
        let ty = &field.ty;
        let value = quote! {
            {
                src.align(<#ty as types::Entity>::align(cfg))?;
                <#ty as types::Entity>::load(cfg, src)?
            }
        };
        let named_value = match &field.ident {
            Some(ident) => quote! { #ident: #value },
            None => value,
        };
        quote! {
            #accum
            #named_value,
        }
    });
    match fields {
        Fields::Named(_) => quote! { #ident { #field_values } },
        Fields::Unnamed(_) => quote! { #ident(#field_values) },
        Fields::Unit => quote! { #ident },
    }
}

pub fn make_load(input: &DeriveInput) -> TokenStream2 {
    let ty = &input.ident;
    let unaligned_load = match &input.data {
        Data::Struct(struct_data) => {
            let struct_load = make_load_fields(&struct_data.fields, quote! { #ty });
            quote! { #struct_load }
        }
        Data::Enum(enum_data) => {
            let matches =
                enum_data
                    .variants
                    .iter()
                    .enumerate()
                    .fold(quote! {}, |accum, (index, variant)| {
                        let var = &variant.ident;
                        let variant_load = make_load_fields(&variant.fields, quote! { #ty::#var });
                        quote! {
                            #accum
                            #index => #variant_load,
                        }
                    });
            quote! {
                let index = usize::load(cfg, src)?;
                src.align(<Self as types::Entity>::align(cfg))?;
                match index {
                    #matches
                    _ => { return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Enum index is out of range")); },
                }
            }
        }
        Data::Union(_) => panic!("Union derive is not supported yet"),
    };
    quote! {
        let self_ = { #unaligned_load };
        src.align(<Self as types::Entity>::align(cfg))?;
        Ok(self_)
    }
}

fn make_store_fields(fields: &Fields, prefix: TokenStream2) -> TokenStream2 {
    fields_iter(fields)
        .enumerate()
        .fold(quote! {}, |accum, (index, field)| {
            let ty = &field.ty;
            let tuple_index = Index {
                index: index as u32,
                span: Span::call_site(),
            };
            let field_name = match &field.ident {
                Some(ident) => quote! { #ident },
                None => quote! { #tuple_index },
            };
            let command = quote! {
                {
                    dst.align(<#ty as types::Entity>::align(cfg))?;
                    <#ty as types::Entity>::store(#prefix #field_name, cfg, dst)?;
                }
            };
            quote! {
                #accum
                #command
            }
        })
}

pub fn make_store(input: &DeriveInput) -> TokenStream2 {
    let ty = &input.ident;
    let unaligned_store =
        match &input.data {
            Data::Struct(struct_data) => make_store_fields(&struct_data.fields, quote! { &self. }),
            Data::Enum(enum_data) => {
                let matches = enum_data.variants.iter().enumerate().fold(
                    quote! {},
                    |accum, (index, variant)| {
                        let var = &variant.ident;
                        let bs = make_bindings(&variant.fields);
                        let (bindings, wrapper) = (bs.bindings, bs.wrapper);
                        let store = make_store_fields(&variant.fields, bs.prefix);
                        quote! {
                            #accum
                            #ty::#var #bindings => {
                                #wrapper
                                <usize as types::Entity>::store(&#index, cfg, dst)?;
                                dst.align(<Self as types::Entity>::align(cfg))?;
                                #store
                            },
                        }
                    },
                );
                quote! {
                    match self {
                        #matches
                    }
                }
            }
            Data::Union(_) => panic!("Union derive is not supported yet"),
        };
    quote! {
        #unaligned_store
        dst.align(<Self as types::Entity>::align(cfg))?;
        Ok(())
    }
}
