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

/// Dynamic runtime type.
pub trait DynType: BasicType {
    /// Clones dynamic type.
    fn clone_dyn(&self) -> Box<dyn DynType>;

    /// Loads the instance of dynamic type.
    fn load_dyn(&self, cfg: &Config, src: &mut dyn Read) -> io::Result<Box<dyn DynValue>>;
}

/// Sized dynamic runtime type.
pub trait SizedDynType: DynType + BasicSizedType {
    /// Clones dynamic type.
    fn clone_sized_dyn(&self) -> Box<dyn SizedDynType>;

    /// Loads the instance of dynamic type.
    fn load_sized_dyn(
        &self,
        cfg: &Config,
        src: &mut dyn Read,
    ) -> io::Result<Box<dyn SizedDynValue>>;

    /// Upcast to DynType.
    fn into_type_dyn(self: Box<Self>) -> Box<dyn DynType>;
}

/// Concrete type.
pub trait Type: BasicType + DynType + Clone {
    type Value: Value<Type = Self>;

    /// Loads the instance of type.
    fn load<R: Read + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value>;
}

/// Sized runtime type.
pub trait SizedType: BasicSizedType + SizedDynType + Type
where
    Self::Value: SizedValue<Type = Self>,
{
}

impl<T> DynType for T
where
    T: Type,
{
    fn clone_dyn(&self) -> Box<dyn DynType> {
        Box::new(self.clone())
    }

    fn load_dyn(&self, cfg: &Config, src: &mut dyn Read) -> io::Result<Box<dyn DynValue>> {
        self.load(cfg, src)
            .map(|v| Box::new(v) as Box<dyn DynValue>)
    }
}

impl<T> SizedDynType for T
where
    T: SizedType,
    T::Value: SizedValue,
{
    fn clone_sized_dyn(&self) -> Box<dyn SizedDynType> {
        Box::new(self.clone())
    }

    fn load_sized_dyn(
        &self,
        cfg: &Config,
        src: &mut dyn Read,
    ) -> io::Result<Box<dyn SizedDynValue>> {
        self.load(cfg, src)
            .map(|v| Box::new(v) as Box<dyn SizedDynValue>)
    }

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
