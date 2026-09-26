use crate::{
    object::tiling::{self, Tiling},
    Material, Object,
};
use ccgeom::Geometry;
use ccgeom::Hyperbolic3;
use std::marker::PhantomData;

pub trait PlaneTiling<G: Geometry>: Tiling {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Err(crate::wgsl::unsupported::<Self>())
    }
}
impl PlaneTiling<Hyperbolic3> for tiling::Uniform {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Uniform)
    }
}
impl PlaneTiling<Hyperbolic3> for tiling::Pentagonal {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Pentagonal)
    }
}
impl PlaneTiling<Hyperbolic3> for tiling::Pentastar {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Pentastar)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TiledPlane<M: Material, K: Tiling, const N: usize> {
    tiling: PhantomData<K>,
    pub materials: [M; N],
    pub border_material: M,
    pub cell_size: f64,
    pub border_width: f64,
}

impl<M: Material, K: Tiling, const N: usize> TiledPlane<M, K, N> {
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

impl<M: Material, K: PlaneTiling<Hyperbolic3>, const N: usize> Object<Hyperbolic3>
    for TiledPlane<M, K, N>
{
    fn wgsl_register(registry: &mut crate::wgsl::Registry) -> crate::wgsl::Result<()> {
        registry.shape(crate::wgsl::ShapeSchema::Plane);
        registry.material(M::wgsl_material_schema()?);
        Ok(())
    }

    fn wgsl_object(&self) -> crate::wgsl::Result<crate::wgsl::ObjectNode> {
        Ok(crate::wgsl::ObjectNode::Tiled {
            shape: crate::wgsl::ShapeValue::plane(),
            materials: self
                .materials
                .iter()
                .map(M::wgsl_material)
                .collect::<crate::wgsl::Result<Vec<_>>>()?,
            border_material: self.border_material.wgsl_material()?,
            tiling: K::wgsl_tiling()?,
            // Plane tilings do not use cell_size; legacy builders may store NaN.
            cell_size: 1.0,
            border_width: self.border_width,
        })
    }
}
