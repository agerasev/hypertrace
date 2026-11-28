//! Tile addresses in the reflection group of a regular right-angled pentagon.
use std::{
    f64::consts::{FRAC_PI_4, PI, TAU},
    sync::LazyLock,
};

use anyhow::{Result, ensure};
use ccgeom::{Geometry3, Hyperboloid3};

use crate::projection::Pose;

pub(super) fn inradius() -> f64 {
    (FRAC_PI_4.cos() / (PI / 5.0).sin()).acosh()
}

pub(super) fn circumradius() -> f64 {
    (1.0 / (PI / 5.0).tan()).acosh()
}

pub(super) fn side_direction(side: u8) -> [f64; 2] {
    // Tile addresses number sides clockwise from +X when viewed from above.
    let angle = -TAU * f64::from(side) / 5.0;
    [angle.cos(), angle.sin()]
}

struct Side {
    direction: [f64; 2],
    reflection: Pose,
}

static SIDES: LazyLock<[Side; 5]> = LazyLock::new(|| {
    // For side zero, this half-turn about Y has exactly zero scalar part.
    let half_turn = Pose::from_rows([[0.0, 0.0, 1.0, 0.0], [0.0; 4]]).unwrap();
    let across = Pose::from_isometry(Hyperboloid3::shift_x(2.0 * inradius())).unwrap();
    let reflection = across.chain(&half_turn).unwrap();
    std::array::from_fn(|index| {
        // A half-turn about the side geodesic reflects the ground pentagon
        // and reverses +Z. Its center moves twice the inradius outward.
        let yaw = Pose::from_isometry(Hyperboloid3::rotate_z(-TAU * index as f64 / 5.0)).unwrap();
        Side {
            direction: side_direction(index as u8),
            reflection: yaw
                .chain(&reflection)
                .unwrap()
                .chain(&yaw.inverse().unwrap())
                .unwrap(),
        }
    })
});

/// Shortest, lexicographically first side-crossing word from the origin tile.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct TileId(Vec<u8>);

impl TileId {
    fn append(&mut self, side: u8) {
        // Each generator squares to one. Two distinct side reflections commute
        // exactly when their sides meet at right angles (neighbors in the 5-cycle).
        // Pass the commutable suffix to find a cancellation; otherwise insert
        // before its first larger letter to keep the lexicographic normal form.
        let mut insert = self.0.len();
        for (index, &other) in self.0.iter().enumerate().rev() {
            if other == side {
                self.0.remove(index);
                return;
            }
            if !matches!(side.abs_diff(other), 1 | 4) {
                break;
            }
            if other > side {
                insert = index;
            }
        }
        self.0.insert(insert, side);
    }

    pub(crate) fn neighbor(&self, side: u8) -> Self {
        let mut next = self.clone();
        next.append(side);
        next
    }

    pub(crate) fn compose(&self, relative: &Self) -> Self {
        let mut next = self.clone();
        for &side in &relative.0 {
            next.append(side);
        }
        next
    }

    pub(crate) fn flips_normal(&self) -> bool {
        !self.0.len().is_multiple_of(2)
    }

    pub(crate) fn seed(&self, seed: u64) -> u64 {
        // FNV-1a over the canonical word, salted by the world's seed.
        self.0
            .iter()
            .fold(seed ^ 0xcbf29ce484222325, |hash, &side| {
                (hash ^ u64::from(side)).wrapping_mul(0x100000001b3)
            })
    }

    pub(crate) fn frame(&self) -> Result<Pose> {
        self.0.iter().try_fold(Pose::identity(), |frame, &side| {
            frame.chain(&SIDES[usize::from(side)].reflection)
        })
    }

    /// Local +Z points up on Earth, including for odd reflection words.
    pub(in crate::world) fn upright_frame(&self) -> Result<Pose> {
        let frame = self.frame()?;
        if self.flips_normal() {
            // This half-turn reflects Y in the ground and preserves the local pentagon.
            let half_turn = Pose::from_rows([[0.0, 1.0, 0.0, 0.0], [0.0; 4]])?;
            frame.chain(&half_turn)
        } else {
            Ok(frame)
        }
    }

