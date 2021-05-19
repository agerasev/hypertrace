use crate::Object;
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config,
};

impl<G: Geometry, T: Object<G>> Object<G> for Vec<T>
where
    Self: Entity,
{
    fn object_name() -> (String, String) {
        (
            format!("Object{}", Self::name().0),
            format!("object_{}", Self::name().1),
        )
    }
    fn object_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::object_name().1))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(T::object_source(cfg))
            .content(&include(&format!(
                "render/light/{}.hh",
                G::geometry_name().1,
            )))
            .content(&include_template!(
                "object/vector.inl",
                ("Self", "self") => Self::object_name(),
                ("Base", "base") => Self::name(),
                ("Geo", "geo") => G::geometry_name(),
                ("Object", "object") => T::object_name(),
            ))
            .build()
    }
}
