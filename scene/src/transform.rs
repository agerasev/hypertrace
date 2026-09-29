use anyhow::{Context as _, ensure};
use ccgeom::embedded::{EmbeddedIsometry, Space3};
use vecmat::QuaternionPair;

use crate::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Geometry {
    Euclidean,
    Hyperbolic,
    Spherical,
}

impl Geometry {
    pub fn sign(self) -> i8 {
        match self {
            Self::Euclidean => 0,
            Self::Hyperbolic => -1,
            Self::Spherical => 1,
        }
    }
    pub fn tag(self) -> u32 {
        match self {
            Self::Euclidean => 0,
            Self::Hyperbolic => 1,
            Self::Spherical => 2,
        }
    }
}

/// A canonical f64 quaternion-pair isometry with runtime curvature selection.
/// All construction, camera motion, and GPU preparation use this representation.
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    isometry: Isometry,
}

// The variants only dispatch const-generic algebra; they all store the same
// quaternion-pair representation. Coordinate charts are never transform states.
#[derive(Clone, Copy, Debug)]
enum Isometry {
    Euclidean(EmbeddedIsometry<f64, 0>),
    Hyperbolic(EmbeddedIsometry<f64, -1>),
    Spherical(EmbeddedIsometry<f64, 1>),
}

fn compose<const K: i8>(
    a: EmbeddedIsometry<f64, K>,
    b: EmbeddedIsometry<f64, K>,
) -> Result<EmbeddedIsometry<f64, K>> {
    let map = EmbeddedIsometry::from_pair(a.pair() * b.pair())
        .context("isometry composition exceeds numerical range")?;
    finite_action(map)
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

impl Transform {
    pub fn geometry(self) -> Geometry {
        match self.isometry {
            Isometry::Euclidean(_) => Geometry::Euclidean,
            Isometry::Hyperbolic(_) => Geometry::Hyperbolic,
            Isometry::Spherical(_) => Geometry::Spherical,
        }
    }
    pub fn identity(geometry: Geometry) -> Self {
        Self {
            isometry: match geometry {
                Geometry::Euclidean => Isometry::Euclidean(EmbeddedIsometry::identity()),
                Geometry::Hyperbolic => Isometry::Hyperbolic(EmbeddedIsometry::identity()),
                Geometry::Spherical => Isometry::Spherical(EmbeddedIsometry::identity()),
            },
        }
    }
    /// Erase the compile-time curvature while retaining the canonical f64 map.
    pub fn from_isometry<const K: i8>(map: EmbeddedIsometry<f64, K>) -> Result<Self> {
        let values = map.pair().into_array();
        Ok(Self {
            isometry: match K {
                0 => Isometry::Euclidean(finite_action(
                    EmbeddedIsometry::from_pair(QuaternionPair::from_array(values))
                        .context("invalid Euclidean isometry")?,
                )?),
                -1 => Isometry::Hyperbolic(finite_action(
                    EmbeddedIsometry::from_pair(QuaternionPair::from_array(values))
                        .context("invalid hyperbolic isometry")?,
                )?),
                1 => Isometry::Spherical(finite_action(
                    EmbeddedIsometry::from_pair(QuaternionPair::from_array(values))
                        .context("invalid spherical isometry")?,
                )?),
                _ => anyhow::bail!("unsupported curvature sign {K}"),
            },
        })
    }
    pub fn chain(&self, inner: &Self) -> Result<Self> {
        Ok(Self {
            isometry: match (self.isometry, inner.isometry) {
                (Isometry::Euclidean(a), Isometry::Euclidean(b)) => {
                    Isometry::Euclidean(compose(a, b)?)
                }
                (Isometry::Hyperbolic(a), Isometry::Hyperbolic(b)) => {
                    Isometry::Hyperbolic(compose(a, b)?)
                }
                (Isometry::Spherical(a), Isometry::Spherical(b)) => {
                    Isometry::Spherical(compose(a, b)?)
                }
                _ => anyhow::bail!("cannot compose transforms from different geometries"),
            },
        })
    }
    pub fn inverse(self) -> Result<Self> {
        Ok(Self {
            isometry: match self.isometry {
                Isometry::Euclidean(map) => Isometry::Euclidean(finite_action(map.inv())?),
                Isometry::Hyperbolic(map) => Isometry::Hyperbolic(finite_action(map.inv())?),
                Isometry::Spherical(map) => Isometry::Spherical(finite_action(map.inv())?),
            },
        })
    }
    /// Apply the isometry to an ambient point or tangent in scalar-first order.
    pub fn apply_vector(self, vector: [f64; 4]) -> [f64; 4] {
        match self.isometry {
            Isometry::Euclidean(map) => map.apply_vector(vector.into()).into(),
            Isometry::Hyperbolic(map) => map.apply_vector(vector.into()).into(),
            Isometry::Spherical(map) => map.apply_vector(vector.into()).into(),
        }
    }
    /// Keep f64 canonical transforms until camera-relative preparation.
    pub fn components(self) -> Result<[[f64; 4]; 2]> {
        let values = match self.isometry {
            Isometry::Euclidean(map) => map.pair().into_array(),
            Isometry::Hyperbolic(map) => map.pair().into_array(),
            Isometry::Spherical(map) => map.pair().into_array(),
        };
        ensure!(values.iter().all(|x| x.is_finite()), "non-finite isometry");
        Ok([
            values[..4].try_into().unwrap(),
            values[4..].try_into().unwrap(),
        ])
    }
    pub fn rows(&self) -> Result<[[f32; 4]; 2]> {
        let rows = self.components()?.map(|r| r.map(|v| v as f32));
        validate_embedded_rows(rows, self.geometry())?;
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
    pub fn from_rows(rows: [[f32; 4]; 2], geometry: Geometry) -> Result<Self> {
        validate_embedded_rows(rows, geometry)?;
        let v: [f64; 8] = std::array::from_fn(|i| f64::from(rows[i / 4][i % 4]));
        Ok(Self {
            isometry: match geometry {
                Geometry::Euclidean => Isometry::Euclidean(
                    EmbeddedIsometry::from_pair(QuaternionPair::from_array(v))
                        .context("invalid Euclidean isometry")?,
                ),
                Geometry::Hyperbolic => Isometry::Hyperbolic(
                    EmbeddedIsometry::from_pair(QuaternionPair::from_array(v))
                        .context("invalid hyperbolic isometry")?,
                ),
                Geometry::Spherical => Isometry::Spherical(
                    EmbeddedIsometry::from_pair(QuaternionPair::from_array(v))
                        .context("invalid spherical isometry")?,
                ),
            },
        })
    }
    pub fn move_local(
        self,
        translation: [f64; 3],
        rotation: [f64; 3],
        radius: f64,
    ) -> Result<Self> {
        fn moved<const K: i8>(
            mut map: EmbeddedIsometry<f64, K>,
            t: [f64; 3],
            r: [f64; 3],
            radius: f64,
        ) -> Result<EmbeddedIsometry<f64, K>> {
            let space = Space3::<f64, K>::new(radius).context("invalid curvature radius")?;
            for (axis, distance) in t.into_iter().enumerate() {
                let mut direction = [0.0; 3];
                direction[axis] = 1.0;
                map = compose(
                    map,
                    space
                        .translation(direction.into(), distance)
                        .context("invalid camera translation")?,
                )?;
            }
            for (axis, angle) in r.into_iter().enumerate() {
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
        Ok(Self {
            isometry: match self.isometry {
                Isometry::Euclidean(m) => {
                    Isometry::Euclidean(moved(m, translation, rotation, radius)?)
                }
                Isometry::Hyperbolic(m) => {
                    Isometry::Hyperbolic(moved(m, translation, rotation, radius)?)
                }
                Isometry::Spherical(m) => {
                    Isometry::Spherical(moved(m, translation, rotation, radius)?)
                }
            },
        })
    }
}

/// Check the stored unit-pair constraints after f32 conversion. The absolute
/// error budget does not grow with the boost: otherwise cancellation would
/// permit transforms with norm far from one. Camera-relative preparation must
/// happen before this check, while canonical transforms remain in f64.
pub fn validate_embedded_rows(rows: [[f32; 4]; 2], geometry: Geometry) -> Result<()> {
    const MAX_METRIC_ERROR: f64 = 1e-3;
    ensure!(
        rows.iter().flatten().all(|x| x.is_finite()),
        "transform is outside f32 range"
    );
    let [a, b] = rows.map(|r| r.map(f64::from));
    let aa = a.iter().map(|x| x * x).sum::<f64>();
    let bb = b.iter().map(|x| x * x).sum::<f64>();
    let ab = a.iter().zip(b).map(|(a, b)| a * b).sum::<f64>();
    let sign = f64::from(geometry.sign());
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
    use ccgeom::{Geometry3, Hyperboloid3};

    fn apply(transform: Transform, p: [f64; 4]) -> [f64; 4] {
        transform.apply_vector(p)
    }
    fn close(a: [f64; 4], b: [f64; 4], tolerance: f64) {
        for i in 0..4 {
            assert!((a[i] - b[i]).abs() <= tolerance, "{a:?} != {b:?}");
        }
    }
    #[test]
    fn composition_inverse_and_upload_roundtrip_cover_all_signs() -> Result<()> {
        for geometry in [
            Geometry::Euclidean,
            Geometry::Hyperbolic,
            Geometry::Spherical,
        ] {
            let a = Transform::identity(geometry).move_local(
                [0.3, -0.2, 0.4],
                [0.1, 0.2, -0.3],
                1.0,
            )?;
            let b =
                Transform::identity(geometry).move_local([-0.1, 0.3, 0.2], [0.3, 0.2, 0.1], 1.0)?;
            let p = [1.0, 0.0, 0.0, 0.0];
            close(apply(a.chain(&b)?, p), apply(a, apply(b, p)), 3e-14);
            close(apply(a.inverse()?, apply(a, p)), p, 3e-14);
            close(
                apply(Transform::from_rows(a.rows()?, geometry)?, p),
                apply(a, p),
                1e-7,
            );
        }
        assert!(
            Transform::identity(Geometry::Spherical)
                .chain(&Transform::identity(Geometry::Hyperbolic))
                .is_err()
        );
        Ok(())
    }
    #[test]
    fn local_movement_uses_physical_radius_and_preserves_full_circuits() -> Result<()> {
        let radius = 2.3;
        let distance = 0.7;
        let sphere = Transform::identity(Geometry::Spherical).move_local(
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
        let full = Transform::identity(Geometry::Spherical).move_local(
            [std::f64::consts::TAU * radius, 0.0, 0.0],
            [0.0; 3],
            radius,
        )?;
        close(
            apply(full, [1.0, 0.0, 0.0, 0.0]),
            [1.0, 0.0, 0.0, 0.0],
            2e-15,
        );
        let hyper = Transform::identity(Geometry::Hyperbolic).move_local(
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
            Transform::identity(Geometry::Euclidean)
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
        let camera = Transform::from_isometry(Hyperboloid3::shift_z(12.0))?;
        let object = Transform::from_isometry(Hyperboloid3::shift_z(12.2))?;
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
        assert!(validate_embedded_rows(boosted, Geometry::Hyperbolic).is_err());
        let non_unit = [[1.1, 0.0, 0.0, 0.0], [0.0; 4]];
        assert!(validate_embedded_rows(non_unit, Geometry::Spherical).is_err());
        let parallel = [[1.0, 0.0, 0.0, 0.0], [0.01, 0.0, 0.0, 0.0]];
        assert!(validate_embedded_rows(parallel, Geometry::Euclidean).is_err());
        for geometry in [
            Geometry::Euclidean,
            Geometry::Hyperbolic,
            Geometry::Spherical,
        ] {
            assert!(validate_embedded_rows([[0.0; 4]; 2], geometry).is_err());
            assert!(validate_embedded_rows([[f32::NAN; 4]; 2], geometry).is_err());
        }
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
        assert!(Transform::from_isometry(map).is_err());
    }
    #[test]
    fn overflowing_compositions_return_errors() -> Result<()> {
        let a = Transform::identity(Geometry::Euclidean).move_local(
            [f64::MAX, 0.0, 0.0],
            [0.0; 3],
            1.0,
        )?;
        assert!(a.chain(&a).is_err());
        Ok(())
    }
}
