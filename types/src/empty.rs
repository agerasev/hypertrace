use crate::{
    Type, SizedType, UnitType,
    Value, SizedValue, UnitValue,
    CountingRead, CountingWrite,
    Config, type_id,
};
use std::{io, fmt::Debug};


#[derive(Clone, Debug, Default)]
pub struct EmptyType;

impl Type for EmptyType {
    type Value = EmptyValue;

    fn id(&self) -> u64 {
        type_id::<Self>()
    }
    fn align(&self, _: &Config) -> usize {
        1
    }
    fn load<R: CountingRead + ?Sized>(&self, _: &Config, _: &mut R) -> io::Result<Self::Value> {
        Ok(EmptyValue::default())
    }
}

impl SizedType for EmptyType {
    fn size(&self, _: &Config) -> usize {
        0
    }
}

impl UnitType for EmptyType {}


#[derive(Clone, Debug, Default)]
pub struct EmptyValue;

impl Value for EmptyValue {
    type Type = EmptyType;

    fn size(&self, _: &Config) -> usize {
        0
    }
    fn type_(&self) -> Self::Type {
        EmptyType::default()
    }
    fn store<W: CountingWrite + ?Sized>(&self, _: &Config, _: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl SizedValue for EmptyValue {}

impl UnitValue for EmptyValue {}
