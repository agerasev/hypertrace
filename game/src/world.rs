use std::{collections::BTreeMap, f64::consts::TAU, ops::Range};

use anyhow::{Context, Result, ensure};
use ccgeom::{Geometry3, Hyperboloid3, Space3};
use wgame::{
    Library,
    glam::{Vec3, Vec4},
    shapes::{Mesh, Polygon, shader::Vertex},
};

use crate::{
    atmosphere::Atmosphere,
    config::{AmbientLight, Config, vegetation::Vegetation},
    projection::Pose,
};

mod ecology;
mod ground_cells;
mod material;
mod moon;
mod plants;
mod stars;

use ecology::{Daylight, Species};
pub(crate) use ground_cells::Patch;
use ground_cells::{RootSampler, TileId};
pub(crate) use material::Material;
use plants::Plants;

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

pub struct World {
    pub(crate) atmosphere: Atmosphere,
    objects: Objects,
    fixed_meshes: Vec<Object<Polygon>>,
    patch: Patch,
    center: Option<TileId>,
    tile_meshes: BTreeMap<TileId, Range<usize>>,
    plants: Plants,
    vegetation: Vegetation,
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

// SplitMix64. Each tile has a separate stream independent of visit order.
struct Samples(u64);

impl Samples {
    const STEP: u64 = 0x9e3779b97f4a7c15;

    fn at_offset(seed: u64, offset: u64) -> Self {
        Self(seed.wrapping_add(Self::STEP.wrapping_mul(offset)))
    }

    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(Self::STEP);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        ((z ^ (z >> 31)) >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Stochastic rounding preserves the mean even below one site per pentagon.
    fn site_count(&mut self, mean: f64) -> u32 {
        mean.floor() as u32 + u32::from(self.next() < mean.fract())
    }
}

impl World {
    /// Rebuild the local patch only when the observer enters a new pentagon.
    pub fn objects_near(&mut self, eye: Pose) -> Result<&Objects> {
        let center = TileId::at(eye)?;
        if self.center.as_ref() != Some(&center) {
            let mut meshes = Vec::with_capacity(self.objects.meshes.len());
            meshes.extend_from_slice(&self.fixed_meshes);
            let mut tile_meshes = BTreeMap::new();
            for tile in self.patch.tiles(&center) {
                let start = meshes.len();
                if let Some(range) = self.tile_meshes.get(&tile) {
                    meshes.extend_from_slice(&self.objects.meshes[range.clone()]);
                } else {
                    self.plant_tile(&tile, &mut meshes)?;
                }
                tile_meshes.insert(tile, start..meshes.len());
            }
            self.objects.meshes = meshes;
            self.tile_meshes = tile_meshes;
            self.center = Some(center);
        }
        Ok(&self.objects)
    }

    fn plant_tile(&self, tile: &TileId, meshes: &mut Vec<Object<Polygon>>) -> Result<()> {
        let vegetation = &self.vegetation;
        let mut samples = Samples(tile.seed(vegetation.seed));
        let count = samples.site_count(vegetation.mean_sites_per_tile());
        if count == 0 {
            return Ok(());
        }
        let sampler = RootSampler::new(tile)?;
        let [min_maturity, max_maturity] = vegetation.maturity;
        for _ in 0..count {
            for species in [Species::Deciduous, Species::Pine, Species::Grass] {
                let root = sampler.root(&mut samples)?;
                let occupied = samples.next();
                let maturity = min_maturity + (max_maturity - min_maturity) * samples.next();
                let yaw = TAU * samples.next();
                let light = self.daylight.at(root.apply_vector([1.0, 0.0, 0.0, 0.0]));
                if occupied >= species.density(light, vegetation) {
                    continue;
                }
                let (shape, material) = self
                    .plants
                    .grown(species, maturity * species.growth(light, vegetation));
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

pub fn world(library: &Library, config: &Config, patch: Patch) -> Result<World> {
    let sun = Sun {
        pose: Pose::from_isometry(Hyperboloid3::shift_y(config.sun.distance))?,
        radiance: config.sun.emission,
    };
    let earth = Pose::identity();
    let mut planes = vec![
        Object {
            shape: InfinitePlane,
            pose: earth,
            material: config.earth.material,
        },
        Object {
            shape: InfinitePlane,
            pose: sun.pose,
            material: Material::emissive(sun.radiance),
        },
    ];
    crate::projection::check_plane_capacity(
        library,
        planes.len() as u64 + u64::from(config.stars.count),
    )?;
    planes.extend(stars::catalogue(&config.stars)?);
    Ok(World {
        atmosphere: Atmosphere {
            ground: earth,
            height: config.atmosphere.height,
            background: config.atmosphere.background,
            scattering: config.atmosphere.scattering,
        },
        objects: Objects {
            planes,
            ..Objects::default()
        },
        fixed_meshes: vec![moon::moon(library, &config.moon)?],
        center: None,
        tile_meshes: BTreeMap::new(),
        patch,
        plants: Plants::new(library, &config.vegetation)?,
        vegetation: config.vegetation.clone(),
        daylight: Daylight::new(earth, &sun)?,
        sun,
        ambient: config.ambient,
    })
}

#[cfg(test)]
mod tests {
    use super::Samples;

    #[test]
    fn fractional_site_counts_keep_the_requested_mean() {
        for mean in [0.0_f64, 0.25, 2.0, 20.625] {
            let mut samples = Samples(7);
            let mut total = 0;
            for _ in 0..16_384 {
                let n = samples.site_count(mean);
                assert!((mean.floor() as u32..=mean.ceil() as u32).contains(&n));
                total += n;
            }
            assert!((f64::from(total) / 16_384.0 - mean).abs() < 0.01);
        }
    }
}
