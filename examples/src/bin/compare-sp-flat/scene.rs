//! Gentler spherical curvature with the same physical marker layout.
//!
//! Every marker has radius 0.12. From top to bottom, the amber, green and blue
//! rows lie 0.8, 1.6 and 2.5 physical units from the camera. Viewing directions
//! stay fixed across the comparison demos. A curvature radius of three
//! keeps all rows before the equator and brings their apparent sizes
//! closer to the flat-space view.
//! Emission and absorption produce clear silhouettes without indirect-light noise.

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

type Marker = Mapped<
    Spherical3,
    Covered<Spherical3, GeodesicSphere, Emissive<Absorbing>>,
    EmbeddedIsometry<f64, 1>,
>;

pub type ExampleScene = SceneImpl<
    Spherical3,
    Mapped<Spherical3, PointView<Spherical3>, EmbeddedIsometry<f64, 1>>,
    Vec<Marker>,
    ConstBg,
    1,
>;

const CURVATURE_RADIUS: f64 = 3.0;
const MARKER_RADIUS: f64 = 0.12;
const ROWS: [(f64, f64, [f32; 3]); 3] = [
    (0.8, 0.45, [0.95, 0.47, 0.13]),
    (1.6, 0.0, [0.13, 0.70, 0.48]),
    (2.5, -0.45, [0.15, 0.40, 0.95]),
];

/// Place nine emissive spheres using physical distances in this demo's geometry.
pub fn scene() -> Result<ExampleScene> {
    let space =
        Space3::<f64, 1>::new(CURVATURE_RADIUS).context("invalid marker scene curvature radius")?;
    let mut markers = Vec::with_capacity(9);
    for (distance, y, color) in ROWS {
        for x in [-0.65, 0.0, 0.65] {
            let map = space
                .translation([x, y, -1.0].into(), distance)
                .context("marker placement exceeds numerical range")?;
            markers.push(Mapped::new(
                Covered::new(
                    GeodesicSphere::new(MARKER_RADIUS),
                    Emissive::new(Absorbing, color.into()),
                ),
                map,
            ));
        }
    }
    let mut scene = SceneImpl::new(
        Mapped::new(PointView::new(1.0), EmbeddedIsometry::identity()),
        markers,
        ConstBg::new([0.0; 3].into()),
    );
    scene.radius = CURVATURE_RADIUS;
    Ok(scene)
}
