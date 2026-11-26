use std::f64::consts::TAU;

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

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Diffuse,
    Translucent { transmission: f32 },
    Emissive,
}

pub struct Object {
    pub mesh: PolygonFill,
    pub pose: Pose,
    pub material: Material,
    pub infinite_plane: bool,
}

pub struct Sun {
    pub pose: Pose,
    pub radiance: [f32; 3],
}

pub struct World {
    pub objects: Vec<Object>,
    pub sun: Sun,
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

/// A geodesic plane is the open disk x²+z²<1 in its local Klein chart.
/// Two triangles cover the disk. The plane material clips the ideal boundary.
pub fn plane(library: &Library) -> PolygonFill {
    let shapes = library.shapes();
    let vertices = [(-1.0, -1.0), (-1.0, 1.0), (1.0, 1.0), (1.0, -1.0)]
        .map(|(x, z)| Vertex::new(Vec4::new(x, 0.0, z, 1.0), Vec3::Z));
    shapes
        .mesh(Mesh::from_arrays(
            shapes.state(),
            &vertices,
            Some(&[0, 1, 2, 0, 2, 3]),
        ))
        .fill_color(color::WHITE)
}

#[derive(Default)]
struct Plant {
    vertices: Vec<([f64; 3], [f32; 3])>,
    indices: Vec<u32>,
}

impl Plant {
    // The root is (0,0,1) in local half-space. +y grows away from
    // the geodesic ground plane and z is an offset in chart height.
    fn vertex(&mut self, [x, y, z]: [f64; 3], rgb: [f32; 3]) -> u32 {
        let index = self.vertices.len() as u32;
        self.vertices.push(([x, y, 1.0 + z], rgb));
        index
    }

    // Stack polygonal rings, close the base, then taper to a single tip.
    fn tapered_rings(
        &mut self,
        height: f64,
        sides: u32,
        rings: &[(f64, f64)],
        tip: f64,
        color: [f32; 3],
    ) {
        let base = self.vertices.len() as u32;
        for (ring, &(level, radius)) in rings.iter().enumerate() {
            for side in 0..sides {
                let angle = TAU * f64::from(side) / f64::from(sides);
                let (sin, cos) = angle.sin_cos();
                let i = self.vertex(
                    [radius * height * cos, level * height, radius * height * sin],
                    color,
                );
                if ring > 0 {
                    let next = base + ring as u32 * sides + (side + 1) % sides;
                    self.indices
                        .extend([i - sides, i, next - sides, next - sides, i, next]);
                }
            }
        }
        let top = self.vertex([0.0, height * tip, 0.0], color);
        for side in 0..sides {
            self.indices
                .extend([top - sides + side, top, top - sides + (side + 1) % sides]);
        }
        for side in 1..sides - 1 {
            self.indices.extend([base, base + side, base + side + 1]);
        }
    }

