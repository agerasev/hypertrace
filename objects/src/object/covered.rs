use crate::{Material, Object, Shape};
use ccgeom::Geometry;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Covered<G: Geometry, S: Shape<G>, M: Material> {
    geometry: PhantomData<G>,
    pub material: M,
    pub shape: S,
}

impl<G: Geometry, S: Shape<G>, M: Material> Covered<G, S, M> {
    pub fn new(shape: S, material: M) -> Self {
        Self {
            geometry: PhantomData,
            material,
            shape,
        }
    }
}

impl<G: Geometry, S: Shape<G>, M: Material> Object<G> for Covered<G, S, M> {
    fn shader_modules() -> crate::shader::Result<Vec<crate::shader::ShaderModule>> {
        Ok(vec![S::shader()?, M::shader()?])
    }

    fn object_node(&self) -> crate::shader::Result<crate::shader::ObjectNode> {
        Ok(crate::shader::ObjectNode::Covered {
            shape: self.shape.encode()?,
            material: self.material.encode()?,
        })
    }
}
