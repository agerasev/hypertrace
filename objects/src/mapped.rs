use ccgeom::{Geometry, Map};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Mapped<G: Geometry, T, M: Map<G::Pos, G::Dir> + 'static> {
    geometry: PhantomData<G>,
    pub map: M,
    pub inner: T,
}

impl<G: Geometry, T, M: Map<G::Pos, G::Dir> + 'static> Mapped<G, T, M> {
    pub fn new(inner: T, map: M) -> Self {
        Self {
            inner,
            map,
            geometry: PhantomData,
        }
    }
}
