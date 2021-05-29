use crate::utils::{fields_iter, FieldsIter, make_bindings};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::iter::Iterator;
use syn::{self, Data, DeriveInput, Fields, Index};

fn make_align_fields<FI: FieldsIter>(fields: &FI) -> TokenStream2 {
    fields_iter(fields).fold(quote! { 1usize }, |accum, field| {
        let ty = &field.ty;
        quote! { types::math::lcm(#accum, <#ty as types::Entity>::align(cfg)) }
    })
}

pub fn make_align(input: &DeriveInput) -> TokenStream2 {
    match &input.data {
        Data::Struct(struct_data) => make_align_fields(&struct_data.fields),
        Data::Union(union_data) => make_align_fields(&union_data.fields),
        Data::Enum(enum_data) => {
            enum_data
                .variants
                .iter()
                .fold(quote! { usize::align(cfg) }, |accum, variant| {
                    let variant_align = make_align_fields(&variant.fields);
                    quote! { types::math::lcm(#accum, #variant_align) }
                })
        }
    }
}

fn make_size_fields<FI: FieldsIter>(fields: &FI, prefix: TokenStream2, init: TokenStream2, fold_fn: TokenStream2) -> TokenStream2 {
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
                #fold_fn(
                    #accum,
                    <#ty as types::Entity>::size(#prefix #field_name, cfg),
                    <#ty as types::Entity>::align(cfg),
                )   
            }
        })
}

pub fn make_size(input: &DeriveInput) -> TokenStream2 {
    let ty = &input.ident;
    let unaligned_size = match &input.data {
        Data::Struct(struct_data) => {
            make_size_fields(
                &struct_data.fields,
                quote! { &self. },
                quote! { 0usize },
                quote! { types::math::aligned_add },
            )
        }
        Data::Union(_) => quote! {
            if !<Self as types::Entity>::is_dyn_sized() {
                <Self as types::Entity>::min_size(cfg)
            } else {
                panic!("Size of dynamically-sized unions cannot be found directly")
            }
        },
        Data::Enum(enum_data) => {
            let matches = enum_data.variants.iter().fold(quote! {}, |accum, variant| {
                let var = &variant.ident;
                let bs = make_bindings(&variant.fields);
                let (bindings, wrapper) = (bs.bindings, bs.wrapper);
                let variant_size = make_size_fields(
                    &variant.fields,
                    bs.prefix,
                    quote! { types::math::upper_multiple(
                        <usize as types::SizedEntity>::static_size(cfg),
                        <Self as types::Entity>::align(cfg),
                    ) },
                    quote!{ types::math::aligned_add },
                );
                quote! {
                    #accum
                    #ty::#var #bindings => {
                        #wrapper
                        #variant_size
                    },
                }
            });
            quote! {
                match self {
                    #matches
                }
            }
        }
    };
    quote! { types::math::upper_multiple(
        std::cmp::max(#unaligned_size, <Self as types::Entity>::min_size(cfg)),
        <Self as types::Entity>::align(cfg),
    ) }
}

