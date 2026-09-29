use crate::{
    object::tiling::{self, Tiling},
    Material, Object,
};
use ccgeom::Hyperboloid3;
use std::marker::PhantomData;

pub trait HorosphereTiling: Tiling {}
impl HorosphereTiling for tiling::Uniform {}
impl HorosphereTiling for tiling::Square {}
impl HorosphereTiling for tiling::Hexagonal {}

#[derive(Clone, Copy, Debug)]
pub struct TiledHorosphere<M: Material, K: HorosphereTiling, const N: usize> {
    tiling: PhantomData<K>,
    pub materials: [M; N],
    pub border_material: M,
    pub cell_size: f64,
    pub border_width: f64,
}

impl<M: Material, K: HorosphereTiling, const N: usize> TiledHorosphere<M, K, N> {
    pub fn new(materials: [M; N], cell_size: f64, border_width: f64, border_material: M) -> Self {
        Self {
            tiling: PhantomData,
            materials,
            border_material,
            cell_size,
            border_width,
        }
    }
}

impl<M: Material, K: HorosphereTiling, const N: usize> Object<Hyperboloid3>
    for TiledHorosphere<M, K, N>
{
    fn shader_modules() -> crate::shader::Result<Vec<crate::shader::ShaderModule>> {
        Ok(vec![
            crate::shape::horosphere_schema(),
            tiling::tiled_schema(K::shader(), vec![M::shader()?; N], M::shader()?)?,
        ])
    }
    fn object_node(&self) -> crate::shader::Result<crate::shader::ObjectNode> {
        Ok(crate::shader::ObjectNode::Covered {
            shape: crate::shape::horosphere(),
            material: tiling::tiled(
                K::shader(),
                self.materials
                    .iter()
                    .map(M::encode)
                    .collect::<crate::shader::Result<Vec<_>>>()?,
                self.border_material.encode()?,
                if K::uses_cell_size() {
                    self.cell_size
                } else {
                    1.0
                },
                self.border_width,
            )?,
        })
    }
}
