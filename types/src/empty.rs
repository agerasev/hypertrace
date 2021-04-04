use crate::{
    Type, SizedType, UnitType,
    Value, SizedValue, UnitValue,
    CountingRead, CountingWrite,
    Config, type_id,
};
use std::{io, fmt::Debug};


pub trait EmptyType: Clone + Debug + Default + 'static {
    type EmptyValue: EmptyValue<EmptyType = Self>;
}

impl<T> Type for T where T: EmptyType {
    type Value = T::EmptyValue;

    fn id(&self) -> u64 {
        type_id::<Self::Value>()
    }
    fn align(&self, _: &Config) -> usize {
        1
    }
    fn load<R: CountingRead + ?Sized>(&self, _: &Config, _: &mut R) -> io::Result<Self::Value> {
        Ok(Self::Value::default())
    }
}

impl<T> SizedType for T where T: EmptyType {
    fn size(&self, _: &Config) -> usize {
        0
    }
}

impl<T> UnitType for T where T: EmptyType {}


pub trait EmptyValue: Clone + Debug + Default + 'static {
    type EmptyType: EmptyType<EmptyValue = Self>;
}

impl<V> Value for V where V: EmptyValue {
    type Type = V::EmptyType;

    fn size(&self, _: &Config) -> usize {
        0
    }
    fn type_(&self) -> Self::Type {
        Self::Type::default()
    }
    fn store<W: CountingWrite + ?Sized>(&self, _: &Config, _: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl<V> SizedValue for V where V: EmptyValue {}

impl<V> UnitValue for V where V: EmptyValue {}
