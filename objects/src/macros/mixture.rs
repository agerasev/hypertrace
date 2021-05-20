#[macro_export]
macro_rules! material_mixture {
    { $self:ident { $field:ident: $ftype:ty },+ $(,)? } => {
        #[derive(Clone, Debug, EntityId, Entity, SizedEntity)]
        pub struct $self {
            $(pub $field: ($ftype, f64)),+
        }
    };
}
