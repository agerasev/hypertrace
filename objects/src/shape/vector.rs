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
    fn shape_prefix() -> String {
        format!("shape_{}", Self::data_prefix())
    }
    fn shape_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::shape_prefix()))
            .tree(Self::data_source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(T::shape_source(cfg))
            .content(&include(&format!(
                "render/light/{}.hh",
                G::geometry_prefix()
            )))
            .content(&include_template!(
                "shape/vector.inl",
                "Self": &Self::name(),
                "self": &Self::shape_prefix(),
                "self_data": &Self::data_prefix(),
                "Geo": &G::name(),
                "geo": &G::geometry_prefix(),
                "Shape": &T::name(),
                "shape": &T::shape_prefix(),
            ))
            .build()
    }
}
