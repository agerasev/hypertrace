use std::{
    io::{self, Write},
};
use crate::{Config};
use super::type_::*;

/// Basic abstract instance.
pub trait BasicInst: 'static {
    /// Stores the instance to abstract writer.
    /// 
    /// The `dst` should have capacity to store at least `self.size()` bytes.
    fn store(&self, config: &Config, dst: &mut dyn Write) -> io::Result<()>;
}

/// Abstract instance of a runtime type.
pub trait Inst: BasicInst {
    /// Size of instance.
    fn size(&self, config: &Config) -> usize;

    /// Returns an abstract type of the instance.
    fn type_dyn(&self) -> Box<dyn Type>;
}

/// Abstract instance of a runtime type.
pub trait SizedInst: Inst {
    /// Returns an abstract type of the instance.
    fn type_sized_dyn(&self) -> Box<dyn SizedType>;

    /// Upcast to type.
    fn into_inst_dyn(self: Box<Self>) -> Box<dyn Inst>;
}

impl<T: SizedInst> Inst for T {
    fn size(&self, config: &Config) -> usize {
        self.size(config)
    }

    fn type_dyn(&self) -> Box<dyn Type> {
        self.type_sized_dyn().into_type_dyn()
    }
}
