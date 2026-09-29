//! Light arriving from behind the camera after taking the long way around S³.
//!
//! The identity camera looks along -z. Four beacons lie behind it, toward +z,
//! yet appear ahead because spherical geodesics close after 2πR physical units.
//! The central cyan beacon is one unit away with radius 0.24: its first forward
//! surface is 2π - 1 - 0.24 units away. Turn around to see the short route,
//! only 1 - 0.24 units long. There is no floor to interrupt the long route;
//! absorbing emitters terminate each vacuum path at its first surface.

use anyhow::Context as _;
use ccgeom::{EmbeddedIsometry, Space3, Spherical3};
use objects::{
    Mapped, SceneImpl,
    background::ConstBg,
    material::{Absorbing, Emissive},
    object::Covered,
    shader::Result,
    shape::GeodesicSphere,
    view::PointView,
};

type Beacon = Mapped<
    Spherical3,
    Covered<Spherical3, GeodesicSphere, Emissive<Absorbing>>,
    EmbeddedIsometry<f64, 1>,
>;

pub type ExampleScene = SceneImpl<
    Spherical3,
    Mapped<Spherical3, PointView<Spherical3>, EmbeddedIsometry<f64, 1>>,
    Vec<Beacon>,
    ConstBg,
    1,
>;

fn beacon(position: [f64; 3], radius: f64, emission: [f32; 3]) -> Result<Beacon> {
    let distance = position.iter().map(|v| v * v).sum::<f64>().sqrt();
    let map = Space3::<f64, 1>::unit()
        .translation(position.into(), distance)
        .context("beacon placement exceeds numerical range")?;
    Ok(Mapped::new(
        Covered::new(
            GeodesicSphere::new(radius),
            Emissive::new(Absorbing, emission.into()),
        ),
        map,
    ))
}

/// Four absorbing emitters in black, unit-radius spherical space.
pub fn scene() -> Result<ExampleScene> {
    let beacons = vec![
        beacon([0.0, 0.0, 1.0], 0.24, [0.25, 1.15, 1.6])?,
        beacon([-0.7, 0.35, 1.25], 0.15, [1.5, 0.45, 0.12])?,
        beacon([0.65, 0.22, 1.35], 0.12, [0.75, 0.3, 1.4])?,
        beacon([0.05, -0.65, 1.3], 0.14, [1.4, 1.05, 0.25])?,
    ];
    let scene = SceneImpl::new(
        Mapped::new(PointView::new(1.0), EmbeddedIsometry::identity()),
        beacons,
        ConstBg::new([0.0; 3].into()),
    );
    Ok(scene)
}
