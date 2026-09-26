#[macro_export]
macro_rules! choice {
    { $self:ident { $( $variant:ident($vtype:ty) ),* $(,)? } } => {
        #[derive(Clone)]
        pub enum $self {
            $( $variant($vtype), )*
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
