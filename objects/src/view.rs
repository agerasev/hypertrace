use ccgeom::Geometry3;
use types::{Entity, SizedEntity, Sourced, Config, Map, source::{SourceInfo, SourceBuilder, include}, include_template};
use std::{marker::PhantomData};
use type_macros::SizedEntity;

pub trait View<G: Geometry3>: SizedEntity + Sourced {}

#[derive(Clone, Debug, SizedEntity)]
pub struct PointView<G: Geometry3> {
    fov: f64,
    phantom: PhantomData<G>,
}

impl<G: Geometry3> PointView<G> {
    pub fn new(fov: f64) -> Self {
        Self { fov, phantom: PhantomData }
    }
}

impl<G: Geometry3 + Sourced> Sourced for PointView<G> {
    fn source(cfg: &Config) -> SourceInfo {
        let gsrc = G::source(cfg);
        SourceInfo::with_root(
            format!("PointView{}", &gsrc.name),
            format!("point_view_{}", &gsrc.prefix),
            format!("view/point/{}.hh", &gsrc.prefix),
        )
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
    G: Geometry3 + Sourced,
    V: View<G> + Sourced,
    M: Map<G::Pos, G::Dir> + Sourced,
> Sourced for MappedView<G, V, M> {
    fn source(cfg: &Config) -> SourceInfo {
        let src = Self::entity_source(cfg);
        let gsrc = G::source(cfg);
        let msrc = M::source(cfg);
        let vsrc = V::source(cfg);

        SourceBuilder::new(format!("generated/mapped_view_{}.hh", Self::type_tag()))
            .tree(src.tree)
            .tree(gsrc.tree)
            .tree(vsrc.tree)
            .content(&include(&format!("geometry/ray_{}.hh", &gsrc.prefix)))
            .content(&include_template!(
                "view/mapped.inl",
                "Self": &src.name,
                "self": &src.prefix,
                "Geo": &gsrc.name,
                "geo": &gsrc.prefix,
                "Map": &msrc.name,
                "map": &msrc.prefix,
                "View": &vsrc.name,
                "view": &vsrc.prefix,
            ))
            .build(src.name, src.prefix)
    }
}

impl<
    G: Geometry3 + Sourced,
    V: View<G> + Sourced,
    M: Map<G::Pos, G::Dir> + Sourced,
> View<G> for MappedView<G, V, M> {}
