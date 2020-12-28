use super::value::*;
use crate::Config;
use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    io::{self, Read},
};

pub use std::collections::hash_map::DefaultHasher;

macro_rules! def_dyn_type {
    ($T:ident, $V:ident) => {
        /// Type unique identifier.
        fn id_dyn(&self) -> u64;

        /// Align of dynamic type.
        fn align_dyn(&self, cfg: &Config) -> usize;

        /// Clones dynamic type.
        fn clone_dyn(&self) -> Box<dyn $T>;

        /// Loads the instance of dynamic type.
        fn load_dyn(&self, cfg: &Config, src: &mut dyn Read) -> io::Result<Box<dyn $V>>;
    };
}

/// Dynamic runtime type.
pub trait TypeDyn {
    def_dyn_type!(TypeDyn, ValueDyn);
}

/// Sized dynamic runtime type.
pub trait SizedTypeDyn {
    def_dyn_type!(SizedTypeDyn, SizedValueDyn);

    /// Size of any instance.
    fn size_dyn(&self, cfg: &Config) -> usize;

    /// Upcast to TypeDyn.
    fn into_type_dyn(self: Box<Self>) -> Box<dyn TypeDyn>;
}

/// Concrete type.
pub trait Type: Clone + 'static {
    /// Associated dynamic value.
    type Value: Value<Type = Self>;

    /// Type unique identifier.
    fn id(&self) -> u64;

    /// Align of dynamic type.
    fn align(&self, cfg: &Config) -> usize;

    /// Loads the instance of type.
    fn load<R: Read + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value>;
}

/// Sized runtime type.
pub trait SizedType: Type
where
    Self::Value: SizedValue<Type = Self>,
{
    /// Size of any instance.
    fn size(&self, cfg: &Config) -> usize;
}

macro_rules! impl_dyn_type {
    ($T:ident, $V:ident) => {
        fn id_dyn(&self) -> u64 {
            self.id()
        }

        fn align_dyn(&self, cfg: &Config) -> usize {
            self.align(cfg)
        }

        fn clone_dyn(&self) -> Box<dyn $T> {
            Box::new(self.clone())
        }

        fn load_dyn(&self, cfg: &Config, src: &mut dyn Read) -> io::Result<Box<dyn $V>> {
            self.load(cfg, src).map(|v| Box::new(v) as Box<dyn $V>)
        }
    };
}

impl<T> TypeDyn for T
where
    T: Type,
{
    impl_dyn_type!(TypeDyn, ValueDyn);
}

impl<T> SizedTypeDyn for T
where
    T: SizedType,
    T::Value: SizedValue,
{
    impl_dyn_type!(SizedTypeDyn, SizedValueDyn);

    fn size_dyn(&self, cfg: &Config) -> usize {
        self.size(cfg)
    }

    fn into_type_dyn(self: Box<Self>) -> Box<dyn TypeDyn> {
        self
    }
}

/// Type unique identifier.
pub fn type_id<T: 'static>() -> u64 {
    let mut hasher = DefaultHasher::new();
    TypeId::of::<T>().hash(&mut hasher);
    hasher.finish()
}
