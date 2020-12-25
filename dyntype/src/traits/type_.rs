use std::{
    io::{self, Read},
    hash::{Hash, Hasher},
    any::TypeId,
};
use crate::{Config, Inst, SizedInst};

pub use std::collections::hash_map::DefaultHasher;

/// Runtime type basic methods.
pub trait BasicType: 'static {
    /// Type unique identifier.
    fn id(&self) -> u64;

    /// Align of dynamic type.
    fn align(&self, config: &Config) -> usize;
}

/// Abstract runtime type.
pub trait Type: BasicType {
    /// Clones abstract type.
    fn clone(&self) -> Box<dyn Type>;

    /// Loads the instance of abstract type.
    fn load(&self, config: &Config, src: &mut dyn Read) -> io::Result<Box<dyn Inst>>;
}

/// Type which instances have the same fixed size.
pub trait SizedType: BasicType {
    // Methods from Type

    /// Clones abstract type.
    fn clone(&self) -> Box<dyn SizedType>;

    /// Loads the instance of abstract type.
    fn load(&self, config: &Config, src: &mut dyn Read) -> io::Result<Box<dyn SizedInst>>;

    // Own methods

    /// Size of any instance.
    fn size(&self, config: &Config) -> usize;

    /// Upcast to type.
    fn into_type(self: Box<Self>) -> Box<dyn Type>;
}

impl<T: SizedType> Type for T {
    fn clone(&self) -> Box<dyn Type> {
        self.clone().into_type()
    }

    fn load(&self, config: &Config, src: &mut dyn Read) -> io::Result<Box<dyn Inst>> {
        self.load(config, src).map(|st| st.into_inst())
    }
}

/// Type unique identifier.
pub fn type_id<T: 'static>() -> u64 {
    let mut hasher = DefaultHasher::new();
    TypeId::of::<T>().hash(&mut hasher);
    hasher.finish()
}
