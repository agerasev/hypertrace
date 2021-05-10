use crate::Shape;
use ccgeom::Geometry3;
use std::marker::PhantomData;
use type_macros::{Entity, Named, SizedEntity};
use types::{
    include_template,
    source::{include, SourceBuilder, SourceTree, Sourced},
    Config, Entity, Map, Named, RayMap, SizedEntity,
};

#[derive(Clone, Copy, Debug, Named, Entity, SizedEntity)]
pub struct MappedShape<G: Geometry3, M: Map<G::Pos, G::Dir>, T: Shape<G>> {
    geometry: PhantomData<G>,
    pub map: M,
    #[getter]
    pub shape: T,
}

impl<G: Geometry3, M: Map<G::Pos, G::Dir>, T: Shape<G>> MappedShape<G, M, T> {
    pub fn new(shape: T, map: M) -> Self {
        Self {
            shape,
            map,
            geometry: PhantomData,
        }
    }
}

impl<G: Geometry3 + Sourced, M: Map<G::Pos, G::Dir> + Sourced, T: Shape<G> + Sourced> Sourced
    for MappedShape<G, M, T>
where
    Self: Entity,
{
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/mapped_shape_{}.hh", Self::type_tag()))
            .tree(Self::type_source(cfg))
            .tree(G::source(cfg))
            .tree(M::source(cfg))
            .tree(T::source(cfg))
            .content(&include(&format!(
                "geometry/ray_{}.hh",
                &G::type_prefix(cfg)
            )))
            .tree(RayMap::<G, M>::source(cfg))
            .content(&include_template!(
                "shape/mapped.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::type_prefix(cfg),
                "Map": &M::type_name(cfg),
                "map": &M::type_prefix(cfg),
                "Shape": &T::type_name(cfg),
                "shape": &T::type_prefix(cfg),
            ))
            .build()
    }
}

impl<G: Geometry3 + Sourced, M: Map<G::Pos, G::Dir> + SizedEntity, T: Shape<G> + SizedEntity>
    Shape<G> for MappedShape<G, M, T>
{
}
