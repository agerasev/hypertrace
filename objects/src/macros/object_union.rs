#[macro_export]
macro_rules! object_union {
    { $self:ident { $( $variant:ident($vtype:ty) ),* $(,)? } } => {
        #[derive(type_macros::EntityId, type_macros::Entity, type_macros::SizedEntity, type_macros::EntitySource)]
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
            )*
        {
            fn object_source(cfg: &types::Config) -> types::source::SourceTree {
                SourceBuilder::new(format!("generated/{}.hh", Self::object_prefix()))
                    .tree(Self::type_source(cfg))
                    .tree(G::geometry_source(cfg))
                    $(
                        .tree(<$vtype>::object_source(cfg))
                    )*
                    .content(vec![
                        format!("typedef union {}Cache {", Self::),
                    ].into_iter().map(|s| s + "\n").collect::<String>())
                    .build()
                types::source::SourceTree::new("abc")
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{object_union, object::Covered, shape::{Sphere, Cube}, material::{Lambertian, Specular}};
    use ccgeom::Euclidean3;

    object_union! {
        TestUnion {
            A(Covered<Euclidean3, Sphere, Lambertian>),
            B(Covered<Euclidean3, Cube, Specular>),
        }
    }
}
