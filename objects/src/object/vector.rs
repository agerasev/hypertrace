use crate::Object;
use ccgeom::Geometry;

impl<G: Geometry, T: Object<G>> Object<G> for Vec<T> {
    fn wgsl_register(registry: &mut crate::wgsl::Registry) -> crate::wgsl::Result<()> {
        T::wgsl_register(registry)
    }

    fn wgsl_object(&self) -> crate::wgsl::Result<crate::wgsl::ObjectNode> {
        Ok(crate::wgsl::ObjectNode::Vector(
            self.iter()
                .map(T::wgsl_object)
                .collect::<crate::wgsl::Result<Vec<_>>>()?,
        ))
    }
}
