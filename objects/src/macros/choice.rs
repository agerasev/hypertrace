#[macro_export]
macro_rules! entity_choice {
    { $self:ident { $( $variant:ident($vtype:ty) ),* $(,)? } } => {
        #[derive(type_macros::EntityId, type_macros::Entity, type_macros::SizedEntity, type_macros::EntitySource)]
        pub enum $self {
            $(
                #[getter]
                $variant(#[getter] $vtype),
            )*
        }

        $(
            impl From<$vtype> for $self {
                fn from(var: $vtype) -> Self {
                    $self::$variant(var)
                }
            }
        )*
    };
}
