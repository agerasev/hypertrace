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
    fn wgsl_register(registry: &mut crate::wgsl::Registry) -> crate::wgsl::Result<()> {
        registry.shape(S::wgsl_shape_schema()?);
        registry.material(M::wgsl_material_schema()?);
        Ok(())
    }

    fn wgsl_object(&self) -> crate::wgsl::Result<crate::wgsl::ObjectNode> {
        Ok(crate::wgsl::ObjectNode::Covered {
            shape: self.shape.wgsl_shape()?,
            material: self.material.wgsl_material()?,
        })
    }
}
