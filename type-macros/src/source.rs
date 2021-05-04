use crate::utils::fields_iter;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::iter::Iterator;
use syn::{self, Attribute, Data, DeriveInput, Fields};

fn getter_required(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .filter_map(|a| a.path.get_ident())
        .any(|i| i == "getter")
}

fn make_source_fields(fields: &Fields, name: TokenStream2, prefix: TokenStream2) -> TokenStream2 {
    let fields_list = fields_iter(fields).collect::<Vec<_>>();
    if !fields_list.is_empty() {
        let content = fields_list.into_iter().enumerate().fold(quote!{}, |accum, (index, field)| {
            let ty = &field.ty;
            let fname = match &field.ident {
                Some(x) => quote!{ std::stringify!(#x) },
                None => quote!{ format!("field_{}", #index) },
            };

            let getter = if getter_required(&field.attrs) {
                quote!{
                    for (ns, nsp) in &namespaces {
                        getter_text += &format!(
                            "{ns}{} *{}__{}{nsp}({ns}{} *self) {{\n",
                            &field_type_name,
                            &#prefix,
                            &#fname,
                            &#name,
                            ns = ns,
                            nsp = nsp,
                        );
                        if field_size > 0 {
                            getter_text += &format!("    return &self->{};\n", &#fname);
                        } else {
                            getter_text += "    return NULL;\n";
                        }
                        getter_text += "}\n";
                    }
                }
            } else {
                quote!{}
            };

            quote!{
                #accum
                {
                    let field_size = <#ty as types::SizedEntity>::type_size(cfg);
                    size = types::math::upper_multiple(size, <#ty as types::Entity>::align(cfg)) +
                        field_size;
                    align = types::math::lcm(align, <#ty as types::Entity>::align(cfg));

                    let field_type_name = <#ty as types::Named>::type_name(cfg);
                    let field_src = <#ty as types::Entity>::type_source(cfg);
                    if field_size > 0 {
                        text += &format!("    {} {};\n", &field_type_name, #fname);
                    }

                    includes.insert(field_src.root().to_path_buf());
                    tree.append(field_src).unwrap();

                    #getter
                }
            }
        });
        quote! {
            let mut size = 0;
            let mut align = 0;
            text += &format!("typedef struct {} {{\n", &#name);
            #content
            text += &format!("}} {};\n", &#name);
        }
    } else {
        quote! {
            text += &format!("typedef void {};\n", &#name);
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

                let getter = if getter_required(&variant.attrs) {
                    quote!{
                        for (ns, nsp) in &namespaces {
                            getter_text += &format!(
                                "{ns}{} *{}__variant_{}{nsp}({ns}{} *self) {{\n",
                                &enum_name,
                                &type_prefix,
                                #index,
                                &type_name,
                                ns = ns,
                                nsp = nsp,
                            );
                            if size > 0 {
                                getter_text += &format!("    return &self->variant_{};\n", #index);
                            } else {
                                getter_text += "    return NULL;\n";
                            }
                            getter_text += "}\n";
                        }
                    }
                } else {
                    quote!{}
                };

                quote!{
                    #accum
                    {
                        let enum_name = format!("{}_Struct{}", type_name, #index);
                        let enum_prefix = format!("{}_struct{}", type_prefix, #index);
                        let mut size = 0;
                        let mut align = 1;
                        #enum_source
                        enum_size = std::cmp::max(enum_size, size);
                        enum_align = types::math::lcm(enum_align, align);
                        if size > 0 {
                            enum_text += &format!("        {} variant_{};\n", enum_name, #index);
                        }

                        #getter
                    }
                }
            });
            quote! {
                let mut enum_size = 0;
                let mut enum_align = 1;
                let mut enum_text = String::new();
                #enums
                //enum_size = types::math::upper_multiple(enum_size, enum_align);
                //text += &[
                //    format!("typedef struct {} __attribute__((aligned({}))) {{\n", type_name, enum_align),
                //    format!("    uchar placeholder[{}];\n", enum_size),
                //    format!("}} {};\n", type_name),
                //].iter().join("")
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
        let mut text = String::new();
        let mut getter_text = String::new();
        let namespaces = [("", ""), ("const ", "__c"), ("__global ", "__g"), ("__global const ", "__gc")];
        #content
        let final_text = [
            "#pragma once\n".into(),
            includes.into_iter().map(|path| format!("#include <{}>\n", path)).collect::<String>(),
            "\n".into(),
            text,
            "\n".into(),
            getter_text,
        ].join("");

        tree.insert(file.into(), final_text).unwrap();
        tree
    }
}
