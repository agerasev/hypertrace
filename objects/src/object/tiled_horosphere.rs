use crate::{
    object::tiling::{self, Tiling},
    Material, Object,
};
use ccgeom::Hyperboloid3;
use std::marker::PhantomData;

pub trait HorosphereTiling: Tiling<Hyperboloid3> {}
impl HorosphereTiling for tiling::Uniform {}
impl HorosphereTiling for tiling::Square {}
impl HorosphereTiling for tiling::Hexagonal {}

#[derive(Clone, Copy, Debug)]
pub struct TiledHorosphere<M: Material<Hyperboloid3>, K: HorosphereTiling, const N: usize> {
    tiling: PhantomData<K>,
    pub materials: [M; N],
    pub border_material: M,
    pub cell_size: f64,
    pub border_width: f64,
}

impl<M: Material<Hyperboloid3>, K: HorosphereTiling, const N: usize> TiledHorosphere<M, K, N> {
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

impl<M: Material<Hyperboloid3>, K: HorosphereTiling, const N: usize> Object<Hyperboloid3>
    for TiledHorosphere<M, K, N>
{
    fn shader_modules() -> crate::shader::Result<crate::shader::Modules<Hyperboloid3>> {
        Ok(crate::shader::Modules {
            shapes: vec![crate::shape::horosphere_schema()],
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
            shape: crate::shape::horosphere(),
            material: tiling::tiled(
                K::shader(),
                self.materials
                    .iter()
                    .map(M::encode)
                    .collect::<crate::shader::Result<Vec<_>>>()?,
                self.border_material.encode()?,
                if K::USES_CELL_SIZE {
                    self.cell_size
                } else {
                    1.0
                },
                self.border_width,
            )?,
        });
        Ok(())
    }
}
