use syn::{self, Fields, Field};
use std::iter::{self, Iterator};

pub fn fields_iter<'a>(fields: &'a Fields) -> Box<dyn Iterator<Item=&'a Field> + 'a> {
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
