use crate::shader::Geometry;
use crate::Object;

impl<G: Geometry, T: Object<G>> Object<G> for Vec<T> {
    fn shader_modules() -> crate::shader::Result<crate::shader::Modules<G>> {
        T::shader_modules()
    }

    fn encode_objects(
        &self,
        outer: crate::shader::Transform<G>,
        output: &mut Vec<crate::shader::EncodedObject<G>>,
    ) -> crate::shader::Result<()> {
        for object in self {
            object.encode_objects(outer, output)?;
        }
        Ok(())
    }
}
