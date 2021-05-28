use crate::{Material, Shape, Object, object::tiling::{self, Tiling}, shape::Plane};
use std::marker::PhantomData;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config,
};
use base::ccgeom::{Hyperbolic3};

pub trait PlaneTiling<G: Geometry>: Tiling {
    fn name() -> String;
}
impl PlaneTiling<Hyperbolic3> for tiling::Uniform {
    fn name() -> String {
        "PLANE_HY_TILING_UNIFORM".into()
    }
}
impl PlaneTiling<Hyperbolic3> for tiling::Pentagonal {
    fn name() -> String {
        "PLANE_HY_TILING_PENTAGONAL".into()
    }
}
impl PlaneTiling<Hyperbolic3> for tiling::Pentastar {
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
    pub fn new(
        materials: [M; N],
        cell_size: f64,
        border_width: f64,
        border_material: M,
    ) -> Self {
        Self {
            tiling: PhantomData,
            materials,
            border_material,
            cell_size,
            border_width,
        }
    }
}

impl<M: Material, K: PlaneTiling<Hyperbolic3>, const N: usize> Object<Hyperbolic3> for TiledPlane<M, K, N> {
    type Cache = TiledPlaneCache<Hyperbolic3>;

    fn object_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::object_name().1))
            .tree(Self::source(cfg))
            .tree(Self::Cache::source(cfg))
            .tree(M::material_source(cfg))
            .tree(<Plane as Shape<Hyperbolic3>>::shape_source(cfg))
            .content(&include(&"render/light/hy.hh"))
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
