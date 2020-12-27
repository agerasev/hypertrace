use super::type_::*;
use crate::Config;
use std::io::{self, Write};

/// Instance of a runtime type base.
pub trait ValueBase: 'static {
    /// Size of instance.
    fn size(&self, cfg: &Config) -> usize;
}

/// Instance of a sized runtime type base.
pub trait SizedValueBase: ValueBase {}

macro_rules! def_dyn_value {
    ($T:ident, $V:ident) => {
        /// Returns the dynamic type of the instance.
        fn type_dyn(&self) -> Box<dyn $T>;

        /// Stores the instance to abstract writer.
        fn store_dyn(&self, cfg: &Config, dst: &mut dyn Write) -> io::Result<()>;
    };
}

/// Abstract instance of a runtime type.
pub trait ValueDyn: ValueBase {
    def_dyn_value!(TypeDyn, ValueDyn);
}

/// Instance of a sized runtime type.
pub trait SizedValueDyn: SizedValueBase {
    def_dyn_value!(SizedTypeDyn, SizedValueDyn);

    /// Upcast to ValueDyn.
    fn into_value_dyn(self: Box<Self>) -> Box<dyn ValueDyn>;
}

/// Instance of a runtime type.
pub trait Value: ValueBase {
    type Type: Type<Value = Self>;

    /// Returns the type of the instance.
    fn type_(&self) -> Self::Type;

    /// Stores the instance to a writer.
    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()>;
}

/// Instance of a sized runtime type.
pub trait SizedValue: SizedValueBase + Value
where
    Self::Type: SizedType<Value = Self>,
{
}

macro_rules! impl_dyn_value {
    ($T:ident, $V:ident) => {
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
