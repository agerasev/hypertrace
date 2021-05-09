use std::marker::PhantomData;
use crate::{Config, Entity, SizedEntity};

/// Minimal size specialization using autoderef technique.
pub struct MinSize<T: Entity>(PhantomData<T>);

impl<T: Entity> Default for MinSize<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub trait MinSizeForEntity {
    fn min_size(&self, cfg: &Config) -> usize;
}

impl<T: Entity> MinSizeForEntity for MinSize<T> {
    fn min_size(&self, _: &Config) -> usize {
        1 // FIXME: Is this correct?
    }
}

pub trait MinSizeForSizedEntity {
    fn min_size(&self, cfg: &Config) -> usize;
}

impl<T: SizedEntity> MinSizeForSizedEntity for &MinSize<T> {
    fn min_size(&self, cfg: &Config) -> usize {
        T::type_size(cfg)
    }
}
