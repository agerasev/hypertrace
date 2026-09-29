use anyhow::{Context as _, ensure};
use ccgeom::embedded::{Embedded3, EmbeddedIsometry, Space3};
use std::fmt::Debug;
use vecmat::{QuaternionPair, Vector};

use crate::Result;

mod sealed {
    pub trait Sealed {}
    impl Sealed for ccgeom::Embedded3<f64, 0> {}
    impl Sealed for ccgeom::Embedded3<f64, -1> {}
    impl Sealed for ccgeom::Embedded3<f64, 1> {}
}

/// A supported canonical embedded geometry, selected at compile time.
///
/// Unsupported curvature signs cannot construct scenes:
/// ```compile_fail
/// use hypertrace_scene::Transform;
/// let _ = Transform::<ccgeom::Embedded3<f64, 2>>::identity();
/// ```
pub trait Geometry:
    sealed::Sealed
    + ccgeom::Geometry3<f64, Pos = Vector<f64, 4>, Dir = Vector<f64, 4>, Map: Copy + Debug>
    + Copy
    + Debug
{
    const SIGN: i8;
    #[doc(hidden)]
    fn map_from_components(values: [f64; 8]) -> Result<Self::Map>;
    #[doc(hidden)]
    fn map_components(map: Self::Map) -> [f64; 8];
    #[doc(hidden)]
    fn map_identity() -> Self::Map;
    #[doc(hidden)]
    fn map_apply(map: Self::Map, vector: [f64; 4]) -> [f64; 4];
    #[doc(hidden)]
    fn map_inverse(map: Self::Map) -> Result<Self::Map>;
    #[doc(hidden)]
    fn map_chain(outer: Self::Map, inner: Self::Map) -> Result<Self::Map>;
    #[doc(hidden)]
    fn map_move(
        map: Self::Map,
        translation: [f64; 3],
        rotation: [f64; 3],
        radius: f64,
    ) -> Result<Self::Map>;
}

fn finite_action<const K: i8>(map: EmbeddedIsometry<f64, K>) -> Result<EmbeddedIsometry<f64, K>> {
    ensure!(
        map.apply_vector([1.0, 0.0, 0.0, 0.0].into())
            .into_iter()
            .all(f64::is_finite),
        "isometry point action exceeds numerical range"
    );
    Ok(map)
}
fn compose<const K: i8>(
    a: EmbeddedIsometry<f64, K>,
    b: EmbeddedIsometry<f64, K>,
) -> Result<EmbeddedIsometry<f64, K>> {
    finite_action(
        EmbeddedIsometry::from_pair(a.pair() * b.pair())
            .context("isometry composition exceeds numerical range")?,
    )
}
fn moved<const K: i8>(
    mut map: EmbeddedIsometry<f64, K>,
    translation: [f64; 3],
    rotation: [f64; 3],
    radius: f64,
) -> Result<EmbeddedIsometry<f64, K>> {
    let space = Space3::<f64, K>::new(radius).context("invalid curvature radius")?;
    for (axis, distance) in translation.into_iter().enumerate() {
        let mut direction = [0.0; 3];
        direction[axis] = 1.0;
        map = compose(
            map,
            space
                .translation(direction.into(), distance)
                .context("invalid camera translation")?,
        )?;
    }
    for (axis, angle) in rotation.into_iter().enumerate() {
        let mut direction = [0.0; 3];
        direction[axis] = 1.0;
        map = compose(
            map,
            EmbeddedIsometry::rotation(direction.into(), angle)
                .context("invalid camera rotation")?,
        )?;
    }
    Ok(map)
}
macro_rules! geometry {
    ($sign:literal) => {
        impl Geometry for Embedded3<f64, $sign> {
            const SIGN: i8 = $sign;
            fn map_from_components(values: [f64; 8]) -> Result<Self::Map> {
                finite_action(
                    EmbeddedIsometry::from_pair(QuaternionPair::from_array(values))
                        .context("invalid embedded isometry")?,
                )
            }
            fn map_components(map: Self::Map) -> [f64; 8] {
                map.pair().into_array()
            }
            fn map_identity() -> Self::Map {
                EmbeddedIsometry::identity()
            }
            fn map_apply(map: Self::Map, vector: [f64; 4]) -> [f64; 4] {
                map.apply_vector(vector.into()).into()
            }
            fn map_inverse(map: Self::Map) -> Result<Self::Map> {
                finite_action(map.inv())
            }
            fn map_chain(outer: Self::Map, inner: Self::Map) -> Result<Self::Map> {
                compose(outer, inner)
            }
            fn map_move(
                map: Self::Map,
                translation: [f64; 3],
                rotation: [f64; 3],
                radius: f64,
            ) -> Result<Self::Map> {
                moved(map, translation, rotation, radius)
            }
        }
    };
}
geometry!(0);
geometry!(-1);
geometry!(1);

