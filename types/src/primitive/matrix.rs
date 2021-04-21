use crate::{
    io::{CntRead, CntWrite},
    primitive::{PrimScal, PrimVec},
    Config, Entity, SizedEntity, SourceInfo,
};
use std::io;
use vecmat::{Matrix, Complex};


impl<T: PrimScal> Entity for Matrix<T, 4, 4> {
    fn align(cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        let mut v = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                v[(i, j)] = T::load(cfg, src)?;
            }
        }
        Ok(v)
    }
    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for i in 0..4 {
            for j in 0..4 {
                self[(i, j)].store(cfg, dst)?;
            }
        }
        Ok(())
    }
    fn entity_source(cfg: &Config) -> SourceInfo {
        let src = T::entity_source(cfg);
        SourceInfo::new(format!("{}16", src.name), format!("{}16", src.prefix))
    }
}

impl<T: PrimScal> SizedEntity for Matrix<T, 4, 4> {
    fn type_size(cfg: &Config) -> usize {
        16 * T::type_size(cfg)
    }
}

impl<T: PrimScal> PrimVec for Matrix<T, 4, 4> {}


impl<T: PrimScal> Entity for Matrix<T, 3, 3> {
    fn align(cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        let mut v = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                v[(i, j)] = T::load(cfg, src)?;
            }
            src.skip(T::type_size(cfg))?;
        }
        src.skip(4 * T::type_size(cfg))?;
        Ok(v)
    }
    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for i in 0..3 {
            for j in 0..3 {
                self[(i, j)].store(cfg, dst)?;
            }
            T::zero().store(cfg, dst)?;
        }
        for _ in 0..3 {
            T::zero().store(cfg, dst)?;
        }
        T::one().store(cfg, dst)?;
        Ok(())
    }
    fn entity_source(cfg: &Config) -> SourceInfo {
        let src = T::entity_source(cfg);
        SourceInfo::new(format!("{}16", src.name), format!("{}16", src.prefix))
    }
}

impl<T: PrimScal> SizedEntity for Matrix<T, 3, 3> {
    fn type_size(cfg: &Config) -> usize {
        16 * T::type_size(cfg)
    }
}

impl<T: PrimScal> PrimVec for Matrix<T, 3, 3> {}


impl<T: PrimScal> Entity for Matrix<Complex<T>, 2, 2> {
    fn align(cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        let mut v = Self::default();
        for i in 0..2 {
            for j in 0..2 {
                v[(i, j)] = Complex::<T>::load(cfg, src)?;
            }
        }
        Ok(v)
    }
    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for i in 0..2 {
            for j in 0..2 {
                self[(i, j)].store(cfg, dst)?;
            }
        }
        Ok(())
    }
    fn entity_source(cfg: &Config) -> SourceInfo {
        let src = T::entity_source(cfg);
        SourceInfo::new(format!("{}8", src.name), format!("{}8", src.prefix))
    }
}

impl<T: PrimScal> SizedEntity for Matrix<Complex<T>, 2, 2> {
    fn type_size(cfg: &Config) -> usize {
        8 * T::type_size(cfg)
    }
}

impl<T: PrimScal> PrimVec for Matrix<Complex<T>, 2, 2> {}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::HOST_CONFIG,
        io::{EntityReader, EntityWriter, TestBuffer},
    };

    #[test]
    fn scalar3x3() {
        let mut buf = TestBuffer::new();
        let sv = Matrix::<u8, 3, 3>::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        buf.writer().write_entity(&HOST_CONFIG, &sv).unwrap();
        assert_eq!(buf.vec, vec![1, 2, 3, 0, 4, 5, 6, 0, 7, 8, 9, 0, 0, 0, 0, 1]);
        let dv = buf.reader().read_entity(&HOST_CONFIG).unwrap();
        assert_eq!(sv, dv);
    }

    #[test]
    fn complex2x2() {
        let mut buf = TestBuffer::new();
        let sv = Matrix::<Complex<u8>, 2, 2>::from([[(1, 2).into(), (3, 4).into()], [(5, 6).into(), (7, 8).into()]]);
        buf.writer().write_entity(&HOST_CONFIG, &sv).unwrap();
        assert_eq!(buf.vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let dv = buf.reader().read_entity(&HOST_CONFIG).unwrap();
        assert_eq!(sv, dv);
    }
}
