use crate::{
    object::tiling::{self, Tiling},
    Material, Object,
};
use ccgeom::Hyperbolic3;
use std::marker::PhantomData;

pub trait HorosphereTiling: Tiling {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Err(crate::wgsl::unsupported::<Self>())
    }
}
impl HorosphereTiling for tiling::Uniform {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Uniform)
    }
}
impl HorosphereTiling for tiling::Square {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Square)
    }
}
impl HorosphereTiling for tiling::Hexagonal {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Hexagonal)
    }
}

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

impl<M: Material, K: HorosphereTiling, const N: usize> Object<Hyperbolic3>
    for TiledHorosphere<M, K, N>
{
    fn wgsl_register(registry: &mut crate::wgsl::Registry) -> crate::wgsl::Result<()> {
        registry.shape(crate::wgsl::ShapeSchema::Horosphere);
        registry.material(M::wgsl_material_schema()?);
        Ok(())
    }

    fn wgsl_object(&self) -> crate::wgsl::Result<crate::wgsl::ObjectNode> {
        let tiling = K::wgsl_tiling()?;
        Ok(crate::wgsl::ObjectNode::Tiled {
            shape: crate::wgsl::ShapeValue::horosphere(),
            materials: self
                .materials
                .iter()
                .map(M::wgsl_material)
                .collect::<crate::wgsl::Result<Vec<_>>>()?,
            border_material: self.border_material.wgsl_material()?,
            tiling,
            cell_size: if tiling == crate::wgsl::Tiling::Uniform {
                1.0
            } else {
                self.cell_size
            },
            border_width: self.border_width,
        })
    }
}