    pub(crate) fn at(eye: Pose) -> Result<Self> {
        let mut tile = Self::default();
        let mut relative = eye;
        let support = inradius().tanh();
        let world_w = eye.apply_vector([1.0, 0.0, 0.0, 0.0])[0];
        loop {
            let [w, x, y, _] = relative.apply_vector([1.0, 0.0, 0.0, 0.0]);
            ensure!(w.is_finite() && w > 0.0, "eye exceeds ground tiling range");
            // Orthogonal projection onto Earth rescales (w,x,y) equally. Thus
            // these Klein coordinates depend only on the point under the eye.
            let (kx, ky) = (x / w, y / w);
            let mut outside = None;
            // Composing an eye-relative frame can cancel terms the size of its
            // world boost. Keep points on a shared side with the first chamber
            // reached from the origin when this sign cannot be resolved.
            let mut largest = 32.0 * f64::EPSILON * (world_w / w).max(1.0);
            ensure!(
                largest < support,
                "ground tile boundary exceeds pose precision"
            );
            for (index, side) in SIDES.iter().enumerate() {
                let [nx, ny] = side.direction;
                let excess = nx * kx + ny * ky - support;
                if excess > largest {
                    outside = Some(index);
                    largest = excess;
                }
            }
            let Some(index) = outside else {
                return Ok(tile);
            };
            // Reflect the relative *pose*, keeping local precision even when the
            // eye and its tile are far from the origin in world coordinates.
            let next = SIDES[index].reflection.chain(&relative)?;
            ensure!(
                next.apply_vector([1.0, 0.0, 0.0, 0.0])[0] < w,
                "ground tile lookup exceeds pose precision"
            );
            relative = next;
            tile.append(index as u8);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use ccgeom::Space3;

    #[test]
    fn different_routes_around_a_corner_name_the_same_tile() {
        let origin = TileId::default();
        let mut touching = BTreeSet::from([origin.clone()]);
        for side in 0..5 {
            let adjacent = (side + 1) % 5;
            let edge = origin.neighbor(side);
            let corner = edge.neighbor(adjacent);
            assert_eq!(edge.neighbor(side), origin);
            assert_eq!(corner, origin.neighbor(adjacent).neighbor(side));
            assert_eq!(corner.neighbor(side).neighbor(adjacent), origin);
            touching.extend([edge, corner]);
        }
        assert_eq!(touching.len(), 11);
    }

    #[test]
    fn reflected_frames_agree_with_addresses_and_preserve_the_ground() -> Result<()> {
        let mut frontier = BTreeSet::from([TileId::default()]);
        for _ in 0..4 {
            let mut next = BTreeSet::new();
            for tile in frontier {
                let frame = tile.frame()?;
                let normal = frame.apply_vector([0.0, 0.0, 0.0, 1.0]);
                let sign = if tile.flips_normal() { -1.0 } else { 1.0 };
                assert!((normal[3] - sign).abs() < 1e-12);
                assert!(normal[0].abs() + normal[1].abs() + normal[2].abs() < 1e-12);
                for (side, geometry) in SIDES.iter().enumerate() {
                    let neighbor = tile.neighbor(side as u8);
                    let step = frame.chain(&geometry.reflection)?;
                    assert_eq!(TileId::at(step)?, neighbor);
                    next.insert(neighbor);
                }
            }
            frontier = next;
        }
        Ok(())
    }

    #[test]
    fn eye_lookup_resolves_nearby_tiles_after_distant_ground_travel() -> Result<()> {
        for direction in [[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [-0.6, -0.8, 0.0]] {
            let eye = Pose::from_isometry(
                Space3::<f64, -1>::unit()
                    .translation(direction.into(), 20.0)
                    .unwrap(),
            )?;
            let tile = TileId::at(eye)?;
            assert_eq!(TileId::at(tile.frame()?)?, tile);
            let local = tile.frame()?.inverse()?.chain(&eye)?;
            let [w, x, y, _] = local.apply_vector([1.0, 0.0, 0.0, 0.0]);
            for side in SIDES.iter() {
                let [nx, ny] = side.direction;
                assert!((x * nx + y * ny) / w < inradius().tanh() + 1e-6);
            }
        }
        Ok(())
    }

    #[test]
    fn a_distant_shared_side_keeps_its_owner_when_the_eye_turns_or_jumps() -> Result<()> {
        let far = Pose::from_isometry(Hyperboloid3::shift_x(20.0))?;
        let tile = TileId::at(far)?;
        let frame = tile.frame()?;
        for side in 0..5 {
            let [x, y] = side_direction(side);
            let site = |offset: f64| {
                let boost = Space3::<f64, -1>::unit()
                    .translation([x, y, 0.0].into(), inradius() + offset)
                    .unwrap();
                frame.chain(&Pose::from_isometry(boost)?)
            };
            let edge = site(0.0)?;
            let owner = TileId::at(edge)?;
            let raised = edge.move_local([0.0, 0.0, 0.05], [0.5, 0.0, 1.3], 1.0)?;
            assert_eq!(TileId::at(raised)?, owner);
            let neighbor = tile.neighbor(side);
            assert!(owner == tile || owner == neighbor);
            assert_eq!(TileId::at(site(-2e-5)?)?, tile);
            assert_eq!(TileId::at(site(2e-5)?)?, neighbor);
        }
        Ok(())
    }
}
