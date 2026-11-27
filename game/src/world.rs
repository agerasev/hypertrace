use std::{collections::BTreeMap, f64::consts::TAU, rc::Rc};

use anyhow::{Context, Result, ensure};
use ccgeom::{Geometry3, Hyperboloid3, Space3};
use wgame::{
    Library,
    glam::{Vec3, Vec4},
    shapes::{Mesh, Polygon, shader::Vertex},
};

use crate::{
    atmosphere::Atmosphere,
    config::{AmbientLight, Config, stars::StarField, vegetation::Vegetation},
    projection::Pose,
};

mod ecology;
mod ground_cells;
mod material;
pub(crate) mod moon;
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

/// Instances batched for drawing in one Earth tile's coordinate frame.
#[derive(Clone)]
pub(crate) struct GroundTile {
    pub pose: Pose,
    pub meshes: Vec<Object<Polygon>>,
    pub planes: Vec<Object<InfinitePlane>>,
}

#[derive(Default)]
pub struct Objects {
    pub meshes: Vec<Object<Polygon>>,
    pub planes: Vec<Object<InfinitePlane>>,
    pub tiles: Vec<Rc<GroundTile>>,
}

pub(crate) const MAX_SUNS: usize = 3;

#[derive(Clone, Copy)]
pub struct Sun {
    pub pose: Pose,
    pub surface_radiance: [f32; 3],
    pub scattering_radiance: [f32; 3],
    pub ambient: crate::config::SunAmbient,
}

pub struct World {
    pub(crate) atmosphere: Atmosphere,
    objects: Objects,
    patch: Patch,
    stars: StarField,
    center: Option<TileId>,
    loaded: BTreeMap<TileId, usize>,
    placed: BTreeMap<TileId, Vec<Object<Polygon>>>,
    plants: Plants,
    vegetation: Vegetation,
    daylight: Daylight,
    pub suns: Vec<Sun>,
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
    /// Place a finite mesh at a world pose. Its foot on Earth selects the
    /// streaming tile; the complete orientation and height are preserved.
    pub fn place_on_ground(&mut self, mut object: Object<Polygon>) -> Result<()> {
        let id = TileId::at(object.pose)?;
        let frame = id.upright_frame()?;
        object.pose = frame.inverse()?.chain(&object.pose)?;
        if let Some(&index) = self.loaded.get(&id) {
            // A previously prepared drawing may still refer to the old tile.
            Rc::make_mut(&mut self.objects.tiles[index])
                .meshes
                .push(object.clone());
        }
        self.placed.entry(id).or_default().push(object);
        Ok(())
    }

    /// Rebuild the local patch only when the observer enters a new pentagon.
    pub fn objects_near(&mut self, eye: Pose) -> Result<&Objects> {
        let center = TileId::at(eye)?;
        if self.center.as_ref() != Some(&center) {
            let mut tiles = Vec::with_capacity(self.objects.tiles.len());
            let mut loaded = BTreeMap::new();
            for id in self.patch.tiles(&center) {
                let tile = match self.loaded.get(&id) {
                    Some(&index) => Rc::clone(&self.objects.tiles[index]),
                    None => Rc::new(self.load_tile(&id)?),
                };
                loaded.insert(id, tiles.len());
                tiles.push(tile);
            }
            self.objects.tiles = tiles;
            self.loaded = loaded;
            self.center = Some(center);
        }
        Ok(&self.objects)
    }

    fn load_tile(&self, id: &TileId) -> Result<GroundTile> {
        let pose = id.upright_frame()?;
        let mut meshes = self.plants_in_tile(id, pose)?;
        if let Some(placed) = self.placed.get(id) {
            meshes.extend_from_slice(placed);
        }
        Ok(GroundTile {
            pose,
            meshes,
            planes: stars::in_tile(&self.stars, id)?,
        })
    }

    fn plants_in_tile(&self, tile: &TileId, frame: Pose) -> Result<Vec<Object<Polygon>>> {
        let vegetation = &self.vegetation;
        let mut samples = Samples(tile.seed(vegetation.seed));
        let count = samples.site_count(vegetation.mean_sites_per_tile());
        if count == 0 {
            return Ok(Vec::new());
        }
        let sampler = RootSampler::new();
        let mut meshes = Vec::new();
        let [min_maturity, max_maturity] = vegetation.maturity;
        for _ in 0..count {
            for species in [Species::Deciduous, Species::Pine, Species::Grass] {
                let root = sampler.local_root(&mut samples)?;
                let occupied = samples.next();
                let maturity = min_maturity + (max_maturity - min_maturity) * samples.next();
                let yaw = TAU * samples.next();
                let position = frame.apply_vector(root.apply_vector([1.0, 0.0, 0.0, 0.0]));
                let (density, growth) = self.daylight.at(position, species, vegetation);
                if occupied >= density {
                    continue;
                }
                let (shape, material) = self.plants.grown(species, maturity * growth);
                meshes.push(Object {
                    shape,
                    pose: root.chain(&Pose::from_isometry(Hyperboloid3::rotate_y(yaw))?)?,
                    material,
                });
            }
        }
        Ok(meshes)
    }
}

