use crate::utils::fields_iter;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::iter::Iterator;
use syn::{self, Attribute, Data, DeriveInput, Fields};

fn is_field_ignored(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .filter_map(|a| a.path.get_ident())
        .any(|i| i == "ignore")
}

fn make_source_fields(fields: &Fields, name: TokenStream2, _prefix: TokenStream2) -> TokenStream2 {
    let fields_list = fields_iter(fields).collect::<Vec<_>>();
    if !fields_list.is_empty() {
        let content = fields_list.into_iter().enumerate().fold(quote!{}, |accum, (index, field)| {
            let ty = &field.ty;
            let fname = match &field.ident {
                Some(x) => quote!{ std::stringify!(#x) },
                None => quote!{ format!("field_{}", #index) },
            };

            if !is_field_ignored(&field.attrs) {
                quote! {
                    #accum
                    {
                        let field_type_name = <#ty as types::Named>::type_name(cfg);
                        let field_src = <#ty as types::Sourced>::source(cfg);
                        text += &format!("    {} {};\n", &field_type_name, #fname);
                        includes.insert(field_src.root().to_path_buf());
                        tree.append(field_src).unwrap();
                    }
                }
            } else {
                quote! { #accum }
            }
        });
        quote! {
            text += &format!("typedef struct {} {{\n", &#name);
            #content
            text += &format!("}} {};\n", &#name);
        }
    } else {
        quote! {
            text += &format!("typedef Empty {};\n", &#name);
        }
    }
}

pub fn make_source(input: &DeriveInput) -> TokenStream2 {
    let content = match &input.data {
        Data::Struct(struct_data) => make_source_fields(
            &struct_data.fields,
            quote! { type_name },
            quote! { type_prefix },
        ),
        Data::Enum(enum_data) => {
            let enums = enum_data.variants.iter().enumerate().fold(quote!{}, |accum, (index, variant)| {
                let enum_source = make_source_fields(&variant.fields, quote!{ enum_name }, quote!{ enum_prefix });

                quote!{
                    #accum
                    {
                        let enum_name = format!("{}__Variant{}", type_name, #index);
                        let enum_prefix = format!("{}__variant_{}", type_prefix, #index);
                        #enum_source
                        enum_text += &format!("        {} variant_{};\n", enum_name, #index);
                    }
                }
            });
            quote! {
                let mut enum_text = String::new();
                #enums
                text += &[
                    format!("typedef struct {} {{\n", type_name),
                    format!("    usize index;\n"),
                    format!("    union {{\n"),
                    enum_text,
                    format!("    }};\n"),
                    format!("}} {};\n", type_name),
                ].join("");
            }
        }
        Data::Union(_) => panic!("Union derive is not supported yet"),
    };
    quote! {
        let type_name = <Self as types::Named>::type_name(cfg);
        let type_prefix = <Self as types::Named>::type_prefix(cfg);
        let file = format!("generated/type_{}.hh", <Self as types::Named>::type_tag());
        let mut tree = types::source::SourceTree::new(file.clone());
        let mut includes = std::collections::BTreeSet::<types::path::PathBuf>::new();
        includes.insert("types.hh".into());
        let mut text = String::new();
        #content
        let final_text = [
            "#pragma once\n".into(),
            includes.into_iter().map(|path| format!("#include <{}>\n", path)).collect::<String>(),
            "\n".into(),
            text,
            "\n".into(),
        ].join("");

        tree.insert(file.into(), final_text).unwrap();
        tree
    }
}
