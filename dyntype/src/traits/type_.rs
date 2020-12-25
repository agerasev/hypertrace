use std::{
    io::{self, Read},
    hash::{Hash, Hasher},
    any::TypeId,
};
use crate::{Config};
use super::inst::*;

pub use std::collections::hash_map::DefaultHasher;

/// Runtime type basic methods.
pub trait BasicType: 'static {
    /// Type unique identifier.
    fn id(&self) -> u64;

    /// Align of dynamic type.
    fn align(&self, config: &Config) -> usize;
}

/// Runtime type basic methods.
pub trait BasicSizedType: BasicType {
    /// Size of any instance.
    fn size(&self, config: &Config) -> usize;
}

/// Abstract runtime type.
pub trait Type: BasicType {
    /// Clones abstract type.
    fn clone_dyn(&self) -> Box<dyn Type>;

    /// Loads the instance of abstract type.
    fn load_dyn(&self, config: &Config, src: &mut dyn Read) -> io::Result<Box<dyn Inst>>;
}

/// Type which instances have the same fixed size.
pub trait SizedType: Type + BasicSizedType {
    /// Clones abstract type.
    fn clone_sized_dyn(&self) -> Box<dyn SizedType>;

    /// Loads the instance of abstract type.
    fn load_sized_dyn(&self, config: &Config, src: &mut dyn Read) -> io::Result<Box<dyn SizedInst>>;

    /// Upcast to type.
    fn into_type_dyn(self: Box<Self>) -> Box<dyn Type>;
}

impl<T: SizedType> Type for T {
    fn clone_dyn(&self) -> Box<dyn Type> {
        self.clone_sized_dyn().into_type_dyn()
    }

    fn load_dyn(&self, config: &Config, src: &mut dyn Read) -> io::Result<Box<dyn Inst>> {
        self.load_sized_dyn(config, src).map(|st| st.into_inst_dyn())
    }
}

/// Type unique identifier.
pub fn type_id<T: 'static>() -> u64 {
    let mut hasher = DefaultHasher::new();
    TypeId::of::<T>().hash(&mut hasher);
    hasher.finish()
}
