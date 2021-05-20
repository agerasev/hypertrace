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
                let mut src = Vec::new();
                src.append(&mut vec![
                    format!("typedef union {}Cache {{", &Self::object_name().0),
                    $(
                        format!("    {}Cache {};", <$vtype>::object_name().0, stringify!($variant)),
                    )*
                    format!("}} {}Cache;", &Self::object_name().0),
                ]);
                for (pref, suff) in &[("", ""), ("const ", "__c")] {
                    src.append(&mut vec![
                        $(
                            format!(
                                "{}Cache *{}_cache__{}{}({}{}Cache *self) {{",
                                <$vtype>::object_name().0,
                                Self::object_name().1,
                                stringify!($variant),
                                suff,
                                pref,
                                Self::object_name().0,
                            ),
                            if <$vtype as types::Entity>::min_size(cfg) > 0 {
                                format!("    return &self->{};", stringify!($variant))
                            } else {
                                format!("    return NULL;")
                            },
                            format!("}}"),
                        )*
                    ]);
                }
                src.push(types::include_template!(
                    "object/interface.inl",
                    ("Self", "self") => Self::object_name(),
                    ("Geo", "geo") => G::geometry_name(),
                ));
                src.append(&mut vec![
                    format!("typedef union {}Cache {{", &Self::object_name().0),
                    $(
                        format!("    {}Cache {};", <$vtype>::object_name().0, stringify!($variant)),
                    )*
                    format!("}} {}Cache;", &Self::object_name().0),
                ]);

                types::source::SourceBuilder::new(format!("generated/{}.hh", Self::object_name().1))
                    .tree(<Self as types::EntitySource>::source(cfg))
                    .tree(G::geometry_source(cfg))
                    $(
                        .tree(<$vtype>::object_source(cfg))
                    )*
                    .content(&src.into_iter().map(|s| s + "\n").collect::<String>())
                    .build()
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
