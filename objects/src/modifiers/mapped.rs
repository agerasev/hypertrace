use types::{Map, Entity, SizedEntity, Config, source::{Sourced, SourceTree, SourceTree}};
use type_macros::SizedEntity;
use ccgeom::{Geometry3};
use std::marker::PhantomData;
use crate::Shape;

#[derive(Clone, Copy, Debug, SizedEntity)]
struct MappedShape<G: Geometry3, M: Map<G::Pos, G::Dir> + SizedEntity, T: Shape<G> + SizedEntity> {
    phantom: PhantomData<G>,
    pub map: M,
    pub shape: T,
}

impl<G: Geometry3, M: Map<G::Pos, G::Dir> + SizedEntity, T: Shape<G> + SizedEntity> Sourced for MappedShape<G, M, T> {
    fn source(cfg: &Config) -> SourceTree {
        let shape_src = <T as Sourced>::source(cfg);
        let map_src = M::source(cfg);
        let mapped_src = Self::type_source(cfg);
        let tag = Self::type_tag();
        let name = format!("Shape{}", tag);
        let prefix = format!("shape_{}", tag);
        let file = format!("generated/shape{}.hh", &tag);
        let mut tree = SourceTree::new(file.clone().into());
        tree.insert(file.into(),
            format!("
                #include <{}>
                #include <{}>
                #include <{}>

                typedef {} {};
                typedef {} {};
            ",
            shape_src.tree.root(),
            map_src.tree.root(),
            mapped_src.tree.root(),
            &mapped_src.name,
            &name,
            &mapped_src.prefix,
            &prefix,
        )).unwrap();
        tree.append(shape_src.tree).unwrap();
        tree.append(map_src.tree).unwrap();
        tree.append(mapped_src.tree).unwrap();
        SourceTree::new(
            name,
            prefix,
            tree,
        )
    }
}

impl<G: Geometry3, M: Map<G::Pos, G::Dir> + SizedEntity, T: Shape<G> + SizedEntity> Shape<G> for MappedShape<G, M, T> {}
