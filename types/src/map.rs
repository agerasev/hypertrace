use crate::{
    include_template,
    io::{CntRead, CntWrite},
    math,
    source::{SourceBuilder, SourceTree, Sourced},
    Config, Entity, Named, SizedEntity,
};
use ccgeom::Map as GeoMap;
use std::{any::TypeId, io};
use vecmat::{
    complex::{Complex, Quaternion},
    transform::{Chain, Linear, Moebius, Reorder, Rotation2, Rotation3, Shift},
    Matrix, Vector,
};

pub trait Map<P, D = P>: GeoMap<P, D> + SizedEntity + Sourced {}

impl<M, P, D> Map<P, D> for M where M: GeoMap<P, D> + SizedEntity + Sourced {}

// Shift

impl Named for Shift<f64, 2> {
    fn type_name(_: &Config) -> String {
        "Shift2".into()
    }
    fn type_prefix(_: &Config) -> String {
        "shf2".into()
    }
}
impl Entity for Shift<f64, 2> {
    impl_entity_unwrap!(Vector<f64, 2>);
}
impl SizedEntity for Shift<f64, 2> {
    impl_sized_entity_unwrap!(Vector<f64, 2>);
}
impl Sourced for Shift<f64, 2> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/shift.hh")
    }
}

impl Named for Shift<f64, 3> {
    fn type_name(_: &Config) -> String {
        "Shift3".into()
    }
    fn type_prefix(_: &Config) -> String {
        "shf3".into()
    }
}
impl Entity for Shift<f64, 3> {
    impl_entity_unwrap!(Vector<f64, 3>);
}
impl SizedEntity for Shift<f64, 3> {
    impl_sized_entity_unwrap!(Vector<f64, 3>);
}
impl Sourced for Shift<f64, 3> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/shift.hh")
    }
}

// Rotation2

impl Named for Rotation2<f64> {
    fn type_name(_: &Config) -> String {
        "Rotation2".into()
    }
    fn type_prefix(_: &Config) -> String {
        "rot2".into()
    }
}
impl Entity for Rotation2<f64> {
    impl_entity_unwrap!(Complex<f64>);
}
impl SizedEntity for Rotation2<f64> {
    impl_sized_entity_unwrap!(Complex<f64>);
}
impl Sourced for Rotation2<f64> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/rotation.hh")
    }
}

// Rotation3

impl Named for Rotation3<f64> {
    fn type_name(_: &Config) -> String {
        "Rotation3".into()
    }
    fn type_prefix(_: &Config) -> String {
        "rot3".into()
    }
}
impl Entity for Rotation3<f64> {
    impl_entity_unwrap!(Quaternion<f64>);
}
impl SizedEntity for Rotation3<f64> {
    impl_sized_entity_unwrap!(Quaternion<f64>);
}
impl Sourced for Rotation3<f64> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/rotation.hh")
    }
}

// Linear

impl Named for Linear<f64, 2> {
    fn type_name(_: &Config) -> String {
        "Linear2".into()
    }
    fn type_prefix(_: &Config) -> String {
        "lin2".into()
    }
}
impl Entity for Linear<f64, 2> {
    impl_entity_unwrap!(Matrix<f64, 2, 2>);
}
impl SizedEntity for Linear<f64, 2> {
    impl_sized_entity_unwrap!(Matrix<f64, 2, 2>);
}
impl Sourced for Linear<f64, 2> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/linear.hh")
    }
}

impl Named for Linear<f64, 3> {
    fn type_name(_: &Config) -> String {
        "Linear3".into()
    }
    fn type_prefix(_: &Config) -> String {
        "lin3".into()
    }
}
impl Entity for Linear<f64, 3> {
    impl_entity_unwrap!(Matrix<f64, 3, 3>);
}
impl SizedEntity for Linear<f64, 3> {
    impl_sized_entity_unwrap!(Matrix<f64, 3, 3>);
}
impl Sourced for Linear<f64, 3> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/linear.hh")
    }
}

// Moebius

impl Named for Moebius<Complex<f64>> {
    fn type_name(_: &Config) -> String {
        "Moebius".into()
    }
    fn type_prefix(_: &Config) -> String {
        "mo".into()
    }
}
impl Entity for Moebius<Complex<f64>> {
    impl_entity_unwrap!(Matrix<Complex<f64>, 2, 2>);
}
impl SizedEntity for Moebius<Complex<f64>> {
    impl_sized_entity_unwrap!(Matrix<Complex<f64>, 2, 2>);
}
impl Sourced for Moebius<Complex<f64>> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/moebius.hh")
    }
}

// Chain

fn hom3_id() -> TypeId {
    TypeId::of::<Chain<Shift<f64, 3>, Rotation3<f64>, Vector<f64, 3>>>()
}
// TODO: Support compact affine transform.
//const AFF3_ID: TypeId = TypeId::of::<Chain<Shift<f64, 3>, Linear<f64, 3>, Vector<f64, 3>>>();

impl<A, B, T> Named for Chain<A, B, T>
where
    A: Map<T> + Reorder<B, T>,
    B: Map<T> + Reorder<A, T>,
    T: SizedEntity + Copy,
{
    fn type_name(_: &Config) -> String {
        if TypeId::of::<Self>() == hom3_id() {
            "Homogenous3".into()
        } else {
            format!("Chain{}", Self::type_tag())
        }
    }
    fn type_prefix(_: &Config) -> String {
        if TypeId::of::<Self>() == hom3_id() {
            "hom3".into()
        } else {
            format!("chain_{}", Self::type_tag())
        }
    }
}
impl<A, B, T> Entity for Chain<A, B, T>
where
    A: Map<T> + Reorder<B, T>,
    B: Map<T> + Reorder<A, T>,
    T: SizedEntity + Copy,
{
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
    fn type_source(cfg: &Config) -> SourceTree {
        if TypeId::of::<Self>() == hom3_id() {
            SourceTree::new("transfomrm/homogenous.hh")
        } else {
            SourceBuilder::new(format!("generated/chain_{}.hh", Self::type_tag()))
                .tree(A::type_source(cfg))
                .tree(B::type_source(cfg))
                .content(&include_template!(
                    "transform/chain.inl",
                    "Self": &Self::type_name(cfg),
                    "self": &Self::type_prefix(cfg),
                    "Outer": &A::type_name(cfg),
                    "outer": &A::type_prefix(cfg),
                    "Inner": &B::type_name(cfg),
                    "inner": &B::type_prefix(cfg),
                    "elem": &T::type_name(cfg),
                ))
                .build()
        }
    }
}
impl<A, B, T> SizedEntity for Chain<A, B, T>
where
    A: Map<T> + Reorder<B, T>,
    B: Map<T> + Reorder<A, T>,
    T: SizedEntity + Copy,
{
    fn type_size(cfg: &Config) -> usize {
        math::upper_multiple(
            math::upper_multiple(A::type_size(cfg), B::align(cfg)) + B::type_size(cfg),
            Self::align(cfg),
        )
    }
}
impl<A, B, T> Sourced for Chain<A, B, T>
where
    A: Map<T> + Reorder<B, T>,
    B: Map<T> + Reorder<A, T>,
    T: SizedEntity + Copy,
{
    fn source(cfg: &Config) -> SourceTree {
        Self::type_source(cfg)
    }
}
