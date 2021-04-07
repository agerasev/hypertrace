use crate::{
    io::CountingRead,
    Config,
    Entity, SizedEntity,
    SourceTree, Value, SizedValue,
    EntityValue, type_id,
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
    fn id(&self) -> u64;

    /// Align of dynamic type.
    fn align(&self, cfg: &Config) -> usize;

    /// Loads the instance of type.
    fn load<R: CountingRead + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value>;

    /// Returns a source tree of the type.
    fn source(&self, cfg: &Config) -> Option<SourceTree>;
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
pub struct EntityType<E: Entity> {
    phantom: PhantomData<E>,
}

impl<E: Entity> EntityType<E> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
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

    fn id(&self) -> u64 {
        type_id::<E>()
    }

    fn align(&self, cfg: &Config) -> usize {
        E::align(cfg)
    }

    fn load<R: CountingRead + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        E::load(cfg, src)
    }

    fn source(&self, cfg: &Config) -> Option<SourceTree> {
        E::source(cfg)
    }
}

impl<E: SizedEntity> SizedType for EntityType<E> {
    fn size(&self, cfg: &Config) -> usize {
        E::type_size(cfg)
    }
}
