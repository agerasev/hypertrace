use crate::{
    io::CountingWrite,
    Config, Entity, SizedEntity, Type, SizedType, EntityType,
};
use std::io;

/// Dynamic entity value.
pub trait Value: 'static {
    /// Associated dynamic type.
    type Type: Type<Value = Self>;

    /// Size of instance.
    fn size(&self, cfg: &Config) -> usize;

    /// Align of the instance. *Must be equal to the align of type.*
    fn align(&self, cfg: &Config) -> usize {
        self.type_of().align(cfg)
    }

    /// Returns the type of the instance.
    fn type_of(&self) -> Self::Type;

    /// Stores the instance to a writer.
    fn store<W: CountingWrite + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()>;
}

/// Sized dynamic entity value.
pub trait SizedValue: Value
where
    Self::Type: SizedType<Value = Self>,
{
}


/// Wrapper that created `Value` from `Entity` instance.
/// Now this is just simply an alias to `Entity`.
pub type EntityValue<E> = E;

impl<E: Entity> Value for EntityValue<E> {
    type Type = EntityType<E>;

    fn size(&self, cfg: &Config) -> usize {
        self.size(cfg)
    }

    fn type_of(&self) -> Self::Type {
        EntityType::new()
    }

    fn store<W: CountingWrite + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        self.store(cfg, dst)
    }
}

impl<E: SizedEntity> SizedValue for EntityValue<E> {}
