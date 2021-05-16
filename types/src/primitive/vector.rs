use crate::{
    io::{CntRead, CntWrite},
    primitive::PrimScal,
    source::SourceTree,
    Config, Entity, EntityId, SizedEntity, EntitySource,
};
use std::{fmt::Debug, io};
use vecmat::Vector;

pub trait PrimVec: SizedEntity + Copy + Default + Debug + 'static {}

macro_rules! impl_entity {
    ($N:expr) => {
        impl<T: PrimScal> PrimVec for Vector<T, $N> {}
        impl<T: PrimScal> EntityId for Vector<T, $N> {
            fn name() -> String {
                format!("{}{}", &T::name(), $N)
            }
            fn data_prefix() -> String {
                format!("{}{}", &T::data_prefix(), $N)
            }
        }
        impl<T: PrimScal> Entity for Vector<T, $N> {
            fn align(cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn size(&self, cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn min_size(cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn is_dyn_sized() -> bool {
                false
            }
            fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
                let mut v = Self::default();
                for i in 0..$N {
                    v[i] = T::load(cfg, src)?;
                }
                Ok(v)
            }
            fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
                for i in 0..$N {
                    self[i].store(cfg, dst)?;
                }
                Ok(())
            }
        }
        impl<T: PrimScal> SizedEntity for Vector<T, $N> {
            fn type_size(cfg: &Config) -> usize {
                T::type_size(cfg) * $N
            }
        }
        impl<T: PrimScal> EntitySource for Vector<T, $N> {
            fn data_source(_: &Config) -> SourceTree {
                SourceTree::new("algebra/vector.hh")
            }
        }
    };
}

impl_entity!(2);
impl_entity!(4);
impl_entity!(8);
impl_entity!(16);

impl<T: PrimScal> PrimVec for Vector<T, 3> {}
impl<T: PrimScal> EntityId for Vector<T, 3> {
    fn name() -> String {
        format!("{}3", &T::name())
    }
    fn data_prefix() -> String {
        format!("{}3", &T::data_prefix())
    }
}
impl<T: PrimScal> Entity for Vector<T, 3> {
    fn align(cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn min_size(cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn is_dyn_sized() -> bool {
        false
    }
    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        let mut v = Self::default();
        for i in 0..3 {
            v[i] = T::load(cfg, src)?;
        }
        src.skip(T::type_size(cfg))?;
        Ok(v)
    }
    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for i in 0..3 {
            self[i].store(cfg, dst)?;
        }
        dst.skip(T::type_size(cfg))?;
        Ok(())
    }
}
impl<T: PrimScal> SizedEntity for Vector<T, 3> {
    fn type_size(cfg: &Config) -> usize {
        T::type_size(cfg) * 4
    }
}
impl<T: PrimScal> EntitySource for Vector<T, 3> {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("algebra/vector.hh")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::HOST_CONFIG,
        io::{EntityReader, EntityWriter, TestBuffer},
    };

    #[test]
    fn test() {
        let mut buf = TestBuffer::new();
        let sv = Vector::<i32, 3>::from([1, 2, 3]);
        buf.writer().write_entity(&HOST_CONFIG, &sv).unwrap();
        let dv = buf.reader().read_entity(&HOST_CONFIG).unwrap();
        assert_eq!(sv, dv);
    }
}
