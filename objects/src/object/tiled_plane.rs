use crate::{
    object::tiling::{self, Tiling},
    Material, Object,
};
use ccgeom::Hyperboloid3;
use std::marker::PhantomData;

pub trait PlaneTiling: Tiling<Hyperboloid3> {}
impl PlaneTiling for tiling::Uniform {}
impl PlaneTiling for tiling::Pentagonal {}
impl PlaneTiling for tiling::Pentastar {}

#[derive(Clone, Copy, Debug)]
pub struct TiledPlane<M: Material<Hyperboloid3>, K: PlaneTiling, const N: usize> {
    tiling: PhantomData<K>,
    pub materials: [M; N],
    pub border_material: M,
    pub border_width: f64,
}

impl<M: Material<Hyperboloid3>, K: PlaneTiling, const N: usize> TiledPlane<M, K, N> {
    pub fn new(materials: [M; N], border_width: f64, border_material: M) -> Self {
        Self {
            tiling: PhantomData,
            materials,
            border_material,
            border_width,
        }
    }
}

impl<M: Material<Hyperboloid3>, K: PlaneTiling, const N: usize> Object<Hyperboloid3>
    for TiledPlane<M, K, N>
{
    fn shader_modules() -> crate::shader::Result<crate::shader::Modules<Hyperboloid3>> {
        Ok(crate::shader::Modules {
            shapes: vec![crate::shape::plane_schema()],
            materials: vec![tiling::tiled_schema(
                K::shader(),
                vec![M::shader()?; N],
                M::shader()?,
            )?],
            libraries: vec![],
        })
    }
    fn encode_objects(
        &self,
        outer: crate::shader::Transform<Hyperboloid3>,
        output: &mut Vec<crate::shader::EncodedObject<Hyperboloid3>>,
    ) -> crate::shader::Result<()> {
        output.push(crate::shader::EncodedObject {
            map: outer,
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
        });
        Ok(())
    }
}
