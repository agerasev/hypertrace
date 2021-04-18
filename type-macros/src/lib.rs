use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput, Data, Fields, Field, Ident, Index};
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

fn make_align_fields(fields: &Fields) -> TokenStream2 {
    fields_iter(fields).fold(quote!{ 1usize }, |accum, field| {
        let ty = &field.ty;
        quote!{ types::math::lcm(#accum, <#ty as types::Entity>::align(cfg)) }
    })
}

fn make_align(input: &DeriveInput) -> TokenStream2 {
    match &input.data {
        Data::Struct(struct_data) => {
            make_align_fields(&struct_data.fields)
        },
        Data::Enum(enum_data) => {
            enum_data.variants.iter().fold(quote!{ usize::align(cfg) }, |accum, variant| {
                let variant_align = make_align_fields(&variant.fields);
                quote!{ types::math::lcm(#accum, #variant_align) }
            })
        },
        Data::Union(_) => panic!("Union derive is not supported yet"),
    }
}

fn make_type_size_fields(fields: &Fields, init: TokenStream2) -> TokenStream2 {
    fields_iter(fields).fold(init, |accum, field| {
        let ty = &field.ty;
        quote!{ types::math::upper_multiple(#accum, <#ty as types::Entity>::align(cfg)) + <#ty as types::SizedEntity>::type_size(cfg) }
    })
}

fn make_type_size(input: &DeriveInput) -> TokenStream2 {
    let unaligned_type_size = match &input.data {
        Data::Struct(struct_data) => {
            make_type_size_fields(&struct_data.fields, quote!{ 0usize })
        },
        Data::Enum(enum_data) => {
            enum_data.variants.iter().fold(quote!{ 0usize }, |accum, variant| {
                let variant_type_size = make_type_size_fields(
                    &variant.fields,
                    quote!{ <usize as types::SizedEntity>::type_size(cfg) },
                );
                quote!{ std::cmp::max(#accum, #variant_type_size) }
            })
        },
        Data::Union(_) => panic!("Union derive is not supported yet"),
    };
    quote!{ types::math::upper_multiple(#unaligned_type_size, <Self as types::Entity>::align(cfg)) }
}

fn make_load_fields(fields: &Fields, ident: TokenStream2) -> TokenStream2 {
    let field_values = fields_iter(fields).fold(quote!{}, |accum, field| {
        let ty = &field.ty;
        let value = quote!{
            {
                src.align(<#ty as types::Entity>::align(cfg))?;
                <#ty as types::Entity>::load(cfg, src)?
            }
        };
        let named_value = match &field.ident {
            Some(ident) => quote!{ #ident: #value },
            None => value,
        };
        quote!{
            #accum
            #named_value,
        }
    });
    match fields {
        Fields::Named(_) => quote!{ #ident { #field_values } },
        Fields::Unnamed(_) => quote!{ #ident(#field_values) },
        Fields::Unit => quote!{ #ident },
    }
}

fn make_load(input: &DeriveInput) -> TokenStream2 {
    let ty = &input.ident;
    let unaligned_load = match &input.data {
        Data::Struct(struct_data) => {
            let struct_load = make_load_fields(&struct_data.fields, quote!{ #ty });
            quote!{ #struct_load }
        },
        Data::Enum(enum_data) => {
            let matches = enum_data.variants.iter().enumerate().fold(quote!{}, |accum, (index, variant)| {
                let var = &variant.ident;
                let variant_load = make_load_fields(&variant.fields, quote!{ #ty::#var });
                quote!{
                    #accum
                    #index => #variant_load,
                }
            });
            quote!{
                let index = usize::load(cfg, src)?;
                src.align(<Self as types::Entity>::align(cfg))?;
                match index {
                    #matches
                    _ => { return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Enum index is out of range")); },
                }
            }
        },
        Data::Union(_) => panic!("Union derive is not supported yet"),
    };
    quote!{
        let self_ = { #unaligned_load };
        src.align(<Self as types::Entity>::align(cfg))?;
        Ok(self_)
    }
}

fn make_store_fields(fields: &Fields, prefix: TokenStream2) -> TokenStream2 {
    fields_iter(fields).enumerate().fold(quote!{}, |accum, (index, field)| {
        let ty = &field.ty;
        let tuple_index = Index { index: index as u32, span: Span::call_site() };
        let field_name = match &field.ident {
            Some(ident) => quote!{ #ident },
            None => quote!{ #tuple_index },
        };
        let command = quote!{
            {
                dst.align(<#ty as types::Entity>::align(cfg))?;
                <#ty as types::Entity>::store(#prefix #field_name, cfg, dst)?;
            }
        };
        quote!{
            #accum
            #command
        }
    })
}

struct StoreBindings {
    bindings: TokenStream2,
    wrapper: TokenStream2,
    prefix: TokenStream2,
}

fn make_store_bindings(fields: &Fields) -> StoreBindings {
    match fields {
        Fields::Named(named_fields) => {
            let bindings = named_fields.named.iter().fold(quote!{}, |accum, field| {
                let ident = field.ident.as_ref().unwrap();
                quote!{ #accum #ident, }
            });
            StoreBindings {
                bindings: quote!{ { #bindings } },
                wrapper: quote!{},
                prefix: quote!{},
            }
        },
        Fields::Unnamed(unnamed_fields) => {
            let bindings = unnamed_fields.unnamed.iter().enumerate().fold(quote!{}, |accum, (index, _)| {
                let ident = Ident::new(&format!("b{}", index), Span::call_site());
                quote!{ #accum #ident, }
            });
            StoreBindings {
                bindings: quote!{ (#bindings) },
                wrapper: quote!{ let wrapper = (#bindings); },
                prefix: quote!{ wrapper. },
            }
        },
        Fields::Unit => StoreBindings {
            bindings: quote!{},
            wrapper: quote!{},
            prefix: quote!{},
        },
    }
}

fn make_store(input: &DeriveInput) -> TokenStream2 {
    let ty = &input.ident;
    let unaligned_store = match &input.data {
        Data::Struct(struct_data) => {
            make_store_fields(&struct_data.fields, quote!{ &self. })
        },
        Data::Enum(enum_data) => {
            let matches = enum_data.variants.iter().enumerate().fold(quote!{}, |accum, (index, variant)| {
                let var = &variant.ident;
                let bs = make_store_bindings(&variant.fields);
                let (bindings, wrapper) = (bs.bindings, bs.wrapper);
                let store = make_store_fields(&variant.fields, bs.prefix);
                quote!{
                    #accum
                    #ty::#var #bindings => {
                        #wrapper
                        <usize as types::Entity>::store(&#index, cfg, dst)?;
                        dst.align(<Self as types::Entity>::align(cfg))?;
                        #store
                    },
                }
            });
            quote!{
                match self {
                    #matches
                }
            }
        },
        Data::Union(_) => panic!("Union derive is not supported yet"),
    };
    quote!{
        #unaligned_store
        dst.align(<Self as types::Entity>::align(cfg))?;
        Ok(())
    }
}
/*
fn make_source_fields(fields: &Fields, name: TokenStream2) -> TokenStream2 {
    let content = fields_iter(fields).enumerate().fold(quote!{}, |accum, (index, field)| {
        let ty = &field.ty;
        let fname = match &field.ident {
            Some(x) => quote!{ std::stringify!(#x) },
            None => quote!{ format!("field_{}", #index) },
        };
        quote!{
            #accum
            {
                let src = <#ty as types::Entity>::entity_source(cfg);
                struct_src += format!("    {} {};\n", &src.name, #fname);
            }
        }
    });
    quote!{
        let mut struct_src = format!("typedef struct {{\n");
        #content
        struct_src += format!("}} {};\n", #name);
    }
}
*/
fn make_source(_input: &DeriveInput) -> TokenStream2 {
    quote!{
        types::SourceInfo::new(
            format!("DerivedSizedEntity{}", <Self as types::Entity>::type_id()),
            format!("derived_sized_entity_{}", <Self as types::Entity>::type_id()),
        )
    }
}

#[proc_macro_derive(SizedEntity)]
pub fn derive_sized_entity(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    let ty = &input.ident;
    let align = make_align(&input);
    let type_size = make_type_size(&input);
    let load = make_load(&input);
    let store = make_store(&input);
    let source = make_source(&input);

    let expanded = quote! {
        impl types::Entity for #ty {
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

        impl types::SizedEntity for #ty {
            fn type_size(cfg: &types::Config) -> usize {
                #type_size
            }
        }
    };

    TokenStream::from(expanded)
}
