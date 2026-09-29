use crate::shader::Geometry;
use crate::{Material, Object, Shape};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Covered<G: Geometry, S: Shape<G>, M: Material<G>> {
    geometry: PhantomData<G>,
    pub material: M,
    pub shape: S,
}

impl<G: Geometry, S: Shape<G>, M: Material<G>> Covered<G, S, M> {
    pub fn new(shape: S, material: M) -> Self {
        Self {
            geometry: PhantomData,
            material,
            shape,
        }
    }
}

impl<G: Geometry, S: Shape<G>, M: Material<G>> Object<G> for Covered<G, S, M> {
    fn shader_modules() -> crate::shader::Result<crate::shader::Modules<G>> {
        Ok(crate::shader::Modules {
            shapes: vec![S::shader()?],
            materials: vec![M::shader()?],
            libraries: vec![],
        })
    }

    fn encode_objects(
        &self,
        outer: crate::shader::Transform<G>,
        output: &mut Vec<crate::shader::EncodedObject<G>>,
    ) -> crate::shader::Result<()> {
        output.push(crate::shader::EncodedObject {
            map: outer,
            shape: self.shape.encode()?,
            material: self.material.encode()?,
        });
        Ok(())
    }
}
