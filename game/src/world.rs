use std::{collections::BTreeMap, f64::consts::TAU, ops::Range};

use anyhow::{Context, Result, ensure};
use ccgeom::{Geometry3, Hyperboloid3, Space3};
use wgame::{
    Library,
    glam::{Vec3, Vec4},
    shapes::{Mesh, Polygon, shader::Vertex},
};

use crate::{atmosphere::Atmosphere, projection::Pose};

mod ecology;
mod ground_cells;
mod moon;
mod plants;
mod stars;

use ecology::{Daylight, Species};
use ground_cells::{Cell, Location};
use plants::Plants;
use stars::Stars;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Diffuse(Vec3),
    Translucent { color: Vec3, transmission: f32 },
    Emissive(Vec3),
}

impl Material {
    pub fn color(&self) -> Vec3 {
        match *self {
            Self::Diffuse(color) | Self::Translucent { color, .. } | Self::Emissive(color) => color,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Object<S> {
    pub shape: S,
    pub pose: Pose,
    pub material: Material,
}

/// The complete geodesic plane through the local origin with normal +Y.
#[derive(Clone, Copy)]
pub struct InfinitePlane;

#[derive(Default)]
pub struct Objects {
    pub meshes: Vec<Object<Polygon>>,
    pub planes: Vec<Object<InfinitePlane>>,
}

pub struct Sun {
    pub pose: Pose,
    pub radiance: [f32; 3],
}

/// Diffuse fill on a white receiver, in linear RGB.
#[derive(Clone, Copy, Default)]
pub struct AmbientLight {
    pub night: [f32; 3],
    pub day: [f32; 3],
}

pub struct World {
    pub(crate) atmosphere: Atmosphere,
    objects: Objects,
    fixed_meshes: Vec<Object<Polygon>>,
    center: Option<Cell>,
    fixed_plane_count: usize,
    stars: Stars,
    cell_meshes: BTreeMap<Cell, Range<usize>>,
    plants: Plants,
    daylight: Daylight,
    pub sun: Sun,
    pub ambient: AmbientLight,
}

/// Upload a static indexed mesh. Each vertex is (local upper-half-space point, RGB).
pub fn mesh(
    library: &Library,
    vertices: &[([f64; 3], [f32; 3])],
    indices: &[u32],
) -> Result<Polygon> {
    mesh_with_normals(library, vertices, indices, |_| Vec3::ZERO)
}

/// A nonzero vertex normal is the spatial part of its embedded tangent normal
/// in the local frame. Zero selects the geometric face normal for flat shading.
pub(super) fn mesh_with_normals(
    library: &Library,
    vertices: &[([f64; 3], [f32; 3])],
    indices: &[u32],
    normal_at: impl Fn([f64; 4]) -> Vec3,
) -> Result<Polygon> {
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
            let normal = normal_at(p.into());
            ensure!(normal.is_finite(), "invalid vertex normal");
            Ok(Vertex::new(xyzw, Vec3::Z)
                .with_color(Vec3::from_array(rgb).extend(1.0))
                .with_normal(normal))
        })
        .collect::<Result<Vec<_>>>()?;
    let shapes = library.shapes();
    Ok(shapes.mesh(Mesh::from_arrays(shapes.state(), &vertices, Some(indices))))
}

pub fn spawn_frame() -> Result<Pose> {
    Pose::from_isometry(Hyperboloid3::shift_z(1.3))
}

// SplitMix64. The cell address fixes each sample stream independently of visit order.
struct Samples(u64);

impl Samples {
    fn for_cell(cell: Cell) -> Self {
        Self(
            (cell.band as u64).wrapping_mul(0x8cb92ba72f3d8dd7)
                ^ (cell.column as u64).wrapping_mul(0xd6e8feb86659fd93),
        )
    }

    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        ((z ^ (z >> 31)) >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
}

impl World {
    /// Rebuild the local patch only when the observer enters a new ground cell.
    pub fn objects_near(&mut self, eye: Pose) -> Result<&Objects> {
        let location = Location::at(eye)?;
        let center = location.cell;
        if self.center != Some(center) {
            let mut meshes = Vec::with_capacity(self.objects.meshes.len());
            meshes.extend_from_slice(&self.fixed_meshes);
            let mut cell_meshes = BTreeMap::new();
            for cell in center.neighborhood()? {
                let start = meshes.len();
                if let Some(range) = self.cell_meshes.get(&cell) {
                    meshes.extend_from_slice(&self.objects.meshes[range.clone()]);
                } else {
                    self.plant_cell(cell, &mut meshes)?;
                }
                cell_meshes.insert(cell, start..meshes.len());
            }
            self.objects.meshes = meshes;
            self.cell_meshes = cell_meshes;
            self.center = Some(center);
        }
        self.objects.planes.truncate(self.fixed_plane_count);
        self.stars
            .append_visible(location.neighbors()?, &mut self.objects.planes)?;
        Ok(&self.objects)
    }

    fn plant_cell(&self, cell: Cell, meshes: &mut Vec<Object<Polygon>>) -> Result<()> {
        let mut samples = Samples::for_cell(cell);
        // Equal numbers of independent sites per species per unit of intrinsic area.
        for _ in 0..8 {
            for species in [Species::Deciduous, Species::Pine, Species::Grass] {
                let root = cell.root(samples.next(), samples.next())?;
                let occupied = samples.next();
                let maturity = 0.65 + 0.35 * samples.next();
                let yaw = TAU * samples.next();
                let light = self.daylight.at(root.apply_vector([1.0, 0.0, 0.0, 0.0]));
                if occupied >= species.density(light) {
                    continue;
                }
                let (shape, material) =
                    self.plants.grown(species, maturity * species.growth(light));
                meshes.push(Object {
                    shape,
                    pose: root.chain(&Pose::from_isometry(Hyperboloid3::rotate_y(yaw))?)?,
                    material,
                });
            }
        }
        Ok(())
    }
}

pub fn world(library: &Library) -> Result<World> {
    let sun = Sun {
        pose: Pose::from_isometry(Hyperboloid3::shift_y(3.0))?,
        radiance: [100.0, 96.0, 82.0],
    };
    let earth = Pose::identity();
    let planes = vec![
        Object {
            shape: InfinitePlane,
            pose: earth,
            material: Material::Diffuse(Vec3::new(0.23, 0.48, 0.10)),
        },
        Object {
            shape: InfinitePlane,
            pose: sun.pose,
            material: Material::Emissive(Vec3::from_array(sun.radiance)),
        },
    ];
    Ok(World {
        fixed_plane_count: planes.len(),
        stars: Stars::default(),
        atmosphere: Atmosphere {
            ground: earth,
            height: 0.45,
            scattering: [0.70, 1.41, 3.08],
        },
        objects: Objects {
            planes,
            ..Objects::default()
        },
        fixed_meshes: vec![moon::moon(library)?],
        center: None,
        cell_meshes: BTreeMap::new(),
        plants: Plants::new(library)?,
        daylight: Daylight::new(earth, &sun)?,
        sun,
        ambient: AmbientLight {
            night: [0.004, 0.005, 0.007],
            day: [0.13, 0.14, 0.15],
        },
    })
}
