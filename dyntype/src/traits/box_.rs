use crate::{config::*, traits::*};
use std::io::{self, Read, Write};

macro_rules! impl_type_box {
    ($T:ident, $V:ident) => {
        impl TypeBase for Box<dyn $T> {
            fn id(&self) -> u64 {
                self.as_ref().id()
            }

            fn align(&self, cfg: &Config) -> usize {
                self.as_ref().align(cfg)
            }
        }

        impl Type for Box<dyn $T> {
            type Value = Box<dyn $V>;

            fn load<R: Read + ?Sized>(&self, cfg: &Config, mut src: &mut R) -> io::Result<Self::Value> {
                self.as_ref().load_dyn(cfg, &mut src)
            }
        }

        impl Clone for Box<dyn $T> {
            fn clone(&self) -> Self {
                self.as_ref().clone_dyn()
            }
        }
    };
}

macro_rules! impl_sized_type_box {
    ($T:ident, $V:ident) => {
        impl SizedTypeBase for Box<dyn $T> {
            fn size(&self, cfg: &Config) -> usize {
                self.as_ref().size(cfg)
            }
        }

        impl SizedType for Box<dyn $T> {}
    };
}

impl_type_box!(TypeDyn, ValueDyn);

impl_type_box!(SizedTypeDyn, SizedValueDyn);
impl_sized_type_box!(SizedTypeDyn, SizedValueDyn);


macro_rules! impl_value_box {
    ($T:ident, $V:ident) => {
        impl ValueBase for Box<dyn $V> {
            fn size(&self, cfg: &Config) -> usize {
                self.as_ref().size(cfg)
            }
        }
        
        impl Value for Box<dyn $V> {
            type Type = Box<dyn $T>;
        
            fn type_(&self) -> Box<dyn $T> {
                self.as_ref().type_dyn()
            }
        
            fn store<W: Write + ?Sized>(&self, cfg: &Config, mut dst: &mut W) -> io::Result<()> {
                self.as_ref().store_dyn(cfg, &mut dst)
            }
        }
    };
}

macro_rules! impl_sized_value_box {
    ($T:ident, $V:ident) => {
        impl SizedValueBase for Box<dyn $V> {}

        impl SizedValue for Box<dyn $V> {}
    };
}

impl_value_box!(TypeDyn, ValueDyn);

impl_value_box!(SizedTypeDyn, SizedValueDyn);
impl_sized_value_box!(SizedTypeDyn, SizedValueDyn);



