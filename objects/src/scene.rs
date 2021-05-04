use types::{Config, Entity, SizedEntity, Sourced, source::{SourceInfo, SourceBuilder}, include_template};
use type_macros::SizedEntity;
use ccgeom::Geometry3;
use crate::{View, Shape};
use std::marker::PhantomData;

#[derive(Clone, Debug, SizedEntity)]
pub struct Scene<G: Geometry3 + Sourced, V: View<G>, T: Shape<G>> {
    pub view: V,
    pub object: T,
    phantom: PhantomData<G>,
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Shape<G>> Scene<G, V, T> {
    pub fn new(view: V, shape: T) -> Self {
        Self { view, object: shape, phantom: PhantomData }
    }
}

impl<G: Geometry3 + Sourced, V: View<G>, T: Shape<G>> Sourced for Scene<G, V, T> where T: SizedEntity {
    fn source(cfg: &Config) -> SourceInfo {
        let src = Self::entity_source(cfg);
        let gsrc = G::source(cfg);
        let vsrc = V::source(cfg);
        let osrc = T::source(cfg);
        SourceBuilder::new(format!("generated/scene_{}.hh", Self::type_tag()))
            .tree(src.tree)
            .tree(gsrc.tree)
            .tree(vsrc.tree)
            .tree(osrc.tree)
            .content(&include_template!(
                "render/scene.inl",
                "Self": &src.name,
                "self": &src.prefix,
                "Geo": &gsrc.name,
                "geo": &gsrc.prefix,
                "View": &vsrc.name,
                "view": &vsrc.prefix,
                "Object": &osrc.name,
                "object": &osrc.prefix,
            ))
            .build(src.name, src.prefix)
    }
}
