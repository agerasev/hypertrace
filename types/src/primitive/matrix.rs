use crate::{
    io::{CntRead, CntWrite},
    primitive::PrimVec,
    source::SourceTree,
    Config, Entity, EntityId, EntitySource, SizedEntity,
};
use base::vecmat::{Complex, Matrix};
use std::io;

// Real2x2

impl EntityId for Matrix<f64, 2, 2> {
    fn name() -> (String, String) {
        ("real2x2".into(), "r22".into())
    }
}
impl Entity for Matrix<f64, 2, 2> {
    fn align(cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn min_size(cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn is_dyn_sized() -> bool {
        false
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
}
impl SizedEntity for Matrix<f64, 2, 2> {
    fn static_size(cfg: &Config) -> usize {
        4 * f64::static_size(cfg)
    }
}
impl EntitySource for Matrix<f64, 2, 2> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("algebra/matrix.hh")
    }
}
impl PrimVec for Matrix<f64, 2, 2> {}

// Real4x4

impl EntityId for Matrix<f64, 4, 4> {
    fn name() -> (String, String) {
        ("real4x4".into(), "r44".into())
    }
}
impl Entity for Matrix<f64, 4, 4> {
    fn align(cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn min_size(cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn is_dyn_sized() -> bool {
        false
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
}
impl SizedEntity for Matrix<f64, 4, 4> {
    fn static_size(cfg: &Config) -> usize {
        16 * f64::static_size(cfg)
    }
}
impl EntitySource for Matrix<f64, 4, 4> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("algebra/matrix.hh")
    }
}
impl PrimVec for Matrix<f64, 4, 4> {}

// Real3x3

impl EntityId for Matrix<f64, 3, 3> {
    fn name() -> (String, String) {
        ("real3x3".into(), "r33".into())
    }
}
impl Entity for Matrix<f64, 3, 3> {
    fn align(cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn min_size(cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn is_dyn_sized() -> bool {
        false
    }
    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        let mut v = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                v[(i, j)] = f64::load(cfg, src)?;
            }
            src.skip(f64::static_size(cfg))?;
        }
        src.skip(4 * f64::static_size(cfg))?;
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
}
impl SizedEntity for Matrix<f64, 3, 3> {
    fn static_size(cfg: &Config) -> usize {
        16 * f64::static_size(cfg)
    }
}
impl EntitySource for Matrix<f64, 3, 3> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("algebra/matrix.hh")
    }
}
impl PrimVec for Matrix<f64, 3, 3> {}

// Complex2x2

impl EntityId for Matrix<Complex<f64>, 2, 2> {
    fn name() -> (String, String) {
        ("comp2x2".into(), "c22".into())
    }
}
impl Entity for Matrix<Complex<f64>, 2, 2> {
    fn align(cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn min_size(cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn is_dyn_sized() -> bool {
        false
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
}
impl SizedEntity for Matrix<Complex<f64>, 2, 2> {
    fn static_size(cfg: &Config) -> usize {
        8 * f64::static_size(cfg)
    }
}
impl EntitySource for Matrix<Complex<f64>, 2, 2> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("algebra/matrix.hh")
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
        let sv = Matrix::<Complex<f64>, 2, 2>::from([
            [(1.0, 2.0).into(), (3.0, 4.0).into()],
            [(5.0, 6.0).into(), (7.0, 8.0).into()],
        ]);
        buf.writer().write_entity(&HOST_CONFIG, &sv).unwrap();
        //assert_eq!(buf.vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let dv = buf.reader().read_entity(&HOST_CONFIG).unwrap();
        assert_eq!(sv, dv);
    }
}
