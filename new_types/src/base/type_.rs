use crate::{
    io::CountingRead, Config, Entity, EntityValue, SizedEntity, SizedValue, SourceInfo, Value,
};
use std::{
    any::type_name,
    fmt::{self, Debug, Formatter},
    io,
    marker::PhantomData,
};

/// Dynamic entity type.
pub trait Type: Clone + Debug + 'static {
    /// Associated dynamic value.
    type Value: Value<Type = Self>;

    /// Type unique identifier.
    fn id(&self, cfg: &Config) -> u64;

    /// Align of dynamic type.
    fn align(&self, cfg: &Config) -> usize;

    /// Loads the instance of type.
    fn load<R: CountingRead>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value>;

    /// Returns a kernel source info of the type.
    fn source(&self, cfg: &Config) -> SourceInfo;
}

/// Sized dynamic entity type.
pub trait SizedType: Type
where
    Self::Value: SizedValue<Type = Self>,
{
    /// Size of any instance.
    fn size(&self, cfg: &Config) -> usize;
}

/// Wrapper that created `Type` from `Entity` type.
pub struct EntityType<E: Entity>(PhantomData<E>);

impl<E: Entity> EntityType<E> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<E: Entity> Default for EntityType<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: Entity> Clone for EntityType<E> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<E: Entity> Debug for EntityType<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", type_name::<E>())
    }
}

impl<E: Entity> Type for EntityType<E> {
    type Value = EntityValue<E>;

    fn id(&self, cfg: &Config) -> u64 {
        E::type_id(cfg)
    }

    fn align(&self, cfg: &Config) -> usize {
        E::align(cfg)
    }

    fn load<R: CountingRead>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        E::load(cfg, src).map(EntityValue::new)
    }

    fn source(&self, cfg: &Config) -> SourceInfo {
        E::source(cfg)
    }
}

impl<E: SizedEntity> SizedType for EntityType<E> {
    fn size(&self, cfg: &Config) -> usize {
        E::type_size(cfg)
    }
}
