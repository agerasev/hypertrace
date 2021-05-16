#[macro_export]
macro_rules! object_union {
    { $self:ident { $( $variant:ident($vtype:ty) ),* $(,)? } } => {
        #[derive(type_macros::EntityId, type_macros::Entity, type_macros::SizedEntity)]
        pub enum $self {
            $( $variant($vtype) ),*
        }

        $(
            impl From<$vtype> for $self {
                fn from(var: $vtype) -> Self {
                    $self::$variant(var)
                }
            }
        )*

        impl<G: types::Geometry> $crate::Object<G> for $self
        where
            Self: types::Entity,
            $(
                $vtype: $crate::Object<G>,
            ),*
        {
        }

        impl types::EntitySource for $self
        where
            Self: types::Entity,
            $(
                $vtype: types::Entity,
            ),*
        {
            fn data_source(cfg: &types::Config) -> types::source::SourceTree {
                /*
                SourceBuilder::new(format!("generated/object_union_{}.hh", Self::tag()))
                    .tree(Self::type_source(cfg))
                    .tree(G::data_source(cfg))
                    .tree(S::data_source(cfg))
                    .tree(M::data_source(cfg))
                    .content(&include(&format!(
                        "geometry/ray_{}.hh",
                        G::data_prefix()
                    )))
                    .content(&include(&format!(
                        "render/light/{}.hh",
                        G::data_prefix()
                    )))
                    .content(&include_template!(
                        "object/covered.inl",
                        "Self": &Self::name(),
                        "self": &Self::data_prefix(),
                        "Geo": &G::name(),
                        "geo": &G::data_prefix(),
                        "Shape": &S::name(),
                        "shape": &S::data_prefix(),
                        "Material": &M::name(),
                        "material": &M::data_prefix(),
                    ))
                    .build()
                */
                types::source::SourceTree::new("abc")
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{object_union, object::Covered, shape::euclidean::Sphere, material::Lambertian};
    use ccgeom::Euclidean3;

    object_union! {
        TestUnion {
            A(Covered<Euclidean3, Sphere, Lambertian>),
        }
    }
}
