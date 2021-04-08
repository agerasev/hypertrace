mod entity;
#[cfg(test)]
mod tests;
mod type_;
mod value;

pub use entity::*;
pub use type_::*;
pub use value::*;

use std::{
    any::TypeId,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// Type unique identifier.
pub fn type_id<T: 'static>() -> u64 {
    let mut hasher = DefaultHasher::new();
    TypeId::of::<T>().hash(&mut hasher);
    hasher.finish()
}
