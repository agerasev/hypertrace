use ccgeom::Geometry3;
use types::{Entity, SizedEntity, Sourced, Config, source::{SourceInfo, SourceBuilder, include}, include_template};
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
        let geo = G::source(cfg);
        SourceInfo::with_root(
            format!("PointView{}", &geo.name),
            format!("point_view_{}", &geo.prefix),
            "view/point.hh",
        )
    }
}

impl<G: Geometry3 + Sourced> View<G> for PointView<G> {}

#[derive(Clone, Debug, SizedEntity)]
pub struct MappedView<G: Geometry3, V: View<G>> {
    map: G::Map,
    inner: V,
}

impl<G: Geometry3 + Sourced, V: View<G> + Sourced> Sourced for MappedView<G, V> where G::Map: SizedEntity {
    fn source(cfg: &Config) -> SourceInfo {
        let gsrc = G::source(cfg);
        let vsrc = V::source(cfg);
        let name = format!("MappedView{}{}", &gsrc.name, Self::type_tag());
        let prefix = format!("mapped_view_{}_{}", &gsrc.prefix, Self::type_tag());

        SourceBuilder::new(format!("generated/mapped_view_{}.hh", Self::type_tag()))
            .tree(gsrc.tree)
            .tree(vsrc.tree)
            .content(&include("geometry/ray.hh"))
            .content(&include_template!(
                "view/mapped.inl",
                "Self": &name,
                "self": &prefix,
                "View": &vsrc.name,
                "view": &vsrc.prefix,
                "Geo": &gsrc.name,
                "geo": &gsrc.prefix,
            ))
            .build(name, prefix)
    }
}

impl<G: Geometry3 + Sourced, V: View<G> + Sourced> View<G> for MappedView<G, V> where G::Map: SizedEntity {}
