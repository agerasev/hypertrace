use super::type_::*;
use crate::Config;
use std::io::{self, Write};

/// Instance of a runtime type base.
pub trait BasicValue: 'static {
    /// Size of instance.
    fn size(&self, cfg: &Config) -> usize;
}

/// Abstract instance of a runtime type.
pub trait DynValue: BasicValue {
    /// Returns the dynamic type of the instance.
    fn type_dyn(&self) -> Box<dyn DynType>;

    /// Stores the instance to abstract writer.
    fn store_dyn(&self, cfg: &Config, dst: &mut dyn Write) -> io::Result<()>;
}

/// Instance of a runtime type.
pub trait Value: BasicValue + DynValue {
    type Type: Type<Value = Self>;

    /// Returns the type of the instance.
    fn type_(&self) -> Self::Type;

    /// Stores the instance to a writer.
    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()>;
}

impl<V> DynValue for V
where
    V: Value,
{
    fn type_dyn(&self) -> Box<dyn DynType> {
        Box::new(self.type_())
    }

    fn store_dyn(&self, cfg: &Config, dst: &mut dyn Write) -> io::Result<()> {
        self.store(cfg, dst)
    }
}
