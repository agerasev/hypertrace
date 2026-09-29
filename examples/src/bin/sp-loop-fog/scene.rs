//! Homogeneous fog on spherical paths that circle the world.
//!
//! Four emissive beacons lie behind the camera. Spherical geodesics bring their
//! light into the forward view by the long route. With no floor, a ray missing
//! all beacons can scatter after multiple complete circuits. Extinction 0.1
//! gives a mean free flight of ten physical units, longer than one circuit.
//! Twelve interactions allow several scattering events before termination.

use anyhow::Context as _;
use ccgeom::{EmbeddedIsometry, Space3, Spherical3};
use objects::{
    Mapped, SceneImpl,
    background::ConstBg,
    material::{Absorbing, Emissive},
    object::Covered,
    shader::{Medium, Result},
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
    12,
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
    let mut scene = SceneImpl::new(
        Mapped::new(PointView::new(1.0), EmbeddedIsometry::identity()),
        beacons,
        ConstBg::new([0.0; 3].into()),
    );
    scene.medium = Medium::homogeneous(0.1, [0.9; 3]);
    Ok(scene)
}
