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
    fn object_prefix() -> String {
        format!("object_{}", Self::data_prefix())
    }
    fn object_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::object_prefix()))
            .tree(Self::data_source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(T::object_source(cfg))
            .content(&include(&format!(
                "render/light/{}.hh",
                G::geometry_prefix()
            )))
            .content(&include_template!(
                "object/vector.inl",
                "Self": &Self::name(),
                "self": &Self::object_prefix(),
                "self_data": &Self::data_prefix(),
                "Geo": &G::name(),
                "geo": &G::geometry_prefix(),
                "Object": &T::name(),
                "object": &T::object_prefix(),
            ))
            .build()
    }
}
