use crate::{
    object::tiling::{self, Tiling},
    shape::Plane,
    Material, Object, Shape,
};
use base::ccgeom::Hyperbolic3;
use std::marker::PhantomData;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config,
};

pub trait PlaneTiling<G: Geometry>: Tiling {
    fn name() -> String;
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Err(crate::wgsl::unsupported::<Self>())
    }
}
impl PlaneTiling<Hyperbolic3> for tiling::Uniform {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Uniform)
    }
    fn name() -> String {
        "PLANE_HY_TILING_UNIFORM".into()
    }
}
impl PlaneTiling<Hyperbolic3> for tiling::Pentagonal {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Pentagonal)
    }
    fn name() -> String {
        "PLANE_HY_TILING_PENTAGONAL".into()
    }
}
impl PlaneTiling<Hyperbolic3> for tiling::Pentastar {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Pentastar)
    }
    fn name() -> String {
        "PLANE_HY_TILING_PENTASTAR".into()
    }
}

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity, EntitySource)]
pub struct TiledPlane<M: Material, K: Tiling, const N: usize> {
    tiling: PhantomData<K>,
    pub materials: [M; N],
    pub border_material: M,
    pub cell_size: f64,
    pub border_width: f64,
}

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity, EntitySource)]
pub struct TiledPlaneCache<G: Geometry> {
    pub normal: G::Dir,
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
    type Cache = TiledPlaneCache<Hyperbolic3>;

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

    fn object_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::object_name().1))
            .tree(Self::source(cfg))
            .tree(Self::Cache::source(cfg))
            .tree(M::material_source(cfg))
            .tree(<Plane as Shape<Hyperbolic3>>::shape_source(cfg))
            .content(&include("render/light/hy.hh"))
            .content(&include_template!(
                "object/hy/tiled_plane/impl.inl",
                ("Self", "self") => Self::object_name(),
                ("Cache", "cache") => Self::Cache::name(),
                ("Material", "material") => M::material_name(),
                "material_count" => format!("{}", N),
                "tiling_type" => K::name(),
            ))
            .build()
    }
}
