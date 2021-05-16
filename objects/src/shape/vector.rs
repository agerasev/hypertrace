use crate::Shape;
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config,
};

impl<G: Geometry, T: Shape<G>> Shape<G> for Vec<T>
where
    Self: Entity,
{
    fn shape_prefix(cfg: &Config) -> String {
        format!("shape_{}", Self::type_prefix(cfg))
    }
    fn shape_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::shape_prefix(cfg)))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(T::shape_source(cfg))
            .content(&include(&format!(
                "render/light/{}.hh",
                G::geometry_prefix(cfg)
            )))
            .content(&include_template!(
                "shape/vector.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::shape_prefix(cfg),
                "self_data": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::geometry_prefix(cfg),
                "Shape": &T::type_name(cfg),
                "shape": &T::shape_prefix(cfg),
            ))
            .build()
    }
}
