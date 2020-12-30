use super::*;
use crate::{config::*, traits::*, utils::*};
use std::io::{self, Read, Write};
use vecmat::Vector;

#[derive(Clone, Copy, Debug)]
pub struct VecType<T: PrimType, const N: usize>
where
    T: UnitType,
    T::Value: PrimValue<Type = T> + Default,
{
    pub elem: T,
}

impl<T: PrimType, const N: usize> Type for VecType<T, N>
where
    T: UnitType,
    T::Value: PrimValue<Type = T> + Default,
{
    type Value = Vector<T::Value, N>;

    fn align(&self, cfg: &Config) -> usize {
        ceil_pow2(N) * self.elem.align(cfg)
    }

    fn id(&self) -> u64 {
        type_id::<Self>()
    }

    fn load<R: Read + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        let mut v = Self::Value::default();
        for i in 0..N {
            v[i] = self.elem.load(cfg, src)?;
        }
        for _ in N..ceil_pow2(N) {
            self.elem.load(cfg, src)?;
        }
        Ok(v)
    }
}

impl<T: PrimType, const N: usize> SizedType for VecType<T, N>
where
    T: UnitType,
    T::Value: PrimValue<Type = T> + Default,
{
    fn size(&self, cfg: &Config) -> usize {
        ceil_pow2(N) * self.elem.size(cfg)
    }
}

impl<V: PrimValue, const N: usize> Value for Vector<V, N>
where
    V: Default,
    V::Type: PrimType<Value = V> + UnitType,
{
    type Type = VecType<V::Type, N>;

    fn size(&self, cfg: &Config) -> usize {
        self.type_().size(cfg)
    }

    fn type_(&self) -> Self::Type {
        Self::Type {
            elem: V::Type::default(),
        }
    }

    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for i in 0..N {
            self[i].store(cfg, dst)?;
        }
        for _ in N..ceil_pow2(N) {
            V::default().store(cfg, dst)?;
        }
        Ok(())
    }
}

impl<V: PrimValue, const N: usize> SizedValue for Vector<V, N>
where
    V: Default,
    V::Type: PrimType<Value = V> + UnitType,
{
}