use crate::{
    object::tiling::{self, Tiling},
    Material, Object,
};
use ccgeom::Hyperboloid3;
use std::marker::PhantomData;

pub trait PlaneTiling: Tiling {}
impl PlaneTiling for tiling::Uniform {}
impl PlaneTiling for tiling::Pentagonal {}
impl PlaneTiling for tiling::Pentastar {}

#[derive(Clone, Copy, Debug)]
pub struct TiledPlane<M: Material, K: Tiling, const N: usize> {
    tiling: PhantomData<K>,
    pub materials: [M; N],
    pub border_material: M,
    pub border_width: f64,
}

impl<M: Material, K: Tiling, const N: usize> TiledPlane<M, K, N> {
    pub fn new(materials: [M; N], border_width: f64, border_material: M) -> Self {
        Self {
            tiling: PhantomData,
            materials,
            border_material,
            border_width,
        }
    }
}

impl<M: Material, K: PlaneTiling, const N: usize> Object<Hyperboloid3> for TiledPlane<M, K, N> {
    fn shader_modules() -> crate::shader::Result<Vec<crate::shader::ShaderModule>> {
        Ok(vec![
            crate::shape::plane_schema(),
            tiling::tiled_schema(K::shader(), vec![M::shader()?; N], M::shader()?)?,
        ])
    }
    fn object_node(&self) -> crate::shader::Result<crate::shader::ObjectNode> {
        Ok(crate::shader::ObjectNode::Covered {
            shape: crate::shape::plane(),
            material: tiling::tiled(
                K::shader(),
                self.materials
                    .iter()
                    .map(M::encode)
                    .collect::<crate::shader::Result<Vec<_>>>()?,
                self.border_material.encode()?,
                1.0,
                self.border_width,
            )?,
        })
    }
}
