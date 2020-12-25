use std::io::{self, Read, Write};
use crate::{Config, Endianness, AddressWidth, BasicType, SizedType, BasicInst, SizedInst, type_id};

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

impl SizedType for DummyType {
    fn clone(&self) -> Box<dyn SizedType> {
        Box::new(DummyType)
    }

    fn load(&self, _: &Config, _: &mut dyn Read) -> io::Result<Box<dyn SizedInst>> {
        Ok(Box::new(DummyInst))
    }

    fn size(&self, _: &Config) -> usize {
        0
    }

    fn into_type(self: Box<Self>) -> Box<dyn super::Type> {
        self
    }
}

impl BasicInst for DummyInst {
    fn store(&self, _: &Config, _: &mut dyn Write) -> io::Result<()> {
        Ok(())
    }
}

impl SizedInst for DummyInst {
    fn type_(&self) -> Box<dyn SizedType> {
        Box::new(DummyType)
    }

    fn into_inst(self: Box<Self>) -> Box<dyn super::Inst> {
        self
    }
}

#[test]
fn empty() {
    let (dty, din) = (DummyType{}, DummyInst{});
    assert_eq!(dty.id(), din.type_().id());
    assert_eq!(dty.align(&CONFIG), 1);
    assert_eq!(dty.size(&CONFIG), 0);
    assert_eq!(din.size(&CONFIG), 0);
}
#[test]
fn empty_dyn() {
    let (dty, din) = (Box::new(DummyType{}) as Box<dyn SizedType>, Box::new(DummyInst{}) as Box<dyn SizedInst>);
    assert_eq!(dty.id(), din.type_().id());
}