/// A canonical f64 quaternion-pair isometry with statically selected curvature.
///
/// Maps from another geometry cannot be supplied:
/// ```compile_fail
/// use hypertrace_scene::Transform;
/// use ccgeom::{Flat3, Hyperboloid3, Geometry3};
/// let _ = Transform::<Flat3>::from_isometry(Hyperboloid3::shift_x(1.0));
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Transform<G: Geometry> {
    isometry: G::Map,
}
impl<G: Geometry> Transform<G> {
    pub fn identity() -> Self {
        Self {
            isometry: G::map_identity(),
        }
    }
    pub fn from_isometry(map: G::Map) -> Result<Self> {
        Ok(Self {
            isometry: G::map_from_components(G::map_components(map))?,
        })
    }
    pub fn chain(&self, inner: &Self) -> Result<Self> {
        Ok(Self {
            isometry: G::map_chain(self.isometry, inner.isometry)?,
        })
    }
    pub fn inverse(self) -> Result<Self> {
        Ok(Self {
            isometry: G::map_inverse(self.isometry)?,
        })
    }
    /// Apply the isometry to an ambient point or tangent in scalar-first order.
    pub fn apply_vector(self, vector: [f64; 4]) -> [f64; 4] {
        G::map_apply(self.isometry, vector)
    }
    /// Keep f64 canonical transforms until camera-relative preparation.
    pub fn components(self) -> Result<[[f64; 4]; 2]> {
        let values = G::map_components(self.isometry);
        ensure!(values.iter().all(|x| x.is_finite()), "non-finite isometry");
        Ok([
            values[..4].try_into().unwrap(),
            values[4..].try_into().unwrap(),
        ])
    }
    pub fn rows(&self) -> Result<[[f32; 4]; 2]> {
        let rows = self.components()?.map(|r| r.map(|v| v as f32));
        validate_embedded_rows::<G>(rows)?;
        Ok(rows)
    }
    pub fn words(&self) -> Result<Vec<u32>> {
        Ok(self
            .rows()?
            .into_iter()
            .flatten()
            .map(f32::to_bits)
            .collect())
    }
    pub fn from_rows(rows: [[f32; 4]; 2]) -> Result<Self> {
        validate_embedded_rows::<G>(rows)?;
        Ok(Self {
            isometry: G::map_from_components(std::array::from_fn(|i| {
                f64::from(rows[i / 4][i % 4])
            }))?,
        })
    }
    pub fn move_local(
        self,
        translation: [f64; 3],
        rotation: [f64; 3],
        radius: f64,
    ) -> Result<Self> {
        Ok(Self {
            isometry: G::map_move(self.isometry, translation, rotation, radius)?,
        })
    }
}

