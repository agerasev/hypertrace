use std::{
    io::{self, Read, Write},
};
use crate::{config::*, traits::*};


impl BasicType for Box<dyn Type> {
    fn id(&self) -> u64 {
        self.as_ref().id()
    }

    fn align(&self, config: &Config) -> usize {
        self.as_ref().align(config)
    }
}

impl Type for Box<dyn Type> {
    fn clone_dyn(&self) -> Box<dyn Type> {
        Type::clone_dyn(self.as_ref())
    }

    fn load_dyn(&self, config: &Config, src: &mut dyn Read) -> io::Result<Box<dyn Inst>> {
        self.as_ref().load_dyn(config, src)
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

impl BasicSizedType for Box<dyn SizedType> {
    fn size(&self, config: &Config) -> usize {
        self.as_ref().size(config)
    }
}

impl SizedType for Box<dyn SizedType> {
    fn clone_sized_dyn(&self) -> Box<dyn SizedType> {
        SizedType::clone_sized_dyn(self.as_ref())
    }

    fn load_sized_dyn(&self, config: &Config, src: &mut dyn Read) -> io::Result<Box<dyn SizedInst>> {
        self.as_ref().load_sized_dyn(config, src)
    }

    fn into_type_dyn(self: Box<Self>) -> Box<dyn Type> {
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

    fn type_dyn(&self) -> Box<dyn Type> {
        self.as_ref().type_dyn()
    }
}

impl BasicInst for Box<dyn SizedInst> {
    fn store(&self, config: &Config, dst: &mut dyn Write) -> io::Result<()> {
        self.as_ref().store(config, dst)
    }
}

impl SizedInst for Box<dyn SizedInst> {
    fn type_sized_dyn(&self) -> Box<dyn SizedType> {
        self.as_ref().type_sized_dyn()
    }

    fn into_inst_dyn(self: Box<Self>) -> Box<dyn Inst> {
        unimplemented!()
    }
}