pub fn world(
    library: &Library,
    config: &Config,
    patch: Patch,
    ground_objects: impl IntoIterator<Item = Object<Polygon>>,
) -> Result<World> {
    let earth = Pose::identity();
    let mut planes = vec![Object {
        shape: InfinitePlane,
        pose: earth,
        material: config.earth.material,
    }];
    let mut suns = Vec::with_capacity(config.suns.len());
    for source in &config.suns {
        let pose = Pose::from_isometry(
            Hyperboloid3::shift_x(source.ground_offset[0])
                .chain(Hyperboloid3::shift_z(source.ground_offset[1]))
                .chain(Hyperboloid3::shift_y(source.distance)),
        )?;
        planes.push(Object {
            shape: InfinitePlane,
            pose,
            material: Material::emissive(source.surface_radiance),
        });
        suns.push(Sun {
            pose,
            surface_radiance: source.surface_radiance,
            scattering_radiance: source.scattering_radiance,
            ambient: source.ambient,
        });
    }
    let daylight = Daylight::new(
        earth,
        suns.iter().zip(config.suns.iter().map(|s| &s.vegetation)),
    )?;
    let mut world = World {
        atmosphere: Atmosphere {
            ground: earth,
            height: config.atmosphere.height,
            background: config.atmosphere.background,
        },
        objects: Objects {
            planes,
            meshes: vec![moon::moon(library, &config.moon)?],
            ..Objects::default()
        },
        center: None,
        loaded: BTreeMap::new(),
        placed: BTreeMap::new(),
        patch,
        stars: config.stars,
        plants: Plants::new(library, &config.vegetation)?,
        vegetation: config.vegetation.clone(),
        daylight,
        suns,
        ambient: config.ambient,
    };
    for object in ground_objects {
        world.place_on_ground(object)?;
    }
    Ok(world)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gpu_tests::headless_library;

    #[test]
    #[ignore = "requires a graphics adapter"]
    fn placed_ground_meshes_survive_streaming_and_only_replace_their_own_tile() -> Result<()> {
        let library = headless_library()?;
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("config.toml");
        let mut config = Config::load(&path)?;
        config.vegetation.site_density_per_species = 0.0;
        config.stars.density = 2.0;
        config.earth.grid.view_radius = 1.0;
        let patch = Patch::new(&config.earth.grid, config.max_sites_per_tile())?;
        let shape = mesh(
            &library,
            &[
                ([0.0, 0.0, 1.0], [1.0; 3]),
                ([0.1, 0.0, 1.0], [1.0; 3]),
                ([0.0, 0.1, 1.0], [1.0; 3]),
            ],
            &[0, 1, 2],
        )?;
        let origin = TileId::default();
        let id = origin.neighbor(0);
        let frame = id.upright_frame()?;
        let poses = [
            frame.move_local([0.1, 0.05, -0.1], [0.2, 0.1, -0.4], 1.0)?,
            frame.move_local([-0.2, 0.04, 0.0], [0.1, -0.3, 0.7], 1.0)?,
        ];
        let object = |pose| Object {
            shape: shape.clone(),
            pose,
            material: Material::emissive([1.0, 0.2, 0.0]),
        };
        let mut world = world(&library, &config, patch, [object(poses[0])])?;
        let loaded = |world: &World, id: &TileId| Rc::clone(&world.objects.tiles[world.loaded[id]]);
        world.objects_near(Pose::identity())?;
        let origin_tile = loaded(&world, &origin);
        let first = loaded(&world, &id);
        assert!(!first.planes.is_empty());
        world.objects_near(frame)?;
        assert!(Rc::ptr_eq(&first, &loaded(&world, &id)));
        world.place_on_ground(object(poses[1]))?;
        assert!(Rc::ptr_eq(&origin_tile, &loaded(&world, &origin)));
        assert_eq!(first.meshes.len(), 1);
        assert_eq!(loaded(&world, &id).meshes.len(), 2);

        world.objects_near(Pose::from_isometry(Hyperboloid3::shift_z(6.0))?)?;
        assert!(!world.loaded.contains_key(&id));
        world.objects_near(frame)?;
        let restored = loaded(&world, &id);
        assert_eq!(restored.meshes.len(), 2);
        assert_eq!(first.planes.len(), restored.planes.len());
        for (original, regenerated) in first.planes.iter().zip(&restored.planes) {
            assert_eq!(original.pose.components()?, regenerated.pose.components()?);
            assert_eq!(original.material.emission, regenerated.material.emission);
            assert_eq!(TileId::at(restored.pose.chain(&regenerated.pose)?)?, id);
        }
        for (mesh, pose) in restored.meshes.iter().zip(poses) {
            let relative = pose.inverse()?.chain(&restored.pose.chain(&mesh.pose)?)?;
            for axis in 0..4 {
                let mut basis = [0.0; 4];
                basis[axis] = 1.0;
                let mapped = relative.apply_vector(basis);
                assert!(
                    mapped
                        .into_iter()
                        .zip(basis)
                        .all(|(a, b)| (a - b).abs() < 1e-12)
                );
            }
        }
        Ok(())
    }

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
