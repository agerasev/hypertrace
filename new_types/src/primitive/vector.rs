use crate::{primitive::PrimScal, Entity, SizedEntity, io::{CountingRead, CountingWrite}, Config, SourceTree};
use std::{io, fmt::Debug};
use vecmat::Vector;

pub trait PrimVec: SizedEntity + Copy + Default + Debug + 'static {}

macro_rules! impl_entity {
    ($N:expr) => {
        impl<T: PrimScal> PrimVec for Vector<T, $N> {}
        impl<T: PrimScal> Entity for Vector<T, $N> {
            fn align(cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn size(&self, cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn load<R: CountingRead + ?Sized>(cfg: &Config, src: &mut R) -> io::Result<Self> {
                let mut v = Self::default();
                for i in 0..$N {
                    v[i] = T::load(cfg, src)?;
                }
                Ok(v)
            }
            fn store<W: CountingWrite + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
                for i in 0..$N {
                    self[i].store(cfg, dst)?;
                }
                Ok(())
            }
            fn source(_: &Config) -> Option<SourceTree> {
                None
            }
        }
        impl<T: PrimScal> SizedEntity for Vector<T, $N> {
            fn type_size(cfg: &Config) -> usize {
                T::type_size(cfg) * $N
            }
        }
    };
}

impl_entity!(2);
impl_entity!(4);
impl_entity!(8);
impl_entity!(16);

impl<T: PrimScal> PrimVec for Vector<T, 3> {}
impl<T: PrimScal> Entity for Vector<T, 3> {
    fn align(cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn load<R: CountingRead + ?Sized>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        let mut v = Self::default();
        for i in 0..3 {
            v[i] = T::load(cfg, src)?;
        }
        let _ = T::load(cfg, src)?;
        Ok(v)
    }
    fn store<W: CountingWrite + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for i in 0..3 {
            self[i].store(cfg, dst)?;
        }
        T::default().store(cfg, dst)?;
        Ok(())
    }
    fn source(_: &Config) -> Option<SourceTree> {
        None
    }
}
impl<T: PrimScal> SizedEntity for Vector<T, 3> {
    fn type_size(cfg: &Config) -> usize {
        T::type_size(cfg) * 4
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{config::HOST_CONFIG, io::{TestBuffer, EntityWriter, EntityReader}};

    #[test]
    fn test() {
        let mut buf = TestBuffer::new();
        let sv = Vector::<i32, 3>::from([1, 2, 3]);
        buf.writer().write_entity(&HOST_CONFIG, &sv).unwrap();
        let dv = buf
            .reader()
            .read_entity(&HOST_CONFIG)
            .unwrap();
        assert_eq!(sv, dv);
    }
}