fn make_min_size_fields<FI: FieldsIter>(fields: &FI, init: TokenStream2, fold_fn: TokenStream2) -> TokenStream2 {
    let iter = fields_iter(fields);
    let len = iter.len();
    iter.enumerate().fold(init, |accum, (index, field)| {
        let ty = &field.ty;
        let size = if index + 1 < len {
            quote! { <#ty as types::SizedEntity>::static_size(cfg) }
        } else {
            quote! { <#ty as types::Entity>::min_size(cfg) }
        };
        quote! {
            #fold_fn(#accum, #size, <#ty as types::Entity>::align(cfg))
        }
    })
}

fn make_static_size_fields<FI: FieldsIter>(fields: &FI, init: TokenStream2, fold_fn: TokenStream2) -> TokenStream2 {
    fields_iter(fields).fold(init, |accum, field| {
        let ty = &field.ty;
        quote! {
            #fold_fn(
                #accum,
                <#ty as types::SizedEntity>::static_size(cfg),
                <#ty as types::Entity>::align(cfg),
            )
        }
    })
}

macro_rules! make_type_size {
    ($input:expr, $make_type_size_fields:ident) => {{
        let unaligned_type_size = match &$input.data {
            Data::Struct(struct_data) => $make_type_size_fields(
                &struct_data.fields,
                quote! { 0usize },
                quote! { types::math::aligned_add },
            ),
            Data::Union(union_data) => $make_type_size_fields(
                &union_data.fields,
                quote! { 0usize },
                quote! { types::math::aligned_max },
            ),
            Data::Enum(enum_data) => {
                enum_data
                    .variants
                    .iter()
                    .fold(quote! { 0usize }, |accum, variant| {
                        let variant_type_size = $make_type_size_fields(
                            &variant.fields,
                            quote! { <usize as types::SizedEntity>::static_size(cfg) },
                            quote! { types::math::aligned_add },
                        );
                        quote! { std::cmp::max(#accum, #variant_type_size) }
                    })
            },
        };
        quote! { types::math::upper_multiple(
            #unaligned_type_size,
            <Self as types::Entity>::align(cfg),
        ) }
    }};
}

pub fn make_min_size(input: &DeriveInput) -> TokenStream2 {
    make_type_size!(input, make_min_size_fields)
}

pub fn make_static_size(input: &DeriveInput) -> TokenStream2 {
    make_type_size!(input, make_static_size_fields)
}

fn make_is_dyn_sized_fields<FI: FieldsIter>(fields: &FI) -> TokenStream2 {
    fields_iter(fields)
        .last()
        .map(|field| {
            let ty = &field.ty;
            quote! { <#ty as types::Entity>::is_dyn_sized() }
        })
        .unwrap_or(quote! { false })
}

pub fn make_is_dyn_sized(input: &DeriveInput) -> TokenStream2 {
    match &input.data {
        Data::Struct(struct_data) => make_is_dyn_sized_fields(&struct_data.fields),
        Data::Union(union_data) => make_is_dyn_sized_fields(&union_data.fields),
        Data::Enum(enum_data) => {
            enum_data
                .variants
                .iter()
                .fold(quote! { false }, |accum, variant| {
                    let variant_is_dyn_sized = make_is_dyn_sized_fields(&variant.fields);
                    quote! { #accum || #variant_is_dyn_sized }
                })
        }
    }
}

fn make_load_fields(fields: &Fields, ident: TokenStream2) -> TokenStream2 {
    let field_values = fields_iter(fields).fold(quote! {}, |accum, field| {
        let ty = &field.ty;
        let value = quote! {
            {
                src.align(<#ty as types::Entity>::align(cfg))?;
                <R as types::io::EntityReader>::read_entity::<#ty>(src, cfg)?
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
                let pp = src.position();
                let index = <R as types::io::EntityReader>::read_entity::<usize>(src, cfg)?;
                src.align(<Self as types::Entity>::align(cfg))?;
                let ret = match index {
                    #matches
                    _ => { return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Enum index is out of range")); },
                };
                let ws = src.position() - pp;
                let ms = <Self as types::Entity>::min_size(cfg);
                if (ws < ms) {
                    src.skip(ms - ws)?;
                }
                ret
            }
        }
        Data::Union(_) => return quote!{
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unions cannot be loaded directly"))
        },
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
                    <W as types::io::EntityWriter>::write_entity::<#ty>(dst, cfg, #prefix #field_name)?;
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
                                <W as types::io::EntityWriter>::write_entity::<usize>(dst, cfg, &#index)?;
                                dst.align(<Self as types::Entity>::align(cfg))?;
                                #store
                            },
                        }
                    },
                );
                quote! {
                    let pp = dst.position();
                    let ret = match self {
                        #matches
                    };
                    let ws = dst.position() - pp;
                    let ms = <Self as types::Entity>::min_size(cfg);
                    if (ws < ms) {
                        dst.skip(ms - ws)?;
                    }
                }
            }
            Data::Union(_) => return quote!{
                Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unions cannot be stored directly"))
            },
        };
    quote! {
        #unaligned_store
        dst.align(<Self as types::Entity>::align(cfg))?;
        Ok(())
    }
}
