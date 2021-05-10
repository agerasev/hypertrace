use crate::{
    io::{CntRead, CntWrite},
    primitive::PrimScal,
    source::SourceTree,
    Config, Entity, Named, SizedEntity, Sourced,
};
use std::{fmt::Debug, io};
use vecmat::Vector;

pub trait PrimVec: SizedEntity + Copy + Default + Debug + 'static {}

macro_rules! impl_entity {
    ($N:expr) => {
        impl<T: PrimScal> PrimVec for Vector<T, $N> {}
        impl<T: PrimScal> Named for Vector<T, $N> {
            fn type_name(cfg: &Config) -> String {
                format!("{}{}", &T::type_name(cfg), $N)
            }
            fn type_prefix(cfg: &Config) -> String {
                format!("{}{}", &T::type_prefix(cfg), $N)
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
            fn type_source(_: &Config) -> SourceTree {
                SourceTree::new("algebra/vector.hh")
            }
        }
        impl<T: PrimScal> SizedEntity for Vector<T, $N> {
            fn type_size(cfg: &Config) -> usize {
                T::type_size(cfg) * $N
            }
        }
        impl<T: PrimScal> Sourced for Vector<T, $N> {
            fn source(cfg: &Config) -> SourceTree {
                Self::type_source(cfg)
            }
        }
    };
}

impl_entity!(2);
impl_entity!(4);
impl_entity!(8);
impl_entity!(16);

impl<T: PrimScal> PrimVec for Vector<T, 3> {}
impl<T: PrimScal> Named for Vector<T, 3> {
    fn type_name(cfg: &Config) -> String {
        format!("{}3", &T::type_name(cfg))
    }
    fn type_prefix(cfg: &Config) -> String {
        format!("{}3", &T::type_prefix(cfg))
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
    fn type_source(_: &Config) -> SourceTree {
        SourceTree::new("algebra/vector.hh")
    }
}
impl<T: PrimScal> SizedEntity for Vector<T, 3> {
    fn type_size(cfg: &Config) -> usize {
        T::type_size(cfg) * 4
    }
}
impl<T: PrimScal> Sourced for Vector<T, 3> {
    fn source(cfg: &Config) -> SourceTree {
        Self::type_source(cfg)
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
