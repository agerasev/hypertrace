mod scal;
mod vec;

pub use scal::*;
pub use vec::*;

use crate::traits::*;

pub trait PrimType: SizedType + UnitType
where
    Self::Value: PrimValue<Type = Self>,
{
}

pub trait PrimValue: SizedValue + UnitValue
where
    Self::Type: PrimType<Value = Self>,
{
}
