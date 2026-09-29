use crate::shader::Geometry;
use crate::{Mapped, Object};

impl<G: Geometry, T: Object<G>, M: crate::shader::RenderMap<G>> Object<G> for Mapped<G, T, M> {
    fn shader_modules() -> crate::shader::Result<crate::shader::Modules<G>> {
        T::shader_modules()
    }

    fn encode_objects(
        &self,
        outer: crate::shader::Transform<G>,
        output: &mut Vec<crate::shader::EncodedObject<G>>,
    ) -> crate::shader::Result<()> {
        self.inner.encode_objects(
            outer.chain(&crate::shader::transform::<G, M>(&self.map)?)?,
            output,
        )
    }
}
