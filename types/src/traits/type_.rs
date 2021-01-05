use super::value::*;
use crate::{io::*, Config};
use std::{
    any::{Any, TypeId},
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    io,
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

        /// Format type information.
        fn debug_fmt_dyn(&self, f: &mut Formatter) -> fmt::Result;

        /// Loads the instance of dynamic type.
        fn load_dyn(&self, cfg: &Config, src: &mut dyn CountingRead) -> io::Result<Box<dyn $V>>;

        /// Cast to `Any`.
        fn as_any(&self) -> &dyn Any;

        /// Cast to mutable `Any`.
        fn as_mut_any(&mut self) -> &mut dyn Any;
    };
}

/// Dynamic runtime type.
pub trait TypeDyn: 'static {
    def_dyn_type!(TypeDyn, ValueDyn);
}

/// Sized dynamic runtime type.
pub trait SizedTypeDyn: 'static {
    def_dyn_type!(SizedTypeDyn, SizedValueDyn);

    /// Size of any instance.
    fn size_dyn(&self, cfg: &Config) -> usize;

    /// Upcast to TypeDyn.
    fn into_type_dyn(self: Box<Self>) -> Box<dyn TypeDyn>;
}

/// Concrete type.
pub trait Type: Clone + Debug + 'static {
    /// Associated dynamic value.
    type Value: Value<Type = Self>;

    /// Type unique identifier.
    fn id(&self) -> u64;

    /// Align of dynamic type.
    fn align(&self, cfg: &Config) -> usize;

    /// Loads the instance of type.
    fn load<R: CountingRead + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value>;
}

/// Sized runtime type.
pub trait SizedType: Type
where
    Self::Value: SizedValue<Type = Self>,
{
    /// Size of any instance.
    fn size(&self, cfg: &Config) -> usize;
}

pub trait UnitType: Type + Default
where
    Self::Value: UnitValue<Type = Self>,
{
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

        fn debug_fmt_dyn(&self, f: &mut Formatter) -> fmt::Result {
            <Self as Debug>::fmt(self, f)
        }

        fn load_dyn(&self, cfg: &Config, src: &mut dyn CountingRead) -> io::Result<Box<dyn $V>> {
            self.load(cfg, src).map(|v| Box::new(v) as Box<dyn $V>)
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_mut_any(&mut self) -> &mut dyn Any {
            self
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
