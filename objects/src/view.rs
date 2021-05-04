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
pub struct MappedView<G: Geometry3, M: Map<G>, V: View<G>> {
    map: M,
    inner: V,
    phantom: PhantomData<G>,
}

impl<
    G: Geometry3 + Sourced,
    M: Map<G> + Sourced,
    V: View<G> + Sourced,
> Sourced for MappedView<G, M, V> {
    fn source(cfg: &Config) -> SourceInfo {
        let gsrc = G::source(cfg);
        let msrc = M::source(cfg);
        let vsrc = V::source(cfg);
        let name = format!("MappedView{}", Self::type_tag());
        let prefix = format!("mapped_view_{}", Self::type_tag());

        SourceBuilder::new(format!("generated/mapped_view_{}.hh", Self::type_tag()))
            .tree(gsrc.tree)
            .tree(vsrc.tree)
            .content(&include(&format!("geometry/ray_{}.hh", &gsrc.prefix)))
            .content(&include_template!(
                "view/mapped.inl",
                "Self": &name,
                "self": &prefix,
                "Geo": &gsrc.name,
                "geo": &gsrc.prefix,
                "Map": &msrc.name,
                "map": &msrc.prefix,
                "View": &vsrc.name,
                "view": &vsrc.prefix,
            ))
            .build(name, prefix)
    }
}

impl<
    G: Geometry3 + Sourced,
    M: Map<G> + Sourced,
    V: View<G> + Sourced,
> View<G> for MappedView<G, M, V> {}
