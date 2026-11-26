use std::f64::consts::FRAC_PI_2;

use anyhow::Result;
use ccgeom::{Geometry3, Hyperboloid3};
use wgame::{Library, glam::Vec3, shapes::Polygon};

use super::{Material, Object, Pose, mesh_with_normals};

/// A finite patch of the horosphere h=1 in its own upper-half-space chart.
pub(super) fn moon(library: &Library) -> Result<Object<Polygon>> {
    // The +z chart axis points away from Earth. These ground offsets and
    // normal gap match the direction and angular radius of the Moon at spawn.
    let pose = Pose::from_isometry(
        Hyperboloid3::shift_x(-0.971987)
            .chain(Hyperboloid3::shift_z(0.020546))
            .chain(Hyperboloid3::shift_y(1.325230))
            .chain(Hyperboloid3::rotate_x(-FRAC_PI_2)),
    )?;

    // In this chart Earth is a hemisphere of radius B=exp(-gap)<1 about
    // (0,0,0). An eye at normal altitude a<gap has horizontal radius at
    // most B*cosh(a)<1. From chart height h<1 its first lunar hits are
    // within sqrt(1-h²) of its horizontal position, even at the limb.
    // The square [-2,2]² therefore covers every first hit from the ground.
    const CELLS: u32 = 128;
    let row = CELLS + 1;
    let coord = |i| 4.0 * f64::from(i) / f64::from(CELLS) - 2.0;
    let mut vertices = Vec::with_capacity((row * row) as usize);
    let mut indices = Vec::with_capacity((6 * CELLS * CELLS) as usize);
    for y in 0..=CELLS {
        for x in 0..=CELLS {
            vertices.push(([coord(x), coord(y), 1.0], [1.0; 3]));
            if x < CELLS && y < CELLS {
                let i = x + y * row;
                indices.extend([i, i + 1, i + row, i + 1, i + row + 1, i + row]);
            }
        }
    }

    // For unit hyperboloid position P on h=1, P-(1,0,0,1) is the outward
    // unit tangent normal. The shader recovers its time component from k·n.
    let shape = mesh_with_normals(library, &vertices, &indices, |[_, x, y, z]| {
        Vec3::new(x as f32, y as f32, (z - 1.0) as f32)
    })?;
    Ok(Object {
        shape,
        pose,
        material: Material::Diffuse(Vec3::new(0.76, 0.76, 0.74)),
    })
}
