use std::f64::consts::FRAC_PI_2;

use anyhow::{Result, ensure};
use ccgeom::{Geometry3, Hyperboloid3};
use wgame::{
    Library,
    glam::Vec3,
    shader::Attribute,
    shapes::{Polygon, shader::Vertex},
};

use super::{Object, Pose, mesh_with_normals};
use crate::config::Moon;

/// A finite patch of the horosphere h=1 in its own upper-half-space chart.
pub(super) fn moon(library: &Library, moon: &Moon) -> Result<Object<Polygon>> {
    // The +z chart axis points away from Earth.
    let pose = Pose::from_isometry(
        Hyperboloid3::shift_x(moon.ground_offset[0])
            .chain(Hyperboloid3::shift_z(moon.ground_offset[1]))
            .chain(Hyperboloid3::shift_y(moon.distance))
            .chain(Hyperboloid3::rotate_x(-FRAC_PI_2)),
    )?;

    // In this chart Earth is a hemisphere of radius B=exp(-gap)<1 about
    // (0,0,0). An eye at normal altitude a<gap has horizontal radius at
    // most B*cosh(a)<1. From chart height h<1 its first lunar hits are
    // within sqrt(1-h²) of its horizontal position, even at the limb.
    // The square [-2,2]² therefore covers every first hit from the ground.
    let cells = moon.subdivisions;
    let row = cells + 1;
    let coord = |i| 4.0 * f64::from(i) / f64::from(cells) - 2.0;
    let vertex_count = row * row;
    let index_count = 6 * cells * cells;
    let limit = library.state().device().limits().max_buffer_size;
    ensure!(
        u64::from(vertex_count) * Vertex::SIZE as u64 <= limit
            && u64::from(index_count) * size_of::<u32>() as u64 <= limit,
        "moon.subdivisions exceeds the device buffer limit"
    );
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    vertices.try_reserve_exact(vertex_count as usize)?;
    indices.try_reserve_exact(index_count as usize)?;
    for y in 0..=cells {
        for x in 0..=cells {
            vertices.push(([coord(x), coord(y), 1.0], [1.0; 3]));
            if x < cells && y < cells {
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
        material: moon.material.build()?,
    })
}
