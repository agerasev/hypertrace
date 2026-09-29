use crate::Object;
use ccgeom::Geometry;

impl<G: Geometry, T: Object<G>> Object<G> for Vec<T> {
    fn shader_modules() -> crate::shader::Result<Vec<crate::shader::ShaderModule>> {
        T::shader_modules()
    }

    fn object_node(&self) -> crate::shader::Result<crate::shader::ObjectNode> {
        Ok(crate::shader::ObjectNode::Vector(
            self.iter()
                .map(T::object_node)
                .collect::<crate::shader::Result<Vec<_>>>()?,
        ))
    }
}
