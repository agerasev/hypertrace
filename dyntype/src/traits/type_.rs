use super::value::*;
use crate::Config;
use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    io::{self, Read},
};

pub use std::collections::hash_map::DefaultHasher;

/// Runtime type base.
pub trait BasicType: 'static {
    /// Type unique identifier.
    fn id(&self) -> u64;

    /// Align of dynamic type.
    fn align(&self, cfg: &Config) -> usize;
}

/// Sized runtime type base.
pub trait BasicSizedType: BasicType {
    /// Size of any instance.
    fn size(&self, cfg: &Config) -> usize;
}

macro_rules! def_dyn_type {
    ($T:ident, $V:ident) => {
        /// Clones dynamic type.
        fn clone_dyn(&self) -> Box<dyn $T>;

        /// Loads the instance of dynamic type.
        fn load_dyn(&self, cfg: &Config, src: &mut dyn Read) -> io::Result<Box<dyn $V>>;
    };
}

/// Dynamic runtime type.
pub trait DynType: BasicType {
    def_dyn_type!(DynType, DynValue);
}

/// Sized dynamic runtime type.
pub trait SizedDynType: BasicSizedType {
    def_dyn_type!(SizedDynType, SizedDynValue);

    /// Upcast to DynType.
    fn into_type_dyn(self: Box<Self>) -> Box<dyn DynType>;
}

/// Concrete type.
pub trait Type: BasicType + Clone {
    type Value: Value<Type = Self>;

    /// Loads the instance of type.
    fn load<R: Read + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value>;
}

/// Sized runtime type.
pub trait SizedType: BasicSizedType + Type
where
    Self::Value: SizedValue<Type = Self>,
{
}

macro_rules! impl_dyn_type {
    ($T:ident, $V:ident) => {
        fn clone_dyn(&self) -> Box<dyn $T> {
            Box::new(self.clone())
        }
    
        fn load_dyn(&self, cfg: &Config, src: &mut dyn Read) -> io::Result<Box<dyn $V>> {
            self.load(cfg, src)
                .map(|v| Box::new(v) as Box<dyn $V>)
        }
    };
}

impl<T> DynType for T
where
    T: Type,
{
    impl_dyn_type!(DynType, DynValue);
}

impl<T> SizedDynType for T
where
    T: SizedType,
    T::Value: SizedValue,
{
    impl_dyn_type!(SizedDynType, SizedDynValue);

    fn into_type_dyn(self: Box<Self>) -> Box<dyn DynType> {
        self
    }
}

/// Type unique identifier.
pub fn type_id<T: 'static>() -> u64 {
    let mut hasher = DefaultHasher::new();
    TypeId::of::<T>().hash(&mut hasher);
    hasher.finish()
}
