use crate::traits::*;

macro_rules! impl_type_box {
    ($T:ident, $V:ident) => {
        impl Type for Box<dyn $T> {
            type Value = Box<dyn $V>;

            fn id(&self) -> u64 {
                self.as_ref().id_dyn()
            }

            fn align(&self, cfg: &$crate::Config) -> usize {
                self.as_ref().align_dyn(cfg)
            }

            fn load<R: $crate::CountingRead + ?Sized>(
                &self,
                cfg: &$crate::Config,
                src: &mut R,
            ) -> std::io::Result<Self::Value> {
                self.as_ref().load_dyn(cfg, src.as_dyn_mut())
            }
        }

        impl Clone for Box<dyn $T> {
            fn clone(&self) -> Self {
                self.as_ref().clone_dyn()
            }
        }

        impl std::fmt::Debug for Box<dyn $T> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.as_ref().debug_fmt_dyn(f)
            }
        }
    };
}

macro_rules! impl_sized_type_box {
    ($T:ident, $V:ident) => {
        impl SizedType for Box<dyn $T> {
            fn size(&self, cfg: &$crate::Config) -> usize {
                self.as_ref().size_dyn(cfg)
            }
        }
    };
}

impl_type_box!(TypeDyn, ValueDyn);

impl_type_box!(SizedTypeDyn, SizedValueDyn);
impl_sized_type_box!(SizedTypeDyn, SizedValueDyn);

macro_rules! impl_value_box {
    ($T:ident, $V:ident) => {
        impl Value for Box<dyn $V> {
            type Type = Box<dyn $T>;

            fn size(&self, cfg: &$crate::Config) -> usize {
                self.as_ref().size_dyn(cfg)
            }

            fn type_(&self) -> Box<dyn $T> {
                self.as_ref().type_dyn()
            }

            fn store<W: $crate::CountingWrite + ?Sized>(
                &self,
                cfg: &$crate::Config,
                dst: &mut W,
            ) -> std::io::Result<()> {
                self.as_ref().store_dyn(cfg, dst.as_dyn_mut())
            }
        }
    };
}

macro_rules! impl_sized_value_box {
    ($T:ident, $V:ident) => {
        impl SizedValue for Box<dyn $V> {}
    };
}

impl_value_box!(TypeDyn, ValueDyn);

impl_value_box!(SizedTypeDyn, SizedValueDyn);
impl_sized_value_box!(SizedTypeDyn, SizedValueDyn);
