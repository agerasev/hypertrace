use crate::{
    io::{CntRead, CntWrite},
    primitive::{PrimVec},
    Config, Entity, SizedEntity, SourceInfo,
};
use std::io;
use vecmat::{Matrix, Complex};

// Real2x2

impl Entity for Matrix<f64, 2, 2> {
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
                v[(i, j)] = f64::load(cfg, src)?;
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
    fn entity_source(_: &Config) -> SourceInfo {
        SourceInfo::with_root(
            "real2x2",
            "r22",
            "algebra/matrix.hh",
        )
    }
}
impl SizedEntity for Matrix<f64, 2, 2> {
    fn type_size(cfg: &Config) -> usize {
        4 * f64::type_size(cfg)
    }
}
impl PrimVec for Matrix<f64, 2, 2> {}

// Real4x4

impl Entity for Matrix<f64, 4, 4> {
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
                v[(i, j)] = f64::load(cfg, src)?;
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
    fn entity_source(_: &Config) -> SourceInfo {
        SourceInfo::with_root(
            "real4x4",
            "r44",
            "algebra/matrix.hh",
        )
    }
}
impl SizedEntity for Matrix<f64, 4, 4> {
    fn type_size(cfg: &Config) -> usize {
        16 * f64::type_size(cfg)
    }
}
impl PrimVec for Matrix<f64, 4, 4> {}

// Real3x3

impl Entity for Matrix<f64, 3, 3> {
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
                v[(i, j)] = f64::load(cfg, src)?;
            }
            src.skip(f64::type_size(cfg))?;
        }
        src.skip(4 * f64::type_size(cfg))?;
        Ok(v)
    }
    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for i in 0..3 {
            for j in 0..3 {
                self[(i, j)].store(cfg, dst)?;
            }
            (0.0).store(cfg, dst)?;
        }
        for _ in 0..3 {
            (0.0).store(cfg, dst)?;
        }
        (1.0).store(cfg, dst)?;
        Ok(())
    }
    fn entity_source(_: &Config) -> SourceInfo {
        SourceInfo::with_root(
            "real3x3",
            "r33",
            "algebra/matrix.hh",
        )
    }
}
impl SizedEntity for Matrix<f64, 3, 3> {
    fn type_size(cfg: &Config) -> usize {
        16 * f64::type_size(cfg)
    }
}
impl PrimVec for Matrix<f64, 3, 3> {}

// Complex2x2

impl Entity for Matrix<Complex<f64>, 2, 2> {
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
                v[(i, j)] = Complex::<f64>::load(cfg, src)?;
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
    fn entity_source(_: &Config) -> SourceInfo {
        SourceInfo::with_root(
            "comp2x2",
            "c22",
            "algebra/matrix.hh",
        )
    }
}
impl SizedEntity for Matrix<Complex<f64>, 2, 2> {
    fn type_size(cfg: &Config) -> usize {
        8 * f64::type_size(cfg)
    }
}
impl PrimVec for Matrix<Complex<f64>, 2, 2> {}

// Tests

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
        let sv = Matrix::<f64, 3, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        buf.writer().write_entity(&HOST_CONFIG, &sv).unwrap();
        //assert_eq!(buf.vec, vec![1, 2, 3, 0, 4, 5, 6, 0, 7, 8, 9, 0, 0, 0, 0, 1]);
        let dv = buf.reader().read_entity(&HOST_CONFIG).unwrap();
        assert_eq!(sv, dv);
    }

    #[test]
    fn complex2x2() {
        let mut buf = TestBuffer::new();
        let sv = Matrix::<Complex<f64>, 2, 2>::from([[(1.0, 2.0).into(), (3.0, 4.0).into()], [(5.0, 6.0).into(), (7.0, 8.0).into()]]);
        buf.writer().write_entity(&HOST_CONFIG, &sv).unwrap();
        //assert_eq!(buf.vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let dv = buf.reader().read_entity(&HOST_CONFIG).unwrap();
        assert_eq!(sv, dv);
    }
}