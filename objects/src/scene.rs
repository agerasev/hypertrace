use crate::{Background, Object, View};
use std::marker::PhantomData;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{SourceBuilder, SourceTree},
    Config,
};

pub trait Scene<G: Geometry>: EntityId + Entity + EntitySource {
    fn scene_prefix() -> String {
        Self::data_prefix()
    }
    fn scene_source(cfg: &Config) -> SourceTree {
        Self::data_source(cfg)
    }
}

#[derive(Clone, Debug, EntityId, Entity, SizedEntity, EntitySource)]
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
        SourceBuilder::new(format!("generated/scene_{}.hh", Self::tag()))
            .tree(Self::data_source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(V::view_source(cfg))
            .tree(T::object_source(cfg))
            .tree(B::background_source(cfg))
            .content(&include_template!(
                "render/scene.inl",
                "Self": Self::name(),
                "self": Self::scene_prefix(),
                "Geo": G::name(),
                "geo": G::geometry_prefix(),
                "View": V::name(),
                "view": V::view_prefix(),
                "Object": T::name(),
                "object": T::object_prefix(),
                "Background": B::name(),
                "background": B::background_prefix(),
                "light_hops": format!("{}", H),
            ))
            .build()
    }
}
