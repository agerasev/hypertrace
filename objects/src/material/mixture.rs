use type_macros::*;
use crate::{Material};

#[derive(Clone, Copy, Debug, EntitySource, Entity, SizedEntity)]
pub struct Component<M: Material> {
    #[getter] pub material: M,
    pub portion: f64,
}

impl<M: Material> From<(M, f64)> for Component<M> {
    fn from((material, portion): (M, f64)) -> Self {
        Component {
            material,
            portion,
        }
    }
}

impl<M: Material> types::EntityId for Component<M> {
    fn name() -> (String, String) {
        (
            format!("Component{}", M::name().0),
            format!("component_{}", M::name().1),
        )
    }
}

#[macro_export]
macro_rules! mixture {
    { $self:ident { $( $component:ident : $mtype:ty ),* $(,)? } } => {
        #[derive(Clone, type_macros::EntityId, type_macros::Entity, type_macros::SizedEntity, type_macros::EntitySource)]
        pub struct $self {
            $(
                pub $component: $crate::material::Component<$mtype>,
            )*
        }

        #[allow(dead_code)]
        impl $self {
            pub fn new( $(
                $component: $crate::material::Component<$mtype>,
            )* ) -> Self {
                Self { $(
                    $component,
                )* }
            }
        }

        impl $crate::Material for $self
        where
            Self: types::Entity,
            $(
                $mtype: $crate::Material,
            )*
        {
            fn material_source(cfg: &types::Config) -> types::source::SourceTree {
                let mcfs = vec![
                    $((
                        <$mtype>::material_name().1,
                        <$crate::material::Component::<$mtype> as types::EntityId>::name().1,
                        stringify!($component),
                    ),)*
                ];
                types::source::SourceBuilder::new(format!("generated/{}.hh", Self::material_name().1))
                    .tree(<Self as types::EntitySource>::source(cfg))
                    $(
                        .tree(<$mtype>::material_source(cfg))
                    )*
                    .content(&types::source::include("material/mixture/macros.hh"))
                    .content(&types::include_template!(
                        "material/mixture/impl.inl",
                        ("Self", "self") => Self::material_name(),
                        "mixture_interact_list" => mcfs.iter().map(|(m, c, f)| {
                            format!("\\\n    mixture_interact_component({}, {}, {})", m, c, f)
                        }).collect(),
                    ))
                    .build()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{mixture, material::{Specular, Lambertian}};

    mixture! {
        TestMixture {
            a: Specular,
            b: Lambertian,
        }
    }
}
