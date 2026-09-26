use crate::{
    object::tiling::{self, Tiling},
    shape::Horosphere,
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

pub trait HorosphereTiling: Tiling {
    fn name() -> String;
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Err(crate::wgsl::unsupported::<Self>())
    }
}
impl HorosphereTiling for tiling::Uniform {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Uniform)
    }
    fn name() -> String {
        "HOROSPHERE_TILING_UNIFORM".into()
    }
}
impl HorosphereTiling for tiling::Square {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Square)
    }
    fn name() -> String {
        "HOROSPHERE_TILING_SQUARE".into()
    }
}
impl HorosphereTiling for tiling::Hexagonal {
    fn wgsl_tiling() -> crate::wgsl::Result<crate::wgsl::Tiling> {
        Ok(crate::wgsl::Tiling::Hexagonal)
    }
    fn name() -> String {
        "HOROSPHERE_TILING_HEXAGONAL".into()
    }
}

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct TiledHorosphere<M: Material, K: HorosphereTiling, const N: usize> {
    tiling: PhantomData<K>,
    pub materials: [M; N],
    pub border_material: M,
    pub cell_size: f64,
    pub border_width: f64,
}

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct TiledHorosphereCache {
    pub normal: <Hyperbolic3 as Geometry>::Dir,
}

impl EntitySource for TiledHorosphereCache {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("object/hy/tiled_horosphere/def.hh")
    }
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

impl<M: Material, K: HorosphereTiling, const N: usize> EntitySource for TiledHorosphere<M, K, N> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::object_name().1))
            .tree(M::material_source(cfg))
            .tree(Horosphere::shape_source(cfg))
            .content(&include("render/light/hy.hh"))
            .content(&include_template!(
                "object/hy/tiled_horosphere/impl.inl",
                ("Self", "self") => Self::object_name(),
                ("Material", "material") => M::material_name(),
                "material_count" => format!("{}", N),
                "tiling_type" => K::name(),
            ))
            .build()
    }
}

impl<M: Material, K: HorosphereTiling, const N: usize> Object<Hyperbolic3>
    for TiledHorosphere<M, K, N>
{
    type Cache = TiledHorosphereCache;

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
