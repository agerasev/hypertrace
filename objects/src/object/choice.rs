#[macro_export]
macro_rules! object_choice {
    { $self:ident { $( $variant:ident($vtype:ty) ),* $(,)? } } => {
        $crate::choice! {
            $self { $(
                $variant($vtype) ,
            )* }
        }

        impl<G: $crate::Geometry> $crate::Object<G> for $self
        where
            $(
                $vtype: $crate::Object<G>,
            )*
        {
            fn wgsl_register(registry: &mut $crate::wgsl::Registry) -> $crate::wgsl::Result<()> {
                $( <$vtype as $crate::Object<G>>::wgsl_register(registry)?; )*
                Ok(())
            }

            fn wgsl_object(&self) -> $crate::wgsl::Result<$crate::wgsl::ObjectNode> {
                match self {
                    $( Self::$variant(value) => <$vtype as $crate::Object<G>>::wgsl_object(value), )*
                }
            }
        }
    };
}
