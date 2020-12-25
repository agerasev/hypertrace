use std::{
    io::{self, Read, Write},
};
use crate::{Config, BasicType, Type, SizedType, BasicInst, Inst, SizedInst};


impl BasicType for Box<dyn Type> {
    fn id(&self) -> u64 {
        self.as_ref().id()
    }

    fn align(&self, config: &Config) -> usize {
        self.as_ref().align(config)
    }
}

impl Type for Box<dyn Type> {
    fn clone(&self) -> Box<dyn Type> {
        Type::clone(self.as_ref())
    }

    fn load(&self, config: &Config, src: &mut dyn Read) -> io::Result<Box<dyn Inst>> {
        self.as_ref().load(config, src)
    }
}

impl BasicType for Box<dyn SizedType> {
    fn id(&self) -> u64 {
        self.as_ref().id()
    }

    fn align(&self, config: &Config) -> usize {
        self.as_ref().align(config)
    }
}

impl SizedType for Box<dyn SizedType> {
    fn clone(&self) -> Box<dyn SizedType> {
        SizedType::clone(self.as_ref())
    }

    fn load(&self, config: &Config, src: &mut dyn Read) -> io::Result<Box<dyn SizedInst>> {
        self.as_ref().load(config, src)
    }

    fn size(&self, config: &Config) -> usize {
        self.as_ref().size(config)
    }

    fn into_type(self: Box<Self>) -> Box<dyn Type> {
        unimplemented!()
    }
}


impl BasicInst for Box<dyn Inst> {
    fn store(&self, config: &Config, dst: &mut dyn Write) -> io::Result<()> {
        self.as_ref().store(config, dst)
    }
}

impl Inst for Box<dyn Inst> {
    fn size(&self, config: &Config) -> usize {
        self.as_ref().size(config)
    }

    fn type_(&self) -> Box<dyn Type> {
        self.as_ref().type_()
    }
}

impl BasicInst for Box<dyn SizedInst> {
    fn store(&self, config: &Config, dst: &mut dyn Write) -> io::Result<()> {
        self.as_ref().store(config, dst)
    }
}

impl SizedInst for Box<dyn SizedInst> {
    fn type_(&self) -> Box<dyn SizedType> {
        self.as_ref().type_()
    }

    fn into_inst(self: Box<Self>) -> Box<dyn Inst> {
        unimplemented!()
    }
}
