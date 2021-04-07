mod entity;
mod type_;
mod value;
#[cfg(test)]
mod tests;

pub use entity::*;
pub use type_::*;
pub use value::*;

use std::{
    any::{TypeId},
    collections::hash_map::{DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

pub struct SourceTree {
    pub name: String,
    pub prefix: String,
    pub root: String,
    pub tree: HashMap<String, String>,
}

/// Type unique identifier.
pub fn type_id<T: 'static>() -> u64 {
    let mut hasher = DefaultHasher::new();
    TypeId::of::<T>().hash(&mut hasher);
    hasher.finish()
}
