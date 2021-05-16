#[macro_export]
macro_rules! object_union {
    { $self:ident { $( $variant:ident($vtype:ty) ),* $(,)? } } => {
        #[derive(type_macros::Named, type_macros::Entity, type_macros::SizedEntity)]
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

        impl types::Sourced for $self
        where
            Self: types::Entity,
            $(
                $vtype: types::Entity,
            ),*
        {
            fn source(cfg: &types::Config) -> types::source::SourceTree {
                /*
                SourceBuilder::new(format!("generated/object_union_{}.hh", Self::type_tag()))
                    .tree(Self::type_source(cfg))
                    .tree(G::source(cfg))
                    .tree(S::source(cfg))
                    .tree(M::source(cfg))
                    .content(&include(&format!(
                        "geometry/ray_{}.hh",
                        G::type_prefix(cfg)
                    )))
                    .content(&include(&format!(
                        "render/light/{}.hh",
                        G::type_prefix(cfg)
                    )))
                    .content(&include_template!(
                        "object/covered.inl",
                        "Self": &Self::type_name(cfg),
                        "self": &Self::type_prefix(cfg),
                        "Geo": &G::type_name(cfg),
                        "geo": &G::type_prefix(cfg),
                        "Shape": &S::type_name(cfg),
                        "shape": &S::type_prefix(cfg),
                        "Material": &M::type_name(cfg),
                        "material": &M::type_prefix(cfg),
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
