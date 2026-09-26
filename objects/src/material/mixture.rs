use crate::Material;

#[derive(Clone, Copy, Debug)]
pub struct Component<M: Material> {
    pub material: M,
    pub portion: f64,
}

impl<M: Material> From<(M, f64)> for Component<M> {
    fn from((material, portion): (M, f64)) -> Self {
        Component { material, portion }
    }
}

#[macro_export]
macro_rules! mixture {
    { $self:ident { $( $component:ident : $mtype:ty ),* $(,)? } } => {
        #[derive(Clone)]
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
            $(
                $mtype: $crate::Material,
            )*
        {
            fn wgsl_material_schema() -> $crate::wgsl::Result<$crate::wgsl::MaterialSchema> {
                Ok($crate::wgsl::MaterialSchema::Mixture(vec![
                    $( <$mtype as $crate::Material>::wgsl_material_schema()?, )*
                ]))
            }

            fn wgsl_material(&self) -> $crate::wgsl::Result<$crate::wgsl::MaterialValue> {
                $crate::wgsl::MaterialValue::mixture(vec![
                    $( (self.$component.portion,
                        <$mtype as $crate::Material>::wgsl_material(&self.$component.material)?), )*
                ])
            }
        }
    };
}
