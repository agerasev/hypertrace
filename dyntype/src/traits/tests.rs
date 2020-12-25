use std::io::{self, Read, Write};
use crate::{config::*, traits::*};

const CONFIG: Config = Config {
    endianness: Endianness::Little,
    address_width: AddressWidth::X64,
    double_support: true,
};

#[derive(Debug)]
struct DummyType;

#[derive(Debug)]
struct DummyInst;

impl BasicType for DummyType {
    fn id(&self) -> u64 {
        type_id::<Self>()
    }

    fn align(&self, _: &Config) -> usize {
        1
    }
}

impl BasicSizedType for DummyType {
    fn size(&self, _: &Config) -> usize {
        0
    }
}

impl SizedType for DummyType {
    fn clone_sized_dyn(&self) -> Box<dyn SizedType> {
        Box::new(DummyType)
    }

    fn load_sized_dyn(&self, _: &Config, _: &mut dyn Read) -> io::Result<Box<dyn SizedInst>> {
        Ok(Box::new(DummyInst))
    }

    fn into_type_dyn(self: Box<Self>) -> Box<dyn super::Type> {
        self
    }
}

impl BasicInst for DummyInst {
    fn store(&self, _: &Config, _: &mut dyn Write) -> io::Result<()> {
        Ok(())
    }
}

impl SizedInst for DummyInst {
    fn type_sized_dyn(&self) -> Box<dyn SizedType> {
        Box::new(DummyType)
    }

    fn into_inst_dyn(self: Box<Self>) -> Box<dyn super::Inst> {
        self
    }
}

#[test]
fn empty() {
    let (dty, din) = (DummyType{}, DummyInst{});
    assert_eq!(dty.id(), din.type_dyn().id());
    assert_eq!(dty.align(&CONFIG), 1);
    assert_eq!(dty.size(&CONFIG), 0);
    assert_eq!(din.size(&CONFIG), 0);
}
#[test]
fn empty_dyn() {
    let (dty, din) = (Box::new(DummyType{}) as Box<dyn SizedType>, Box::new(DummyInst{}) as Box<dyn SizedInst>);
    assert_eq!(dty.id(), din.type_dyn().id());
}
