use crate::{Entity, SizedEntity, Config, source::{Sourced, SourceInfo}, math, io::{CntRead, CntWrite}};
use vecmat::{Vector, Matrix, complex::{Complex, Quaternion}, transform::{Shift, Linear, Rotation2, Rotation3, Moebius, Chain, Reorder}};
use std::io;
use ccgeom::Map as GeoMap;

pub trait Map<P, D = P>: GeoMap<P, D> + SizedEntity + Sourced {}

impl<M, P, D> Map<P, D> for M where M: GeoMap<P, D> + SizedEntity + Sourced {}

// Shift

impl Entity for Shift<f64, 2> {
    impl_entity_unwrap!(Vector<f64, 2>);
}
impl SizedEntity for Shift<f64, 2> {
    impl_sized_entity_unwrap!(Vector<f64, 2>);
}
impl Sourced for Shift<f64, 2> {
    fn source(_: &Config) -> SourceInfo {
        unimplemented!()
        //SourceInfo::with_root("Shift2", "shf2", "algebra/shift.hh")
    }
}

impl Entity for Shift<f64, 3> {
    impl_entity_unwrap!(Vector<f64, 3>);
}
impl SizedEntity for Shift<f64, 3> {
    impl_sized_entity_unwrap!(Vector<f64, 3>);
}
impl Sourced for Shift<f64, 3> {
    fn source(_: &Config) -> SourceInfo {
        unimplemented!()
        //SourceInfo::with_root("Shift3", "shf3", "algebra/shift.hh")
    }
}

// Rotation2

impl Entity for Rotation2<f64> {
    impl_entity_unwrap!(Complex<f64>);
}
impl SizedEntity for Rotation2<f64> {
    impl_sized_entity_unwrap!(Complex<f64>);
}
impl Sourced for Rotation2<f64> {
    fn source(_: &Config) -> SourceInfo {
        SourceInfo::with_root("Rotation2", "rot2", "algebra/rotation.hh")
    }
}

// Rotation3

impl Entity for Rotation3<f64> {
    impl_entity_unwrap!(Quaternion<f64>);
}
impl SizedEntity for Rotation3<f64> {
    impl_sized_entity_unwrap!(Quaternion<f64>);
}
impl Sourced for Rotation3<f64> {
    fn source(_: &Config) -> SourceInfo {
        SourceInfo::with_root("Rotation3", "rot3", "algebra/rotation.hh")
    }
}

// Linear

impl Entity for Linear<f64, 2> {
    impl_entity_unwrap!(Matrix<f64, 2, 2>);
}
impl SizedEntity for Linear<f64, 2> {
    impl_sized_entity_unwrap!(Matrix<f64, 2, 2>);
}
impl Sourced for Linear<f64, 2> {
    fn source(_: &Config) -> SourceInfo {
        SourceInfo::with_root("Linear2", "lin2", "algebra/linear.hh")
    }
}

impl Entity for Linear<f64, 3> {
    impl_entity_unwrap!(Matrix<f64, 3, 3>);
}
impl SizedEntity for Linear<f64, 3> {
    impl_sized_entity_unwrap!(Matrix<f64, 3, 3>);
}
impl Sourced for Linear<f64, 3> {
    fn source(_: &Config) -> SourceInfo {
        SourceInfo::with_root("Linear3", "lin3", "algebra/linear.hh")
    }
}

// Moebius

impl Entity for Moebius<Complex<f64>> {
    impl_entity_unwrap!(Matrix<Complex<f64>, 2, 2>);
}
impl SizedEntity for Moebius<Complex<f64>> {
    impl_sized_entity_unwrap!(Matrix<Complex<f64>, 2, 2>);
}
impl Sourced for Moebius<Complex<f64>> {
    fn source(_: &Config) -> SourceInfo {
        SourceInfo::with_root("Moebius", "mo", "algebra/moebius.hh")
    }
}

// Chain

impl<A, B, T> Entity for Chain<A, B, T> where A: Map<T> + Reorder<B, T>, B: Map<T> + Reorder<A, T>, T: SizedEntity + Copy {
    fn align(cfg: &Config) -> usize {
        math::lcm(A::align(cfg), B::align(cfg))
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        let a = A::load(cfg, src)?;
        src.align(B::align(cfg))?;
        let b = B::load(cfg, src)?;
        src.align(Self::align(cfg))?;
        Ok(Self::new(a, b))
    }
    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        self.outer().store(cfg, dst)?;
        dst.align(B::align(cfg))?;
        self.inner().store(cfg, dst)?;
        dst.align(Self::align(cfg))?;
        Ok(())
    }
    fn entity_source(_cfg: &Config) -> SourceInfo {
        unimplemented!()
        // FIXME: build source
        //let a = A::entity_source(cfg);
        //let b = B::entity_source(cfg);
        //SourceInfo::new("", "", "")
    }
}
impl<A, B, T> SizedEntity for Chain<A, B, T> where A: Map<T> + Reorder<B, T>, B: Map<T> + Reorder<A, T>, T: SizedEntity + Copy {
    fn type_size(cfg: &Config) -> usize {
        math::upper_multiple(
            math::upper_multiple(A::type_size(cfg), B::align(cfg)) + B::type_size(cfg),
            Self::align(cfg)
        )
    }
}
impl<A, B, T> Sourced for Chain<A, B, T> where A: Map<T> + Reorder<B, T>, B: Map<T> + Reorder<A, T>, T: SizedEntity + Copy {
    fn source(cfg: &Config) -> SourceInfo {
        Self::entity_source(cfg)
    }
}
