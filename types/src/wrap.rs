#[macro_export]
macro_rules! wrap_def {
    ($T:ident, $V:ident, $W:ty) => {
        #[derive(Clone, Debug, Default)]
        pub struct $T(<$W as $crate::Value>::Type);

        #[derive(Clone, Debug, Default)]
        pub struct $V($W);
    };
}

#[macro_export]
macro_rules! wrap_impl {
    ($T:ident, $V:ident, $W:ty) => {
        impl $crate::Type for $T where $W: $crate::Value {
            type Value = $V;
        
            fn id(&self) -> u64 {
                $crate::type_id::<Self::Value>()
            }
            fn align(&self, cfg: &$crate::Config) -> usize {
                self.0.align(cfg)
            }
            fn load<R: $crate::CountingRead + ?Sized>(&self, cfg: &$crate::Config, src: &mut R) -> std::io::Result<Self::Value> {
                self.0.load(cfg, src).map($V)
            }
        }

        impl $crate::Value for $V where $W: $crate::Value {
            type Type = $T;
        
            fn size(&self, cfg: &$crate::Config) -> usize {
                self.0.size(cfg)
            }
            fn type_(&self) -> Self::Type {
                $T(self.0.type_())
            }
            fn store<W: $crate::CountingWrite + ?Sized>(&self, cfg: &$crate::Config, dst: &mut W) -> std::io::Result<()> {
                self.0.store(cfg, dst)
            }
        }
    };
}

#[macro_export]
macro_rules! wrap_impl_sized {
    ($T:ident, $V:ident, $W:ty) => {
        impl $crate::SizedType for $T where $W: $crate::SizedValue {
            fn size(&self, cfg: &$crate::Config) -> usize {
                self.0.size(cfg)
            }
        }

        impl $crate::SizedValue for $V where $W: $crate::SizedValue {}
    };
}

#[macro_export]
macro_rules! wrap_impl_unit {
    ($T:ident, $V:ident, $W:ty) => {
        impl $crate::UnitType for $T where $W: $crate::UnitValue {}

        impl $crate::UnitValue for $V where $W: $crate::UnitValue {}        
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    wrap_def!(DummyType, DummyValue, u32);
    wrap_impl!(DummyType, DummyValue, u32);
    wrap_impl_sized!(DummyType, DummyValue, u32);
    wrap_impl_unit!(DummyType, DummyValue, u32);
}
