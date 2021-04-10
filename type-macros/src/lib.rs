use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput, Data, Fields, Field};
use std::iter::{self, Iterator};


fn fields_iter<'a>(fields: &'a Fields) -> Box<dyn Iterator<Item=&'a Field> + 'a> {
    match fields {
        Fields::Named(named_fields) => {
            Box::new(named_fields.named.iter())
        },
        Fields::Unnamed(unnamed_fields) => {
            Box::new(unnamed_fields.unnamed.iter())
        },
        Fields::Unit => {
            Box::new(iter::empty())
        }
    }
}

fn make_align_from_fields(fields: &Fields) -> TokenStream2 {
    fields_iter(fields).fold(quote!{ 1usize }, |accum, field| {
        let ty = &field.ty;
        quote!{ types::math::lcm(#accum, <#ty as types::Entity>::align(cfg)) }
    })
}

fn make_align(data: &Data) -> TokenStream2 {
    match data {
        Data::Struct(struct_data) => {
            make_align_from_fields(&struct_data.fields)
        },
        Data::Enum(enum_data) => {
            enum_data.variants.iter().fold(quote!{ 1usize }, |accum, variant| {
                let variant_align = make_align_from_fields(&variant.fields);
                quote!{ types::math::lcm(#accum, #variant_align) }
            })
        },
        Data::Union(_) => panic!("Union derive is not supported yet"),
    }
}

fn make_type_size_from_fields(fields: &Fields) -> TokenStream2 {
    fields_iter(fields).fold(quote!{ 0usize }, |accum, field| {
        let ty = &field.ty;
        quote!{ types::math::upper_multiple(#accum, <#ty as types::Entity>::align(cfg)) + <#ty as types::SizedEntity>::type_size(cfg) }
    })
}

fn make_type_size(data: &Data) -> TokenStream2 {
    let unaligned_type_size = match data {
        Data::Struct(struct_data) => {
            make_type_size_from_fields(&struct_data.fields)
        },
        Data::Enum(enum_data) => {
            enum_data.variants.iter().fold(quote!{ 1usize }, |accum, variant| {
                let variant_type_size = make_type_size_from_fields(&variant.fields);
                quote!{ std::cmp::max(#accum, #variant_type_size) }
            })
        },
        Data::Union(_) => panic!("Union derive is not supported yet"),
    };
    quote!{ types::math::upper_multiple(#unaligned_type_size, <Self as types::Entity>::align(cfg)) }
}

#[proc_macro_derive(SizedEntity)]
pub fn derive_sized_entity(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    let ty = &input.ident;
    let align = make_align(&input.data);
    let type_size = make_type_size(&input.data);

    let expanded = quote! {
        impl types::Entity for #ty {
            fn align(cfg: &types::Config) -> usize {
                #align
            }
    
            fn size(&self, cfg: &types::Config) -> usize {
                <Self as types::SizedEntity>::type_size(cfg)
            }
            
            fn load<R: types::io::CntRead>(cfg: &types::Config, src: &mut R) -> std::io::Result<Self> {
                Err(std::io::ErrorKind::Other.into())
            }
    
            fn store<W: types::io::CntWrite>(&self, cfg: &types::Config, dst: &mut W) -> std::io::Result<()> {
                Err(std::io::ErrorKind::Other.into())
            }
    
            fn source(cfg: &types::Config) -> types::SourceInfo {
                types::SourceInfo::new(String::new(), String::new())
            }
        }

        impl types::SizedEntity for #ty {
            fn type_size(cfg: &types::Config) -> usize {
                #type_size
            }
        }
    };

    TokenStream::from(expanded)
}
