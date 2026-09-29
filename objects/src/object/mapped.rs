use crate::{Mapped, Object};
use ccgeom::Geometry;

impl<G: Geometry, T: Object<G>, M: crate::shader::RenderMap<G>> Object<G> for Mapped<G, T, M> {
    fn shader_modules() -> crate::shader::Result<Vec<crate::shader::ShaderModule>> {
        T::shader_modules()
    }

    fn object_node(&self) -> crate::shader::Result<crate::shader::ObjectNode> {
        Ok(crate::shader::ObjectNode::Mapped {
            map: crate::shader::transform::<G, M>(&self.map)?,
            inner: Box::new(self.inner.object_node()?),
        })
    }
}
