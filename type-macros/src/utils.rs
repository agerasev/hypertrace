use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::iter::{self, ExactSizeIterator};
use syn::{self, Field, Fields, FieldsNamed, FieldsUnnamed, Ident};

pub type FieldsIterator<'a> = Box<dyn ExactSizeIterator<Item = &'a Field> + 'a>;

pub trait FieldsIter {
    fn fields_iter(&self) -> FieldsIterator<'_>;
}

impl FieldsIter for Fields {
    fn fields_iter(&self) -> FieldsIterator<'_> {
        match self {
            Fields::Named(named_fields) => named_fields.fields_iter(),
            Fields::Unnamed(unnamed_fields) => unnamed_fields.fields_iter(),
            Fields::Unit => Box::new(iter::empty()),
        }
    }
}
impl FieldsIter for FieldsNamed {
    fn fields_iter(&self) -> FieldsIterator<'_> {
        Box::new(self.named.iter())
    }
}
impl FieldsIter for FieldsUnnamed {
    fn fields_iter(&self) -> FieldsIterator<'_> {
        Box::new(self.unnamed.iter())
    }
}

pub fn fields_iter<FI: FieldsIter>(fields: &FI) -> FieldsIterator<'_> {
    fields.fields_iter()
}

pub struct Bindings {
    pub bindings: TokenStream2,
    pub wrapper: TokenStream2,
    pub prefix: TokenStream2,
}

pub fn make_bindings(fields: &Fields) -> Bindings {
    match fields {
        Fields::Named(named_fields) => {
            let bindings = named_fields.named.iter().fold(quote! {}, |accum, field| {
                let ident = field.ident.as_ref().unwrap();
                quote! { #accum #ident, }
            });
            Bindings {
                bindings: quote! { { #bindings } },
                wrapper: quote! {},
                prefix: quote! {},
            }
        }
        Fields::Unnamed(unnamed_fields) => {
            let bindings =
                unnamed_fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .fold(quote! {}, |accum, (index, _)| {
                        let ident = Ident::new(&format!("b{}", index), Span::call_site());
                        quote! { #accum #ident, }
                    });
            Bindings {
                bindings: quote! { (#bindings) },
                wrapper: quote! { let wrapper = (#bindings); },
                prefix: quote! { wrapper. },
            }
        }
        Fields::Unit => Bindings {
            bindings: quote! {},
            wrapper: quote! {},
            prefix: quote! {},
        },
    }
}
