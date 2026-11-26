use std::f64::consts::{FRAC_PI_2, TAU};

use anyhow::{Context, Result, ensure};
use ccgeom::{Geometry3, Hyperboloid3, Space3};
use wgame::{
    Library,
    gfx::types::color,
    glam::{Vec3, Vec4},
    prelude::*,
    shapes::{Mesh, PolygonFill, shader::Vertex},
};

use crate::projection::Pose;

pub struct Object {
    pub mesh: PolygonFill,
    pub pose: Pose,
}

/// Upload a static indexed mesh. Each vertex is (local upper-half-space point, RGB).
pub fn mesh(
    library: &Library,
    vertices: &[([f64; 3], [f32; 3])],
    indices: &[u32],
) -> Result<PolygonFill> {
    ensure!(
        !indices.is_empty() && indices.len().is_multiple_of(3),
        "expected triangles"
    );
    ensure!(
        indices.iter().all(|&i| (i as usize) < vertices.len()),
        "invalid vertex index"
    );
    let space = Space3::<f64, -1>::unit();
    let vertices = vertices
        .iter()
        .map(|&(point, rgb)| {
            let p = space
                .point_from_half_space(point.into())
                .context("invalid half-space vertex")?;
            ensure!(
                p.into_iter().all(f64::is_finite) && p[0] > 0.0,
                "local vertex exceeds chart range"
            );
            // Homogeneous Klein coordinates bound all local GPU inputs by one.
            let xyzw = Vec4::new(
                (p[1] / p[0]) as f32,
                (p[2] / p[0]) as f32,
                (p[3] / p[0]) as f32,
                1.0,
            );
            ensure!(rgb.into_iter().all(f32::is_finite), "invalid vertex color");
            Ok(Vertex::new(xyzw, Vec3::Z).with_color(Vec3::from_array(rgb).extend(1.0)))
        })
        .collect::<Result<Vec<_>>>()?;
    let shapes = library.shapes();
    Ok(shapes
        .mesh(Mesh::from_arrays(shapes.state(), &vertices, Some(indices)))
        .fill_color(color::WHITE))
}

// The unit hemisphere is a geodesic plane through the local origin (0,0,1).
// Dilating all three half-space coordinates moves the plane along the viewing axis.
fn cross_section(x: f64, y: f64, displacement: f64) -> [f64; 3] {
    [x, y, (1.0 - x * x - y * y).sqrt()].map(|v| v * displacement.exp())
}

fn gate(library: &Library) -> Result<PolygonFill> {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    const SIDES: u32 = 8;
    for side in 0..SIDES {
        let angle = TAU * f64::from(side) / f64::from(SIDES) + TAU / 16.0;
        let (sin, cos) = angle.sin_cos();
        for (radius, depth, shade) in [
            (0.62, 0.04, 1.0),
            (0.54, 0.04, 0.82),
            (0.54, -0.04, 0.34),
            (0.62, -0.04, 0.58),
        ] {
            vertices.push((cross_section(radius * cos, radius * sin, depth), [shade; 3]));
        }
        for edge in 0..4 {
            let a = 4 * side;
            let b = 4 * ((side + 1) % SIDES);
            let next = (edge + 1) % 4;
            indices.extend([a + edge, b + edge, b + next, a + edge, b + next, a + next]);
        }
    }
    mesh(library, &vertices, &indices)
}

const SPACING: f64 = 0.82;

fn walkway(library: &Library) -> Result<PolygonFill> {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    // Three panels and two edge strips provide parallax when walking through the gates.
    let edges = [-0.40, -0.37, -0.13, 0.13, 0.37, 0.40];
    let shades = [0.68, 0.25, 0.32, 0.25, 0.68];
    for (stripe, ends) in edges.windows(2).enumerate() {
        let base = vertices.len() as u32;
        for depth in [SPACING * 0.49, -SPACING * 0.49] {
            for &x in ends {
                vertices.push((cross_section(x, -0.47, depth), [shades[stripe]; 3]));
            }
        }
        indices.extend([base, base + 1, base + 2, base + 2, base + 1, base + 3]);
    }
    mesh(library, &vertices, &indices)
}

pub fn objects(library: &Library) -> Result<Vec<Object>> {
    let gate = gate(library)?;
    let walkway = walkway(library)?.multiply_color(Vec3::new(0.43, 0.62, 0.72));
    let colors = [
        Vec3::new(0.20, 0.80, 0.80),
        Vec3::new(1.0, 0.64, 0.24),
        Vec3::new(0.60, 0.48, 0.90),
    ];
    let mut objects = Vec::new();
    for station in -2_i32..8 {
        let distance = 1.10 + f64::from(station) * SPACING;
        let center = Hyperboloid3::shift_z(-distance);
        let turn = f64::from(station.rem_euclid(3)) * FRAC_PI_2 / 8.0;
        objects.push(Object {
            pose: Pose::from_isometry(center.chain(Hyperboloid3::rotate_z(turn)))?,
            mesh: gate.multiply_color(colors[station.rem_euclid(3) as usize]),
        });
        objects.push(Object {
            pose: Pose::from_isometry(Hyperboloid3::shift_z(-distance - SPACING / 2.0))?,
            mesh: walkway.clone(),
        });
    }
    Ok(objects)
}
