use std::marker::PhantomData;
use type_macros::*;
use types::{prelude::*, Config, source::{SourceTree, SourceBuilder, include}, include_template};
use crate::{Shape};
use ccgeom::Geometry3;

#[derive(Clone, Debug, Default, Entity)]
pub struct ShapeVector<G: Geometry3, T: Shape<G>> {
    geometry: PhantomData<G>,
    pub shapes: Vec<T>,
}

impl<G: Geometry3, T: Shape<G>> ShapeVector<G, T> {
    pub fn new() -> Self {
        Self { geometry: PhantomData, shapes: Vec::new() }
    }
    pub fn from_shapes(shapes: Vec<T>) -> Self {
        Self { geometry: PhantomData, shapes }
    }
}

impl<G: Geometry3, T: Shape<G>> Named for ShapeVector<G, T> {
    fn type_name(_: &Config) -> String {
        format!("ShapeVector{}", Self::type_tag())
    }
    fn type_prefix(_: &Config) -> String {
        format!("shape_vector_{}", Self::type_tag())
    }
}

impl<G: Geometry3 + Sourced, T: Shape<G>> Sourced for ShapeVector<G, T> where Self: Entity {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::type_prefix(cfg)))
            .tree(Self::type_source(cfg))
            .tree(G::source(cfg))
            .tree(T::source(cfg))
            .content(&include(&format!(
                "render/light/{}.hh",
                G::type_prefix(cfg)
            )))
            .content(&include_template!(
                "shape/vector.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::type_prefix(cfg),
                "Shape": &T::type_name(cfg),
                "shape": &T::type_prefix(cfg),
            ))
            .build()
    }
}

impl<G: Geometry3 + Sourced, T: Shape<G>> Shape<G> for ShapeVector<G, T> where Self: Entity {}
