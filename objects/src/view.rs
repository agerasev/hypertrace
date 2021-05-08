use ccgeom::Geometry3;
use types::{Named, Entity, SizedEntity, Sourced, Config, Map, source::{SourceTree, SourceBuilder, include}, include_template};
use std::{marker::PhantomData};
use type_macros::SizedEntity;

pub trait View<G: Geometry3>: SizedEntity + Sourced {}

#[derive(Clone, Debug, SizedEntity)]
pub struct PointView<G: Geometry3 + Named> {
    pub fov: f64,
    phantom: PhantomData<G>,
}

impl<G: Geometry3 + Named> PointView<G> {
    pub fn new(fov: f64) -> Self {
        Self { fov, phantom: PhantomData }
    }
}

impl<G: Geometry3 + Named> Named for PointView<G> {
    fn type_name(cfg: &Config) -> String {
        format!("PointView{}", G::type_name(cfg))
    }
    fn type_prefix(cfg: &Config) -> String {
        format!("point_view_{}", G::type_prefix(cfg))
    }
}

impl<G: Geometry3 + Sourced> Sourced for PointView<G> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("view/point/{}.hh", &G::type_prefix(cfg)))
        .tree(G::source(cfg))
        .build()
    }
}

impl<G: Geometry3 + Sourced> View<G> for PointView<G> {}

#[derive(Clone, Debug, SizedEntity)]
pub struct MappedView<G: Geometry3, V: View<G>, M: Map<G::Pos, G::Dir>> {
    pub inner: V,
    pub map: M,
    phantom: PhantomData<G>,
}

impl<
    G: Geometry3,
    V: View<G>,
    M: Map<G::Pos, G::Dir>,
> MappedView<G, V, M> {
    pub fn new(inner: V, map: M) -> Self {
        Self { inner, map, phantom: PhantomData }
    }
}

impl<
    G: Geometry3,
    V: View<G>,
    M: Map<G::Pos, G::Dir>,
> Named for MappedView<G, V, M> {
    fn type_name(_: &Config) -> String {
        format!("MappedView{}", Self::type_tag())
    }
    fn type_prefix(_: &Config) -> String {
        format!("mapped_view_{}", Self::type_tag())
    }
}

impl<
    G: Geometry3 + Sourced,
    V: View<G> + Sourced,
    M: Map<G::Pos, G::Dir> + Sourced,
> Sourced for MappedView<G, V, M> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/mapped_view_{}.hh", Self::type_tag()))
            .tree(Self::type_source(cfg))
            .tree(G::source(cfg))
            .tree(M::source(cfg))
            .tree(V::source(cfg))
            .content(&include(&format!("geometry/ray_{}.hh", &G::type_prefix(cfg))))
            .content(&include_template!(
                "view/mapped.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::type_prefix(cfg),
                "Map": &M::type_name(cfg),
                "map": &M::type_prefix(cfg),
                "View": &V::type_name(cfg),
                "view": &V::type_prefix(cfg),
            ))
            .build()
    }
}

impl<
    G: Geometry3 + Sourced,
    V: View<G> + Sourced,
    M: Map<G::Pos, G::Dir> + Sourced,
> View<G> for MappedView<G, V, M> {}
