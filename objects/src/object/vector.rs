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
    fn object_prefix(cfg: &Config) -> String {
        format!("object_{}", Self::type_prefix(cfg))
    }
    fn object_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::object_prefix(cfg)))
            .tree(Self::source(cfg))
            .tree(G::geometry_source(cfg))
            .tree(T::object_source(cfg))
            .content(&include(&format!(
                "render/light/{}.hh",
                G::geometry_prefix(cfg)
            )))
            .content(&include_template!(
                "object/vector.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::object_prefix(cfg),
                "self_data": &Self::type_prefix(cfg),
                "Geo": &G::type_name(cfg),
                "geo": &G::geometry_prefix(cfg),
                "Object": &T::type_name(cfg),
                "object": &T::object_prefix(cfg),
            ))
            .build()
    }
}
