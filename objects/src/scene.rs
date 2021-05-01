use types::{Config, Entity, SizedEntity, Sourced, source::{SourceInfo, SourceTree}};
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
        let sources = vec![
            Self::entity_source(cfg),
            G::source(cfg),
            V::source(cfg),
            T::source(cfg),
        ];
        let mut content = String::new();
        let file = format!("generated/scene_{}.hh", Self::type_tag());
        let mut tree = SourceTree::new(file.clone().into());
        for src in sources.into_iter() {
            content += &format!("#include <{}>\n", src.tree.root());
            tree.append(src.tree).unwrap();
        }
        let src = Self::entity_source(cfg);
        let name = format!("Scene{}", Self::type_tag());
        let prefix = format!("scene_{}", Self::type_tag());
        content += &format!("typedef {} {};", &src.name, &name);
        tree.insert(file.into(), content).unwrap();
        SourceInfo::new(
            name,
            prefix,
            tree,
        )
    }
}
