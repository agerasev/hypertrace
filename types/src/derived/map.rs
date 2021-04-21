use crate::{Entity, SizedEntity, primitive::{PrimScal, PrimVec}, Config, source::{Sourced, SourceInfo}/*, math*/};
use vecmat::{Vector, Matrix, complex::{Complex, Quaternion}, transform::{Shift, Linear, Rotation2, Rotation3, Moebius/*, Chain*/}};


pub trait Map<T>: SizedEntity + Sourced {}


// Shift

impl<T: PrimScal, const N: usize> Entity for Shift<T, N> where Vector<T, N>: PrimVec {
    impl_entity_unwrap!(Vector<T, N>);
}

impl<T: PrimScal, const N: usize> SizedEntity for Shift<T, N> where Vector<T, N>: PrimVec {
    impl_sized_entity_unwrap!(Vector<T, N>);
}

impl<T: PrimScal, const N: usize> Sourced for Shift<T, N> where Vector<T, N>: PrimVec {
    fn source(cfg: &Config) -> SourceInfo {
        Self::entity_source(cfg)
    }
}

impl<T: PrimScal, const N: usize> Map<Vector<T, N>> for Shift<T, N> where Vector<T, N>: PrimVec {}


// Rotation2

impl<T: PrimScal> Entity for Rotation2<T> {
    impl_entity_unwrap!(Complex<T>);
}

impl<T: PrimScal> SizedEntity for Rotation2<T> {
    impl_sized_entity_unwrap!(Complex<T>);
}

impl<T: PrimScal> Sourced for Rotation2<T> {
    fn source(cfg: &Config) -> SourceInfo {
        Self::entity_source(cfg)
    }
}

impl<T: PrimScal> Map<Vector<T, 2>> for Rotation2<T> {}


// Rotation3

impl<T: PrimScal> Entity for Rotation3<T> {
    impl_entity_unwrap!(Quaternion<T>);
}

impl<T: PrimScal> SizedEntity for Rotation3<T> {
    impl_sized_entity_unwrap!(Quaternion<T>);
}

impl<T: PrimScal> Sourced for Rotation3<T> {
    fn source(cfg: &Config) -> SourceInfo {
        Self::entity_source(cfg)
    }
}

impl<T: PrimScal> Map<Quaternion<T>> for Rotation3<T> {}


// Linear

impl<T: PrimScal, const N: usize> Entity for Linear<T, N> where Matrix<T, N, N>: PrimVec {
    impl_entity_unwrap!(Matrix<T, N, N>);
}

impl<T: PrimScal, const N: usize> SizedEntity for Linear<T, N> where Matrix<T, N, N>: PrimVec {
    impl_sized_entity_unwrap!(Matrix<T, N, N>);
}

impl<T: PrimScal, const N: usize> Sourced for Linear<T, N> where Matrix<T, N, N>: PrimVec {
    fn source(cfg: &Config) -> SourceInfo {
        Self::entity_source(cfg)
    }
}

impl<T: PrimScal, const N: usize> Map<Vector<T, N>> for Linear<T, N> where Matrix<T, N, N>: PrimVec {}


// Moebius

impl<T: PrimScal> Entity for Moebius<Complex<T>> {
    impl_entity_unwrap!(Matrix<Complex<T>, 2, 2>);
}

impl<T: PrimScal> SizedEntity for Moebius<Complex<T>> {
    impl_sized_entity_unwrap!(Matrix<Complex<T>, 2, 2>);
}

impl<T: PrimScal> Sourced for Moebius<Complex<T>> {
    fn source(cfg: &Config) -> SourceInfo {
        Self::entity_source(cfg)
    }
}

impl<T: PrimScal> Map<Complex<T>> for Moebius<Complex<T>> {}

impl<T: PrimScal> Map<Quaternion<T>> for Moebius<Complex<T>> {}


// Chain
/*
impl<A, B, T> Entity for Chain<A, B, T> where A: Map<T>, B: Map<T>, T: SizedEntity {
    fn align(cfg: &Config) -> usize {
        mathSelf::type_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::type_size(cfg)
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
    fn entity_source(cfg: &Config) -> SourceInfo {
        let src = T::entity_source(cfg);
        SourceInfo::new(format!("{}3", src.name), format!("{}3", src.prefix))
    }
}

impl<T: PrimScal> SizedEntity for Vector<T, 3> {
    fn type_size(cfg: &Config) -> usize {
        T::type_size(cfg) * 4
    }
}

impl<T: PrimScal> PrimVec for Vector<T, 3> {}
*/
