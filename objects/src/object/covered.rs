use crate::{Material, Object, Shape};
use std::marker::PhantomData;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config,
};

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity, EntitySource)]
pub struct Covered<G: Geometry, S: Shape<G>, M: Material> {
    geometry: PhantomData<G>,
    #[getter]
    pub material: M,
    #[getter]
    pub shape: S,
}

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity, EntitySource)]
pub struct Cache<G: Geometry> {
    pub normal: G::Dir,
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

impl<G: Geometry, S: Shape<G>, M: Material> Object<G> for Covered<G, S, M>
where
    Self: Entity,
{
    type Cache = Cache<G>;

    fn object_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::object_name().1))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(S::shape_source(cfg))
            .tree(M::material_source(cfg))
            .tree(Self::Cache::source(cfg))
            .content(&include(&format!(
                "geometry/ray_{}.hh",
                G::geometry_name().1,
            )))
            .content(&include(&format!(
                "render/light/{}.hh",
                G::geometry_name().1,
            )))
            .content(&include_template!(
                "object/covered.inl",
                ("Self", "self") => Self::object_name(),
                ("Geo", "geo") => G::geometry_name(),
                ("Shape", "shape") => S::shape_name(),
                ("Material", "material") => M::material_name(),
                ("Cache", "cache") => Self::Cache::name(),
            ))
            .build()
    }
}
