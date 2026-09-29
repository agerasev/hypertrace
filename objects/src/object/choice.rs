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
            fn shader_modules()->$crate::shader::Result<Vec<$crate::shader::ShaderModule>> {
                let mut modules=Vec::new();
                $(modules.extend(<$vtype as $crate::Object<G>>::shader_modules()?);)*
                Ok(modules)
            }

            fn object_node(&self) -> $crate::shader::Result<$crate::shader::ObjectNode> {
                match self {
                    $( Self::$variant(value) => <$vtype as $crate::Object<G>>::object_node(value), )*
                }
            }
        }
    };
}
