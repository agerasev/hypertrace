use std::marker::PhantomData;
use type_macros::*;
use types::{prelude::*, Map};

#[derive(Clone, EntityId, Entity, SizedEntity, EntitySource)]
pub struct Mapped<G: Geometry, T: Entity, M: Map<G::Pos, G::Dir>> {
    geometry: PhantomData<G>,
    pub map: M,
    #[getter]
    pub inner: T,
}

impl<G: Geometry, T: Entity, M: Map<G::Pos, G::Dir>> Mapped<G, T, M> {
    pub fn new(inner: T, map: M) -> Self {
        Self {
            inner,
            map,
            geometry: PhantomData,
        }
    }
}
