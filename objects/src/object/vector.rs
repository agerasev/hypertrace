use crate::Object;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{include, type_alias, SourceBuilder, SourceTree},
    Config,
};

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity, EntitySource)]
pub struct Cache<T: SizedEntity> {
    pub index: usize,
    pub inner: T,
}

impl<G: Geometry, T: Object<G>> Object<G> for Vec<T>
where
    Self: Entity,
{
    type Cache = Cache<T::Cache>;

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
            .tree(Self::Cache::source(cfg))
            .content(&include(&format!(
                "render/light/{}.hh",
                G::geometry_name().1,
            )))
            .content(&type_alias(&Self::object_name().0, &Self::name().0))
            .content(&include_template!(
                "object/vector.inl",
                ("Self", "self") => Self::object_name(),
                ("Base", "base") => Self::name(),
                ("Cache", "cache") => Self::Cache::name(),
                ("Geo", "geo") => G::geometry_name(),
                ("Object", "object") => T::object_name(),
                ("ObjectCache", "object_cache") => T::Cache::name(),
            ))
            .build()
    }
}
