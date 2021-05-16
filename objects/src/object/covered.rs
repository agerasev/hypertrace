use crate::{Material, Object, Shape};
use std::marker::PhantomData;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config,
};

#[derive(Clone, Copy, Debug, Named, Entity, SizedEntity, Sourced)]
pub struct Covered<G: Geometry, S: Shape<G>, M: Material> {
    geometry: PhantomData<G>,
    #[getter]
    pub material: M,
    #[getter]
    pub shape: S,
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
    fn object_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::object_prefix(cfg)))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(S::shape_source(cfg))
            .tree(M::material_source(cfg))
            .content(&include(&format!(
                "geometry/ray_{}.hh",
                G::geometry_prefix(cfg)
            )))
            .content(&include(&format!(
                "render/light/{}.hh",
                G::geometry_prefix(cfg)
            )))
            .content(&include_template!(
                "object/covered.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::object_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::geometry_prefix(cfg),
                "Shape": &S::type_name(cfg),
                "shape": &S::shape_prefix(cfg),
                "Material": &M::type_name(cfg),
                "material": &M::material_prefix(cfg),
            ))
            .build()
    }
}
