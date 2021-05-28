use crate::{
    include_template,
    io::{CntRead, CntWrite},
    math,
    source::{SourceBuilder, SourceTree},
    Config, Entity, EntityId, SizedEntity, EntitySource,
};
use base::{
    ccgeom::Map as GeoMap,
    vecmat::{
        complex::{Complex, Quaternion},
        transform::{Chain, Linear, Moebius, Reorder, Rotation2, Rotation3, Shift},
        Matrix, Vector,
    },
};
use std::{any::TypeId, io};

pub trait Map<P, D = P>: GeoMap<P, D> + SizedEntity {
    fn map_name() -> (String, String);
    fn map_source(cfg: &Config) -> SourceTree;
}

impl<M, P, D> Map<P, D> for M
where
    M: GeoMap<P, D> + SizedEntity,
{
    fn map_name() -> (String, String) {
        Self::name()
    }
    fn map_source(cfg: &Config) -> SourceTree {
        Self::source(cfg)
    }
}

// Shift

impl EntityId for Shift<f64, 2> {
    fn name() -> (String, String) {
        ("Shift2".into(), "shf2".into())
    }
}
impl Entity for Shift<f64, 2> {
    impl_entity_unwrap!(Vector<f64, 2>);
}
impl SizedEntity for Shift<f64, 2> {
    impl_sized_entity_unwrap!(Vector<f64, 2>);
}
impl EntitySource for Shift<f64, 2> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/shift.hh")
    }
}

impl EntityId for Shift<f64, 3> {
    fn name() -> (String, String) {
        ("Shift3".into(), "shf3".into())
    }
}
impl Entity for Shift<f64, 3> {
    impl_entity_unwrap!(Vector<f64, 3>);
}
impl SizedEntity for Shift<f64, 3> {
    impl_sized_entity_unwrap!(Vector<f64, 3>);
}
impl EntitySource for Shift<f64, 3> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/shift.hh")
    }
}

// Rotation2

impl EntityId for Rotation2<f64> {
    fn name() -> (String, String) {
        ("Rotation2".into(), "rot2".into())
    }
}
impl Entity for Rotation2<f64> {
    impl_entity_unwrap!(Complex<f64>);
}
impl SizedEntity for Rotation2<f64> {
    impl_sized_entity_unwrap!(Complex<f64>);
}
impl EntitySource for Rotation2<f64> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/rotation.hh")
    }
}

// Rotation3

impl EntityId for Rotation3<f64> {
    fn name() -> (String, String) {
        ("Rotation3".into(), "rot3".into())
    }
}
impl Entity for Rotation3<f64> {
    impl_entity_unwrap!(Quaternion<f64>);
}
impl SizedEntity for Rotation3<f64> {
    impl_sized_entity_unwrap!(Quaternion<f64>);
}
impl EntitySource for Rotation3<f64> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/rotation.hh")
    }
}

// Linear

impl EntityId for Linear<f64, 2> {
    fn name() -> (String, String) {
        ("Linear2".into(), "lin2".into())
    }
}
impl Entity for Linear<f64, 2> {
    impl_entity_unwrap!(Matrix<f64, 2, 2>);
}
impl SizedEntity for Linear<f64, 2> {
    impl_sized_entity_unwrap!(Matrix<f64, 2, 2>);
}
impl EntitySource for Linear<f64, 2> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/linear.hh")
    }
}

impl EntityId for Linear<f64, 3> {
    fn name() -> (String, String) {
        ("Linear3".into(), "lin3".into())
    }
}
impl Entity for Linear<f64, 3> {
    impl_entity_unwrap!(Matrix<f64, 3, 3>);
}
impl SizedEntity for Linear<f64, 3> {
    impl_sized_entity_unwrap!(Matrix<f64, 3, 3>);
}
impl EntitySource for Linear<f64, 3> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("transform/linear.hh")
    }
}

// Moebius

impl EntityId for Moebius<Complex<f64>> {
    fn name() -> (String, String) {
        ("Moebius".into(), "mo".into())
    }
}
impl Entity for Moebius<Complex<f64>> {
    impl_entity_unwrap!(Matrix<Complex<f64>, 2, 2>);
}
impl SizedEntity for Moebius<Complex<f64>> {
    impl_sized_entity_unwrap!(Matrix<Complex<f64>, 2, 2>);
}
impl EntitySource for Moebius<Complex<f64>> {
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

impl<A, B, T> EntityId for Chain<A, B, T>
where
    A: Map<T> + Reorder<B, T>,
    B: Map<T> + Reorder<A, T>,
    T: SizedEntity + Copy,
{
    fn name() -> (String, String) {
        if TypeId::of::<Self>() == hom3_id() {
            ("Homogenous3".into(), "hom3".into())
        } else {
            (
                format!("Chain{}", Self::tag()),
                format!("chain_{}", Self::tag()),
            )
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
        Self::static_size(cfg)
    }
    fn min_size(cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn is_dyn_sized() -> bool {
        false
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
}
impl<A, B, T> SizedEntity for Chain<A, B, T>
where
    A: Map<T> + Reorder<B, T>,
    B: Map<T> + Reorder<A, T>,
    T: SizedEntity + Copy,
{
    fn static_size(cfg: &Config) -> usize {
        math::upper_multiple(
            math::upper_multiple(A::static_size(cfg), B::align(cfg)) + B::static_size(cfg),
            Self::align(cfg),
        )
    }
}
impl<A, B, T> EntitySource for Chain<A, B, T>
where
    A: Map<T> + Reorder<B, T>,
    B: Map<T> + Reorder<A, T>,
    T: SizedEntity + Copy,
{
    fn source(cfg: &Config) -> SourceTree {
        if TypeId::of::<Self>() == hom3_id() {
            SourceTree::new("transform/homogenous.hh")
        } else {
            SourceBuilder::new(format!("generated/chain_{}.hh", Self::tag()))
                .tree(A::source(cfg))
                .tree(B::source(cfg))
                .content(&include_template!(
                    "transform/chain.inl",
                    ("Self", "self") => Self::name(),
                    ("Outer", "outer") => A::name(),
                    ("Inner", "inner") => B::name(),
                    "elem" => T::name().0,
                ))
                .build()
        }
    }
}
