//! The same physical marker arrangement in each constant-curvature space.
//!
//! Every marker has radius 0.12. From top to bottom, the amber, green, and blue
//! rows lie 0.8, 1.6, and 2.5 physical units from the camera. Their viewing
//! directions stay fixed, so changes in apparent size expose the geometry.
//! In unit-radius spherical space the last row lies beyond the equator and
//! appears larger again; hyperbolic space makes distant markers smaller.
//!
//! Markers emit light and absorb incident paths, giving clean silhouettes
//! without indirect-light noise. Increasing the curvature radius approaches
//! the Euclidean image while preserving physical object sizes and distances.

use anyhow::{Context as _, ensure};
use ccgeom::{Embedded3, EmbeddedIsometry, Space3};
use objects::{
    Mapped, SceneImpl,
    background::ConstBg,
    material::{Absorbing, Emissive},
    object::Covered,
    shader::Result,
    shape::GeodesicSphere,
    view::PointView,
};

type Geometry<const K: i8> = Embedded3<f64, K>;
type Marker<const K: i8> = Mapped<
    Geometry<K>,
    Covered<Geometry<K>, GeodesicSphere, Emissive<Absorbing>>,
    EmbeddedIsometry<f64, K>,
>;

pub type ExampleScene<const K: i8, const H: usize> = SceneImpl<
    Geometry<K>,
    Mapped<Geometry<K>, PointView<Geometry<K>>, EmbeddedIsometry<f64, K>>,
    Vec<Marker<K>>,
    ConstBg,
    H,
>;

const MARKER_RADIUS: f64 = 0.12;
const ROWS: [(f64, f64, [f32; 3]); 3] = [
    (0.8, 0.45, [0.95, 0.47, 0.13]),
    (1.6, 0.0, [0.13, 0.70, 0.48]),
    (2.5, -0.45, [0.15, 0.40, 0.95]),
];

/// Build the shared comparison, using physical distances for every placement.
///
/// `K` is -1, 0, or +1. Euclidean space requires `radius == 1`. Spherical
/// radii must keep the far markers before the camera's antipode, so the row
/// distances remain their shortest distances from the camera.
pub fn scene<const K: i8, const H: usize>(radius: f64) -> Result<ExampleScene<K, H>> {
    let space = Space3::<f64, K>::new(radius).context("invalid comparison curvature radius")?;
    ensure!(
        K != 1 || ROWS[2].0 + MARKER_RADIUS < std::f64::consts::PI * radius,
        "comparison markers must lie before the spherical antipode"
    );
    let mut markers = Vec::with_capacity(9);
    for (distance, y, color) in ROWS {
        for x in [-0.65, 0.0, 0.65] {
            let map = space
                .translation([x, y, -1.0].into(), distance)
                .context("comparison marker placement exceeds numerical range")?;
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
    scene.radius = radius;
    Ok(scene)
}

#[cfg(test)]
mod tests {
    use super::*;
    use objects::{Scene as _, shader};
    use vecmat::Vector;

    fn endpoint(map: shader::Transform) -> Vector<f64, 4> {
        let origin = [1.0, 0.0, 0.0, 0.0].into();
        match map {
            shader::Transform::Flat(map) => map.apply_vector(origin),
            shader::Transform::Hyperboloid(map) => map.apply_vector(origin),
            shader::Transform::Spherical(map) => map.apply_vector(origin),
            _ => panic!("comparison must lower directly to embedded maps"),
        }
    }

    fn check_physical_layout<const K: i8>(radius: f64) -> Vec<u32> {
        let example = scene::<K, 2>(radius).unwrap();
        let definition = example.definition().unwrap();
        let compiled = shader::compile(&definition).unwrap();
        assert_eq!(compiled.geometry.sign(), K);
        assert_eq!(compiled.transforms.len(), 9);
        assert!(matches!(definition.medium, shader::Medium::Vacuum));
        assert_eq!(definition.radius, radius);
        assert!(matches!(
            definition.background,
            shader::Background::Constant([0.0, 0.0, 0.0])
        ));

        let space = Space3::<f64, K>::new(radius).unwrap();
        for (i, map) in compiled.transforms.iter().enumerate() {
            let p = endpoint(*map);
            let distance = space.distance(space.origin(), p);
            let expected = [0.8, 1.6, 2.5][i / 3];
            assert!((distance - expected).abs() < 1e-12);
            assert_eq!(example.object[i].inner.shape.radius, 0.12);
            // All geometries must preserve the same camera directions as
            // well as distances, rather than accidentally using chart units.
            let expected_direction =
                Vector::from([[-0.65, 0.0, 0.65][i % 3], [0.45, 0.0, -0.45][i / 3], -1.0])
                    .normalize();
            let direction = Vector::from([p[1], p[2], p[3]]).normalize();
            assert!((direction - expected_direction).length() < 1e-12);
        }
        compiled.words
    }

    #[test]
    fn comparison_preserves_physical_layout_and_payloads_across_curvatures() {
        let euclidean = check_physical_layout::<0>(1.0);
        assert_eq!(check_physical_layout::<-1>(1.0), euclidean);
        assert_eq!(check_physical_layout::<1>(1.0), euclidean);
        assert_eq!(check_physical_layout::<-1>(3.0), euclidean);
        assert_eq!(check_physical_layout::<1>(3.0), euclidean);
    }

    #[test]
    fn invalid_or_wrapping_comparison_radii_return_errors() {
        assert!(scene::<0, 2>(3.0).is_err());
        assert!(scene::<1, 2>(0.5).is_err());
        assert!(scene::<-1, 2>(0.0).is_err());
        assert!(scene::<1, 2>(f64::NAN).is_err());
        assert!(scene::<-1, 2>(f64::INFINITY).is_err());
    }
}
