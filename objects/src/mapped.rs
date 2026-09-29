use crate::shader::{Geometry, RenderMap};
use std::marker::PhantomData;

/// Apply a map whose geometry matches the component's construction geometry.
///
/// A map from another curvature is rejected when constructing the wrapper:
/// ```compile_fail
/// use ccgeom::{Flat3, Geometry3, Spherical3};
/// use hypertrace_objects::{Mapped, shape::Sphere};
/// let _ = Mapped::<Flat3, _, _>::new(Sphere, Spherical3::shift_x(1.0));
/// ```
#[derive(Clone)]
pub struct Mapped<G: Geometry, T, M: RenderMap<G> + 'static> {
    geometry: PhantomData<G>,
    pub map: M,
    pub inner: T,
}

impl<G: Geometry, T, M: RenderMap<G> + 'static> Mapped<G, T, M> {
    pub fn new(inner: T, map: M) -> Self {
        Self {
            inner,
            map,
            geometry: PhantomData,
        }
    }
}
