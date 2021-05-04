use proc_macro2::{TokenStream as TokenStream2};
use quote::quote;
use syn::{self, DeriveInput, Data, Fields};
use std::iter::Iterator;
use crate::utils::fields_iter;

fn make_source_fields(fields: &Fields, name: TokenStream2, prefix: TokenStream2) -> TokenStream2 {
    let fields_list = fields_iter(fields).collect::<Vec<_>>();
    if !fields_list.is_empty() {
        let content = fields_list.into_iter().enumerate().fold(quote!{}, |accum, (index, field)| {
            let ty = &field.ty;
            let fname = match &field.ident {
                Some(x) => quote!{ std::stringify!(#x) },
                None => quote!{ format!("field_{}", #index) },
            };
            quote!{
                #accum
                {
                    let field_size = <#ty as types::SizedEntity>::type_size(cfg);
                    size = types::math::upper_multiple(size, <#ty as types::Entity>::align(cfg)) +
                        field_size;
                    align = types::math::lcm(align, <#ty as types::Entity>::align(cfg));

                    let src = <#ty as types::Entity>::entity_source(cfg);
                    if field_size > 0 {
                        text += &format!("    {} {};\n", &src.name, #fname);
                    }

                    includes.insert(src.tree.root().to_path_buf());
                    tree.append(src.tree).unwrap();

                    for (ns, nsp) in &namespaces {
                        getter_text += &format!(
                            "{ns}{} *{}__{}{nsp}({ns}{} *self) {{\n",
                            &src.name,
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
            }
        });
        quote!{
            let mut size = 0;
            let mut align = 0;
            text += &format!("typedef struct {} {{\n", &#name);
            #content
            text += &format!("}} {};\n", &#name);
        }
    } else {
        quote!{
            text += &format!("typedef void {};\n", &#name);
        }
    }
}

pub fn make_source(input: &DeriveInput) -> TokenStream2 {
    let (content, (name, prefix)) = match &input.data {
        Data::Struct(struct_data) => {
            (make_source_fields(&struct_data.fields, quote!{ type_name }, quote!{ type_prefix }), ("Struct", "struct"))
        },
        Data::Enum(enum_data) => {
            let enums = enum_data.variants.iter().enumerate().fold(quote!{}, |accum, (index, variant)| {
                let enum_source = make_source_fields(&variant.fields, quote!{ enum_name }, quote!{ enum_prefix });
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

                        for (ns, nsp) in &namespaces {
                            getter_text += &format!(
                                "{ns}{} *{}__variant_{}{nsp}({ns}{} *self) {{\n",
                                &enum_name,
                                &enum_prefix,
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
                }
            });
            (quote!{
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
            }, ("Enum", "enum"))
        },
        Data::Union(_) => panic!("Union derive is not supported yet"),
    };
    quote!{
        let type_id = <Self as types::Entity>::type_tag();
        let type_name = format!("{}{}", #name, &type_id);
        let type_prefix = format!("{}{}", #prefix, &type_id);
        let file = format!("generated/{}{}.hh", #prefix, &type_id);
        let mut tree = types::source::SourceTree::new(file.clone().into());
        let mut includes = std::collections::HashSet::<types::path::PathBuf>::new();
        let mut text = String::new();
        let mut getter_text = String::new();
        let namespaces = [("", ""), ("const ", "__c"), ("__global ", "__g"), ("__global const ", "__gc")];
        #content
        let final_text = [
            includes.into_iter().map(|path| format!("#include <{}>\n", path)).collect::<String>(),
            text,
            getter_text,
        ].join("");

        tree.insert(file.into(), final_text).unwrap();
        types::source::SourceInfo::new(
            type_name,
            type_prefix,
            tree,
        )
    }
}
