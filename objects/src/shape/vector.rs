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
    fn shape_name() -> (String, String) {
        (
            format!("Shape{}", Self::name().0),
            format!("shape_{}", Self::name().1),
        )
    }
    fn shape_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::shape_name().1))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(T::shape_source(cfg))
            .content(&include(&format!(
                "render/light/{}.hh",
                G::geometry_name().1
            )))
            .content(&include_template!(
                "shape/vector.inl",
                ("Self", "self") => Self::shape_name(),
                ("Base", "base") => Self::name(),
                ("Geo", "geo") => G::geometry_name(),
                ("Shape", "shape") => T::shape_name(),
            ))
            .build()
    }
}
