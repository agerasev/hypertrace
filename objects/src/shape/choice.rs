#[macro_export]
macro_rules! shape_choice {
    { $self:ident { $( $variant:ident($vtype:ty) ),* $(,)? } } => {
        $crate::entity_choice! {
            $self { $(
                $variant($vtype) ,
            )* }
        }

        impl<G: types::Geometry> $crate::Shape<G> for $self
        where
            Self: types::Entity,
            $(
                $vtype: $crate::Shape<G>,
            )*
        {
            fn shape_name() -> (String, String) {
                (
                    format!("Shape{}", <Self as types::EntityId>::name().0),
                    format!("shape_{}", <Self as types::EntityId>::name().1),
                )
            }
            fn shape_source(cfg: &types::Config) -> types::source::SourceTree {
                let obj_vars = vec![
                    $((
                        <$vtype>::shape_name().1,
                        stringify!($variant),
                    ),)*
                ];
                types::source::SourceBuilder::new(format!("generated/{}.hh", Self::shape_name().1))
                    .tree(G::geometry_source(cfg))
                    .content(&types::source::include(&format!(
                        "render/light/{}.hh",
                        G::geometry_name().1,
                    )))
                    .tree(<Self as types::EntitySource>::source(cfg))
                    $(
                        .tree(<$vtype>::shape_source(cfg))
                    )*
                    .content(&types::source::type_alias(
                        &Self::shape_name().0, 
                        &<Self as types::EntityId>::name().0,
                    ))
                    .content(&types::source::include("shape/choice/macros.hh"))
                    .content(&types::include_template!(
                        "shape/choice/impl.inl",
                        ("Self", "self") => Self::shape_name(),
                        ("Geo", "geo") => G::geometry_name(),
                        "shape_choice_detect_list" => obj_vars.iter().enumerate().map(|(i, (obj, var))| {
                            format!("\\\n    shape_choice_detect_variant({}, {}, {}, {})", <Self as types::EntityId>::name().1, i, obj, var)
                        }).collect(),
                    ))
                    .build()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{shape_choice, shape::{Sphere, Cube}};

    shape_choice! {
        TestChoice {
            A(Sphere),
            B(Cube),
        }
    }
}