/// Check the stored unit-pair constraints after f32 conversion. The absolute
/// error budget does not grow with the boost: otherwise cancellation would
/// permit transforms with norm far from one. Camera-relative preparation must
/// happen before this check, while canonical transforms remain in f64.
pub fn validate_embedded_rows<G: Geometry>(rows: [[f32; 4]; 2]) -> Result<()> {
    const MAX_METRIC_ERROR: f64 = 1e-3;
    ensure!(
        rows.iter().flatten().all(|x| x.is_finite()),
        "transform is outside f32 range"
    );
    let [a, b] = rows.map(|r| r.map(f64::from));
    let aa = a.iter().map(|x| x * x).sum::<f64>();
    let bb = b.iter().map(|x| x * x).sum::<f64>();
    let ab = a.iter().zip(b).map(|(a, b)| a * b).sum::<f64>();
    let sign = f64::from(G::SIGN);
    let norm = aa + sign * bb;
    // A conservative rounding budget for dot products, their subtraction, and
    // the pair sandwich. This also prevents a cast with accidental cancellation
    // agreement from passing while equivalent GPU arithmetic loses the metric.
    let cancellation_budget = 16.0 * f64::from(f32::EPSILON) * (aa + sign.abs() * bb).max(1.0);
    ensure!(
        cancellation_budget <= MAX_METRIC_ERROR,
        "embedded isometry exceeds the f32 metric precision budget; use relative coordinates"
    );
    ensure!(
        (norm - 1.0).abs() <= MAX_METRIC_ERROR,
        "embedded isometry must have unit norm after f32 conversion"
    );
    ensure!(
        ab.abs() <= MAX_METRIC_ERROR,
        "embedded isometry components must be orthogonal after f32 conversion"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ccgeom::{Flat3, Geometry3, Hyperboloid3, Spherical3};

    fn apply<G: Geometry>(transform: Transform<G>, p: [f64; 4]) -> [f64; 4] {
        transform.apply_vector(p)
    }
    fn close(a: [f64; 4], b: [f64; 4], tolerance: f64) {
        for i in 0..4 {
            assert!((a[i] - b[i]).abs() <= tolerance, "{a:?} != {b:?}");
        }
    }
    #[test]
    fn composition_inverse_and_upload_roundtrip_cover_all_signs() -> Result<()> {
        fn check<G: Geometry>() -> Result<()> {
            let a =
                Transform::<G>::identity().move_local([0.3, -0.2, 0.4], [0.1, 0.2, -0.3], 1.0)?;
            let b =
                Transform::<G>::identity().move_local([-0.1, 0.3, 0.2], [0.3, 0.2, 0.1], 1.0)?;
            let p = [1.0, 0.0, 0.0, 0.0];
            close(apply(a.chain(&b)?, p), apply(a, apply(b, p)), 3e-14);
            close(apply(a.inverse()?, apply(a, p)), p, 3e-14);
            close(
                apply(Transform::<G>::from_rows(a.rows()?)?, p),
                apply(a, p),
                1e-7,
            );
            Ok(())
        }
        check::<Flat3>()?;
        check::<Hyperboloid3>()?;
        check::<Spherical3>()?;
        Ok(())
    }
    #[test]
    fn local_movement_uses_physical_radius_and_preserves_full_circuits() -> Result<()> {
        let radius = 2.3;
        let distance = 0.7;
        let sphere = Transform::<Spherical3>::identity().move_local(
            [distance, 0.0, 0.0],
            [0.0; 3],
            radius,
        )?;
        close(
            apply(sphere, [1.0, 0.0, 0.0, 0.0]),
            [
                (distance / radius).cos(),
                (distance / radius).sin(),
                0.0,
                0.0,
            ],
            2e-15,
        );
        let full = Transform::<Spherical3>::identity().move_local(
            [std::f64::consts::TAU * radius, 0.0, 0.0],
            [0.0; 3],
            radius,
        )?;
        close(
            apply(full, [1.0, 0.0, 0.0, 0.0]),
            [1.0, 0.0, 0.0, 0.0],
            2e-15,
        );
        let hyper = Transform::<Hyperboloid3>::identity().move_local(
            [distance, 0.0, 0.0],
            [0.0; 3],
            radius,
        )?;
        close(
            apply(hyper, [1.0, 0.0, 0.0, 0.0]),
            [
                (distance / radius).cosh(),
                (distance / radius).sinh(),
                0.0,
                0.0,
            ],
            2e-15,
        );
        assert!(
            Transform::<Flat3>::identity()
                .move_local([0.0; 3], [0.0; 3], radius)
                .is_err()
        );
        assert!(
            sphere
                .move_local([f64::INFINITY, 0.0, 0.0], [0.0; 3], radius)
                .is_err()
        );
        Ok(())
    }
    #[test]
    fn camera_relative_composition_precedes_f32_validation() -> Result<()> {
        let camera = Transform::<Hyperboloid3>::from_isometry(Hyperboloid3::shift_z(12.0))?;
        let object = Transform::<Hyperboloid3>::from_isometry(Hyperboloid3::shift_z(12.2))?;
        assert!(camera.rows().is_err());
        assert!(object.rows().is_err());
        let relative = camera.inverse()?.chain(&object)?;
        relative.rows()?;
        close(
            apply(relative, [1.0, 0.0, 0.0, 0.0]),
            [0.2_f64.cosh(), 0.0, 0.0, 0.2_f64.sinh()],
            2e-10,
        );
        Ok(())
    }
    #[test]
    fn cast_validation_has_a_bounded_absolute_metric_budget() {
        // Positive f64 norm alone is insufficient when aa and bb are large.
        let boosted = [[5000.0005, 0.0, 0.0, 0.0], [0.0, 5000.0, 0.0, 0.0]];
        assert!(validate_embedded_rows::<Hyperboloid3>(boosted).is_err());
        let non_unit = [[1.1, 0.0, 0.0, 0.0], [0.0; 4]];
        assert!(validate_embedded_rows::<Spherical3>(non_unit).is_err());
        let parallel = [[1.0, 0.0, 0.0, 0.0], [0.01, 0.0, 0.0, 0.0]];
        assert!(validate_embedded_rows::<Flat3>(parallel).is_err());
        fn check<G: Geometry>() {
            assert!(validate_embedded_rows::<G>([[0.0; 4]; 2]).is_err());
            assert!(validate_embedded_rows::<G>([[f32::NAN; 4]; 2]).is_err());
        }
        check::<Flat3>();
        check::<Hyperboloid3>();
        check::<Spherical3>();
    }
    #[test]
    fn construction_rejects_finite_pairs_with_overflowing_point_action() {
        let map = EmbeddedIsometry::<f64, 0>::from_pair(QuaternionPair::from_array([
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            f64::MAX,
            0.0,
            0.0,
        ]))
        .unwrap();
        assert!(map.pair().into_array().into_iter().all(f64::is_finite));
        assert!(Transform::<Flat3>::from_isometry(map).is_err());
    }
    #[test]
    fn overflowing_compositions_return_errors() -> Result<()> {
        let a = Transform::<Flat3>::identity().move_local([f64::MAX, 0.0, 0.0], [0.0; 3], 1.0)?;
        assert!(a.chain(&a).is_err());
        Ok(())
    }
}
