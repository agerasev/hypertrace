use crate::{config::*, traits::*};
use std::io::{self, Read, Write};

impl BasicType for Box<dyn DynType> {
    fn id(&self) -> u64 {
        self.as_ref().id()
    }

    fn align(&self, cfg: &Config) -> usize {
        self.as_ref().align(cfg)
    }

    fn size(&self, cfg: &Config) -> Option<usize> {
        self.as_ref().size(cfg)
    }
}

impl Type for Box<dyn DynType> {
    type Value = Box<dyn DynValue>;

    fn load<R: Read + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        self.as_ref().load_dyn(cfg, &mut Box::new(src))
    }
}

impl Clone for Box<dyn DynType> {
    fn clone(&self) -> Self {
        self.as_ref().clone_dyn()
    }
}

impl BasicValue for Box<dyn DynValue> {
    fn size(&self, cfg: &Config) -> usize {
        self.as_ref().size(cfg)
    }
}

impl Value for Box<dyn DynValue> {
    type Type = Box<dyn DynType>;

    fn type_(&self) -> Box<dyn DynType> {
        self.as_ref().type_dyn()
    }

    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        self.as_ref().store_dyn(cfg, &mut Box::new(dst))
    }
}
