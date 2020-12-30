use super::type_::*;
use crate::Config;
use std::io::{self, Write};

macro_rules! def_dyn_value {
    ($T:ident, $V:ident) => {
        /// Size of instance.
        fn size_dyn(&self, cfg: &Config) -> usize;

        /// Returns the dynamic type of the instance.
        fn type_dyn(&self) -> Box<dyn $T>;

        /// Stores the instance to abstract writer.
        fn store_dyn(&self, cfg: &Config, dst: &mut dyn Write) -> io::Result<()>;
    };
}

/// Abstract instance of a runtime type.
pub trait ValueDyn {
    def_dyn_value!(TypeDyn, ValueDyn);
}

/// Instance of a sized runtime type.
pub trait SizedValueDyn {
    def_dyn_value!(SizedTypeDyn, SizedValueDyn);

    /// Upcast to ValueDyn.
    fn into_value_dyn(self: Box<Self>) -> Box<dyn ValueDyn>;
}

/// Instance of a runtime type.
pub trait Value: 'static {
    /// Associated dynamic type.
    type Type: Type<Value = Self>;

    /// Size of instance.
    fn size(&self, cfg: &Config) -> usize;

    /// Returns the type of the instance.
    fn type_(&self) -> Self::Type;

    /// Stores the instance to a writer.
    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()>;
}

/// Instance of a sized runtime type.
pub trait SizedValue: Value
where
    Self::Type: SizedType<Value = Self>,
{
}

pub trait UnitValue: Value
where
    Self::Type: UnitType<Value = Self>,
{
}

macro_rules! impl_dyn_value {
    ($T:ident, $V:ident) => {
        fn size_dyn(&self, cfg: &Config) -> usize {
            self.size(cfg)
        }

        fn type_dyn(&self) -> Box<dyn $T> {
            Box::new(self.type_())
        }

        fn store_dyn(&self, cfg: &Config, dst: &mut dyn Write) -> io::Result<()> {
            self.store(cfg, dst)
        }
    };
}

impl<V> ValueDyn for V
where
    V: Value,
{
    impl_dyn_value!(TypeDyn, ValueDyn);
}

impl<V> SizedValueDyn for V
where
    V: SizedValue,
    V::Type: SizedType,
{
    impl_dyn_value!(SizedTypeDyn, SizedValueDyn);

    fn into_value_dyn(self: Box<Self>) -> Box<dyn ValueDyn> {
        self
    }
}