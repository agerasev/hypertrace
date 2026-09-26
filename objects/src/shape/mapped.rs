use crate::{Mapped, Shape};
use ccgeom::{Geometry, Map};

impl<G: Geometry, T: Shape<G>, M: Map<G::Pos, G::Dir> + 'static> Shape<G> for Mapped<G, T, M> {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        Ok(crate::wgsl::ShapeSchema::Mapped {
            geometry: crate::wgsl::geometry::<G>()?,
            inner: Box::new(T::wgsl_shape_schema()?),
        })
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        self.inner
            .wgsl_shape()?
            .mapped(crate::wgsl::transform::<G, M>(&self.map)?)
    }
}