    fn upload(&self, library: &Library) -> Result<PolygonFill> {
        mesh(library, &self.vertices, &self.indices)
    }
}

fn tree(library: &Library, height: f64, conifer: bool) -> Result<PolygonFill> {
    let mut plant = Plant::default();
    plant.tapered_rings(
        height,
        5,
        &[(0.0, 0.048), (0.50, 0.033)],
        0.56,
        [0.29, 0.14, 0.055],
    );
    if conifer {
        plant.tapered_rings(height, 7, &[(0.24, 0.29)], 0.73, [0.07, 0.32, 0.14]);
        plant.tapered_rings(height, 7, &[(0.44, 0.23)], 0.88, [0.095, 0.39, 0.15]);
        plant.tapered_rings(height, 7, &[(0.64, 0.16)], 1.0, [0.15, 0.47, 0.18]);
    } else {
        plant.tapered_rings(
            height,
            7,
            &[(0.34, 0.14), (0.47, 0.30), (0.71, 0.34), (0.89, 0.21)],
            1.0,
            [0.28, 0.51, 0.10],
        );
    }
    plant.upload(library)
}

fn grass(library: &Library) -> Result<PolygonFill> {
    let mut plant = Plant::default();
    for blade in 0..9 {
        let angle = TAU * f64::from(blade) / 9.0;
        let (sin, cos) = angle.sin_cos();
        let height = 0.016 + 0.003 * f64::from(blade * 5 % 7);
        // The blade bends outward near its tip, where it catches sunlight
        // even when the nearly vertical lower face is in shade.
        let mut strip = |radius: f64, width: f64, y, tint| {
            [-width, width].map(|offset| {
                plant.vertex(
                    [radius * cos - offset * sin, y, radius * sin + offset * cos],
                    tint,
                )
            })
        };
        let [a, b] = strip(0.007, 0.0045, 0.0, [0.18, 0.39, 0.06]);
        let [c, d] = strip(0.018, 0.0035, height * 0.80, [0.28, 0.53, 0.09]);
        let tip = plant.vertex([0.037 * cos, height, 0.037 * sin], [0.35, 0.61, 0.13]);
        plant.indices.extend([a, b, c, b, d, c, c, d, tip]);
    }
    plant.upload(library)
}

pub fn initial_camera() -> Result<Pose> {
    Pose::from_isometry(
        Hyperboloid3::shift_z(1.3)
            .chain(Hyperboloid3::shift_y(0.075))
            .chain(Hyperboloid3::rotate_x(0.14)),
    )
}

fn vegetation(library: &Library, objects: &mut Vec<Object>) -> Result<()> {
    let trees = [
        tree(library, 0.16, false)?,
        tree(library, 0.22, true)?,
        tree(library, 0.25, false)?,
        tree(library, 0.15, true)?,
    ];
    let grass = grass(library)?;
    let mut seed = 191813u32;
    let mut sample = || {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(seed) / (f64::from(u32::MAX) + 1.0)
    };

    // Hyperbolic disk area is proportional to cosh(r)-1. Plants occupy
    // the first two curvature radii around the permanent subsolar point.
    for i in 0..570 {
        let radius = (1.0 + sample() * (2.0_f64.cosh() - 1.0)).acosh();
        let azimuth = TAU * sample();
        let root = Hyperboloid3::rotate_y(azimuth)
            .chain(Hyperboloid3::shift_z(radius))
            .chain(Hyperboloid3::rotate_y(TAU * sample()));
        let pose = Pose::from_isometry(root)?;
        let [_, x, _, z] = pose.apply_vector([1.0, 0.0, 0.0, 0.0]);
        let (mesh, material) = if i < 200 {
            // Leave a winding meadow among the groves, including the spawn point.
            if (x - 0.18 * (2.0 * z).sin()).abs() < 0.28
                || (4.0 * x + z).sin() + (2.5 * z - x).cos() < -0.40
            {
                continue;
            }
            let index = (sample() * trees.len() as f64) as usize;
            (trees[index].clone(), Material::Diffuse)
        } else {
            (grass.clone(), Material::Translucent { transmission: 0.22 })
        };
        objects.push(Object {
            mesh,
            pose,
            material,
            infinite_plane: false,
        });
    }
    Ok(())
}

pub fn world(library: &Library) -> Result<World> {
    let sheet = plane(library);
    let sun = Sun {
        pose: Pose::from_isometry(Hyperboloid3::shift_y(3.0))?,
        radiance: [100.0, 94.0, 72.0],
    };
    let mut objects = vec![
        Object {
            mesh: sheet.multiply_color(Vec3::new(0.23, 0.48, 0.10)),
            pose: Pose::identity(),
            material: Material::Diffuse,
            infinite_plane: true,
        },
        Object {
            mesh: sheet.clone(),
            pose: sun.pose,
            material: Material::Emissive,
            infinite_plane: true,
        },
        Object {
            mesh: sheet.multiply_color(Vec3::new(0.76, 0.76, 0.74)),
            pose: Pose::from_isometry(
                Hyperboloid3::shift_x(-1.0).chain(Hyperboloid3::shift_y(2.0)),
            )?,
            material: Material::Diffuse,
            infinite_plane: true,
        },
    ];
    vegetation(library, &mut objects)?;
    Ok(World { objects, sun })
}
