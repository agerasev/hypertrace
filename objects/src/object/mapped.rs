use crate::{Mapped, Object};
use ccgeom::{Geometry, Map};

impl<G: Geometry, T: Object<G>, M: Map<G::Pos, G::Dir> + 'static> Object<G> for Mapped<G, T, M> {
    fn wgsl_register(registry: &mut crate::wgsl::Registry) -> crate::wgsl::Result<()> {
        T::wgsl_register(registry)
    }

    fn wgsl_object(&self) -> crate::wgsl::Result<crate::wgsl::ObjectNode> {
        Ok(crate::wgsl::ObjectNode::Mapped {
            map: crate::wgsl::transform::<G, M>(&self.map)?,
            inner: Box::new(self.inner.wgsl_object()?),
        })
    }
}
