use super::*;
use crate::{config::*, io::*, traits::*, utils::*};
use std::io;
use vecmat::Vector;

#[derive(Clone, Copy, Default, Debug)]
pub struct VecType<T: PrimType, const N: usize>
where
    T::Value: PrimValue + Default,
{
    pub elem: T,
}

impl<T: PrimType, const N: usize> Type for VecType<T, N>
where
    T::Value: PrimValue + Default,
{
    type Value = Vector<T::Value, N>;

    fn align(&self, cfg: &Config) -> usize {
        ceil_pow2(N) * self.elem.align(cfg)
    }

    fn id(&self) -> u64 {
        type_id::<Self>()
    }

    fn load<R: CountingRead + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
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
    T::Value: PrimValue + Default,
{
    fn size(&self, cfg: &Config) -> usize {
        ceil_pow2(N) * self.elem.size(cfg)
    }
}

impl<T: PrimType, const N: usize> UnitType for VecType<T, N> where T::Value: PrimValue + Default {}

impl<T: PrimType, const N: usize> PrimType for VecType<T, N> where T::Value: PrimValue + Default {}

impl<V: PrimValue + Default, const N: usize> Value for Vector<V, N>
where
    V::Type: PrimType,
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

    fn store<W: CountingWrite + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for i in 0..N {
            self[i].store(cfg, dst)?;
        }
        for _ in N..ceil_pow2(N) {
            V::default().store(cfg, dst)?;
        }
        Ok(())
    }
}

impl<V: PrimValue + Default, const N: usize> SizedValue for Vector<V, N> where V::Type: PrimType {}

impl<V: PrimValue + Default, const N: usize> UnitValue for Vector<V, N> where V::Type: PrimType {}

impl<V: PrimValue + Default, const N: usize> PrimValue for Vector<V, N> where V::Type: PrimType {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut buf = TestBuffer::new();
        let sv = Vector::<i32, 3>::from([1, 2, 3]);
        buf.writer().write_value(&HOST_CONFIG, &sv).unwrap();
        let dv = buf
            .reader()
            .read_value(&HOST_CONFIG, &VecType::<I32Type, 3>::default())
            .unwrap();
        assert_eq!(sv, dv);
    }
}
