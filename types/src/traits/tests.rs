use crate::{config::*, traits::*};
use std::io::{self, Read, Write};

const CFG: Config = Config {
    endian: Endian::Little,
    address_width: AddressWidth::X64,
    double_support: true,
};

#[derive(Debug, Clone)]
struct DummyType;

#[derive(Debug)]
struct DummyValue;

impl Type for DummyType {
    type Value = DummyValue;

    fn id(&self) -> u64 {
        type_id::<Self>()
    }

    fn align(&self, _: &Config) -> usize {
        1
    }

    fn load<R: Read + ?Sized>(&self, _: &Config, _: &mut R) -> io::Result<DummyValue> {
        Ok(DummyValue)
    }
}

impl SizedType for DummyType {
    fn size(&self, _: &Config) -> usize {
        0
    }
}

impl Value for DummyValue {
    type Type = DummyType;

    fn size(&self, cfg: &Config) -> usize {
        DummyType.size(cfg)
    }

    fn type_(&self) -> DummyType {
        DummyType
    }

    fn store<W: Write + ?Sized>(&self, _: &Config, _: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl SizedValue for DummyValue {}

#[test]
fn empty() {
    let (dty, din) = (DummyType {}, DummyValue {});
    assert_eq!(dty.id(), din.type_().id());
    assert_eq!(dty.align(&CFG), 1);
    assert_eq!(dty.size(&CFG), 0);
    assert_eq!(din.size(&CFG), 0);
}

#[test]
fn empty_dyn() {
    let (dty, din) = (
        Box::new(DummyType) as Box<dyn SizedTypeDyn>,
        Box::new(DummyValue) as Box<dyn SizedValueDyn>,
    );
    assert_eq!(dty.id(), din.type_().id());
    assert_eq!(dty.into_type_dyn().id(), din.into_value_dyn().type_().id());
}
