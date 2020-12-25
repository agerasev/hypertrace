use std::{
    io::{self, Write},
};
use crate::{Config, Type, SizedType};

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
    fn type_(&self) -> Box<dyn Type>;
}

/// Abstract instance of a runtime type.
pub trait SizedInst: BasicInst {
    // Methods from Inst
    fn size(&self, config: &Config) -> usize {
        self.type_().size(config)
    }

    /// Returns an abstract type of the instance.
    fn type_(&self) -> Box<dyn SizedType>;

    // Own methods

    /// Upcast to type.
    fn into_inst(self: Box<Self>) -> Box<dyn Inst>;
}

impl<T: SizedInst> Inst for T {
    fn size(&self, config: &Config) -> usize {
        self.size(config)
    }

    fn type_(&self) -> Box<dyn Type> {
        self.type_().into_type()
    }
}
