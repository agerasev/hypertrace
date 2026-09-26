#[macro_export]
macro_rules! shape_choice {
    { $self:ident { $( $variant:ident($vtype:ty) ),* $(,)? } } => {
        $crate::choice! {
            $self { $(
                $variant($vtype) ,
            )* }
        }

        impl<G: $crate::Geometry> $crate::Shape<G> for $self
        where
            $(
                $vtype: $crate::Shape<G>,
            )*
        {
            fn wgsl_shape_schema() -> $crate::wgsl::Result<$crate::wgsl::ShapeSchema> {
                Ok($crate::wgsl::ShapeSchema::Choice(vec![
                    $( <$vtype as $crate::Shape<G>>::wgsl_shape_schema()?, )*
                ]))
            }

            fn wgsl_shape(&self) -> $crate::wgsl::Result<$crate::wgsl::ShapeValue> {
                #[allow(non_camel_case_types)]
                enum VariantIndex { $( $variant, )* }
                let variants = vec![ $( <$vtype as $crate::Shape<G>>::wgsl_shape_schema()?, )* ];
                match self {
                    $( Self::$variant(value) => $crate::wgsl::ShapeValue::choice(
                        variants, VariantIndex::$variant as usize,
                        <$vtype as $crate::Shape<G>>::wgsl_shape(value)?,
                    ), )*
                }
            }
        }
    };
}
