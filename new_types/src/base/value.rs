use crate::{io::CountingWrite, Config, Entity, EntityType, SizedEntity, SizedType, Type};
use ref_cast::RefCast;
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
    fn store<W: CountingWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()>;
}

/// Sized dynamic entity value.
pub trait SizedValue: Value
where
    Self::Type: SizedType<Value = Self>,
{
}

/// Wrapper that created `Value` from `Entity` instance.

#[derive(RefCast)]
#[repr(transparent)]
pub struct EntityValue<E: Entity>(E);

impl<E: Entity> EntityValue<E> {
    pub fn new(entity: E) -> Self {
        EntityValue(entity)
    }

    pub fn new_ref(entity: &E) -> &Self {
        EntityValue::ref_cast(entity)
    }

    pub fn new_mut(entity: &mut E) -> &mut Self {
        EntityValue::ref_cast_mut(entity)
    }

    pub fn into_entity(self) -> E {
        self.0
    }

    pub fn entity(&self) -> &E {
        &self.0
    }

    pub fn entity_mut(&mut self) -> &mut E {
        &mut self.0
    }
}

impl<E: Entity> Value for EntityValue<E> {
    type Type = EntityType<E>;

    fn size(&self, cfg: &Config) -> usize {
        self.0.size(cfg)
    }

    fn type_of(&self) -> Self::Type {
        EntityType::new()
    }

    fn store<W: CountingWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        self.0.store(cfg, dst)
    }
}

impl<E: SizedEntity> SizedValue for EntityValue<E> {}
