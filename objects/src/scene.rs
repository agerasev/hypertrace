use crate::{Background, Object, View};
use std::marker::PhantomData;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{SourceBuilder, SourceTree},
    Config,
};

pub trait Scene<G: Geometry>: Named + Entity + Sourced {
    fn scene_prefix(cfg: &Config) -> String {
        Self::type_prefix(cfg)
    }
    fn scene_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

#[derive(Clone, Debug, Named, Entity, SizedEntity, Sourced)]
pub struct SceneImpl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>, const H: usize> {
    geometry: PhantomData<G>,
    #[getter]
    pub view: V,
    #[getter]
    pub background: B,
    #[getter]
    pub object: T,
}

impl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>, const H: usize>
    SceneImpl<G, V, T, B, H>
{
    pub fn new(view: V, object: T, background: B) -> Self {
        Self {
            view,
            background,
            object,
            geometry: PhantomData,
        }
    }
}

impl<G: Geometry, V: View<G>, T: Object<G>, B: Background<G>, const H: usize> Scene<G>
    for SceneImpl<G, V, T, B, H>
where
    Self: Entity,
{
    fn scene_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/scene_{}.hh", Self::type_tag()))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(V::view_source(cfg))
            .tree(T::object_source(cfg))
            .tree(B::background_source(cfg))
            .content(&include_template!(
                "render/scene.inl",
                "Self": Self::type_name(cfg),
                "self": Self::scene_prefix(cfg),
                "Geo": G::type_name(cfg),
                "geo": G::geometry_prefix(cfg),
                "View": V::type_name(cfg),
                "view": V::view_prefix(cfg),
                "Object": T::type_name(cfg),
                "object": T::object_prefix(cfg),
                "Background": B::type_name(cfg),
                "background": B::background_prefix(cfg),
                "light_hops": format!("{}", H),
            ))
            .build()
    }
}
