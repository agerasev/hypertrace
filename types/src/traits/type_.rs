use super::value::*;
use crate::{io::*, Config};
use std::{
    any::TypeId,
    fmt::Debug,
    hash::{Hash, Hasher},
    io,
};
pub use std::collections::hash_map::DefaultHasher;

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


macro_rules! def_type_dyn {
    ($T:ident, $V:ident) => {
        /// Type unique identifier.
        fn id_dyn(&self) -> u64;

        /// Align of dynamic type.
        fn align_dyn(&self, cfg: &$crate::Config) -> usize;

        /// Clones dynamic type.
        fn clone_dyn(&self) -> Box<dyn $T>;

        /// Format type information.
        fn debug_fmt_dyn(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;

        /// Loads the instance of dynamic type.
        fn load_dyn(&self, cfg: &$crate::Config, src: &mut dyn $crate::CountingRead) -> std::io::Result<Box<dyn $V>>;

        /// Cast to `Any`.
        fn as_any(&self) -> &dyn std::any::Any;

        /// Cast to mutable `Any`.
        fn as_mut_any(&mut self) -> &mut dyn std::any::Any;

        /// Upcast to TypeDyn.
        fn into_type_dyn(self: Box<Self>) -> Box<dyn TypeDyn>;
    };
}

macro_rules! def_sized_type_dyn {
    ($T:ident, $V:ident) => {
        /// Size of any instance.
        fn size_dyn(&self, cfg: &$crate::Config) -> usize;

        /// Upcast to SizedTypeDyn.
        fn into_sized_type_dyn(self: Box<Self>) -> Box<dyn SizedTypeDyn>;
    };
}

/// Dynamic runtime type.
pub trait TypeDyn: 'static {
    def_type_dyn!(TypeDyn, ValueDyn);
}

/// Sized dynamic runtime type.
pub trait SizedTypeDyn: 'static {
    def_type_dyn!(SizedTypeDyn, SizedValueDyn);
    def_sized_type_dyn!(SizedTypeDyn, SizedValueDyn);
}

macro_rules! impl_type_dyn {
    ($T:ident, $V:ident) => {
        fn id_dyn(&self) -> u64 {
            self.id()
        }

        fn align_dyn(&self, cfg: &$crate::Config) -> usize {
            self.align(cfg)
        }

        fn clone_dyn(&self) -> Box<dyn $T> {
            Box::new(self.clone())
        }

        fn debug_fmt_dyn(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            <Self as std::fmt::Debug>::fmt(self, f)
        }

        fn load_dyn(&self, cfg: &$crate::Config, src: &mut dyn $crate::CountingRead) -> std::io::Result<Box<dyn $V>> {
            self.load(cfg, src).map(|v| Box::new(v) as Box<dyn $V>)
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
            self
        }

        fn into_type_dyn(self: Box<Self>) -> Box<dyn TypeDyn> {
            self
        }
    };
}

macro_rules! impl_sized_type_dyn {
    ($T:ident, $V:ident) => {
        fn size_dyn(&self, cfg: &$crate::Config) -> usize {
            self.size(cfg)
        }

        fn into_sized_type_dyn(self: Box<Self>) -> Box<dyn SizedTypeDyn> {
            self
        }
    };
}

impl<T> TypeDyn for T
where
    T: Type,
{
    impl_type_dyn!(TypeDyn, ValueDyn);
}

impl<T> SizedTypeDyn for T
where
    T: SizedType,
    T::Value: SizedValue,
{
    impl_type_dyn!(SizedTypeDyn, SizedValueDyn);
    impl_sized_type_dyn!(SizedTypeDyn, SizedValueDyn);
}

/// Type unique identifier.
pub fn type_id<T: 'static>() -> u64 {
    let mut hasher = DefaultHasher::new();
    TypeId::of::<T>().hash(&mut hasher);
    hasher.finish()
}
