#[macro_export]
macro_rules! object_choice {
    { $self:ident($cache:ident) { $( $variant:ident($vtype:ty) ),* $(,)? } } => {
        $crate::entity_choice! {
            $self { $(
                $variant($vtype) ,
            )* }
        }

        #[allow(non_snake_case)]
        #[derive(Clone, type_macros::EntityId, type_macros::Entity, type_macros::SizedEntity, type_macros::EntitySource)]
        pub struct $cache<G: types::Geometry>
        where
            $(
                $vtype: $crate::Object<G>,
            )*
        {
            $(
                #[getter]
                pub $variant: <$vtype as $crate::Object<G>>::Cache,
            )*
        }

        impl<G: types::Geometry> $crate::Object<G> for $self
        where
            Self: types::Entity,
            $(
                $vtype: $crate::Object<G>,
            )*
        {
            type Cache = $cache<G>;

            fn object_name() -> (String, String) {
                (
                    format!("Object{}", <Self as types::EntityId>::name().0),
                    format!("object_{}", <Self as types::EntityId>::name().1),
                )
            }
            fn object_source(cfg: &types::Config) -> types::source::SourceTree {
                let obj_vars = vec![
                    $((
                        <$vtype>::object_name().1,
                        stringify!($variant),
                    ),)*
                ];
                types::source::SourceBuilder::new(format!("generated/{}.hh", Self::object_name().1))
                    .tree(G::geometry_source(cfg))
                    .content(&types::source::include(&format!(
                        "render/light/{}.hh",
                        G::geometry_name().1,
                    )))
                    .tree(<Self as types::EntitySource>::source(cfg))
                    $(
                        .tree(<$vtype>::object_source(cfg))
                    )*
                    .tree(<Self::Cache as types::EntitySource>::source(cfg))
                    .content(&types::source::type_alias(
                        &Self::object_name().0,
                        &<Self as types::EntityId>::name().0,
                    ))
                    .content(&types::source::include("object/choice/macros.hh"))
                    .content(&types::include_template!(
                        "object/choice/impl.inl",
                        ("Self", "self") => Self::object_name(),
                        ("Geo", "geo") => G::geometry_name(),
                        ("Cache", "cache") => <Self::Cache as types::EntityId>::name(),
                        "object_choice_detect_list" => obj_vars.iter().enumerate().map(|(i, (obj, var))| {
                            format!("\\\n    object_choice_detect_variant({}, {}, {}, {})", <Self as types::EntityId>::name().1, i, obj, var)
                        }).collect(),
                        "object_choice_interact_list" => obj_vars.iter().enumerate().map(|(i, (obj, var))| {
                            format!("\\\n    object_choice_interact_variant({}, {}, {}, {})", <Self as types::EntityId>::name().1, i, obj, var)
                        }).collect(),
                    ))
                    .build()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        material::{Lambertian, Specular},
        object::Covered,
        object_choice,
        shape::{Cube, Sphere},
    };
    use base::ccgeom::Euclidean3;

    object_choice! {
        TestChoice(TestChoiceCache) {
            A(Covered<Euclidean3, Sphere, Lambertian>),
            B(Covered<Euclidean3, Cube, Specular>),
        }
    }
}
