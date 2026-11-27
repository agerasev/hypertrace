//! Persistent tile-local instances and immutable camera-frame snapshots.
use std::{
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
    rc::{Rc, Weak},
};

use anyhow::Result;
use wgame::gfx::{BakedScene, Instance, Scene};
use wgpu::util::DeviceExt;

use super::{Pose, Shader, instance_capacity, projective_pair};
use crate::{
    atmosphere::{AirFrame, FrameUniform, frame_uniform},
    world::GroundTile,
};

struct BakedTile {
    // A weak reference keeps this cache key unique even after the tile is evicted.
    source: Weak<GroundTile>,
    meshes: Vec<BakedScene>,
    planes: Vec<BakedScene>,
    frame_buffer: wgpu::Buffer,
}

impl BakedTile {
    fn new(shader: &Shader, tile: &Rc<GroundTile>) -> Result<Self> {
        let (frame_buffer, binding) = shader.air.tile_binding();
        let sun_in_tile = tile.pose.inverse()?.chain(&shader.sun_pose)?;
        let mut meshes = Vec::new();
        // Even a whole tile of identical objects must fit Wgame's instance buffers.
        let capacity = instance_capacity(shader.air.device()) as usize;
        for chunk in tile.meshes.chunks(capacity) {
            // Ground surfaces write depth. Group opaque instances by their rendering
            // resources, retaining first-seen batch order and insertion order within each.
            #[expect(
                clippy::mutable_key_type,
                reason = "Wgame hashes stable GPU resource handles"
            )]
            let mut indices = HashMap::new();
            let mut batches = Vec::new();
            for object in chunk {
                let mesh = shader.shaded_mesh(
                    &object.shape,
                    &shader.tile_mesh,
                    object.pose,
                    object.pose.inverse()?.chain(&sun_in_tile)?,
                    object.material,
                    &binding,
                )?;
                let index = indices.entry(mesh.resource()).or_insert_with(|| {
                    batches.push(Scene::default());
                    batches.len() - 1
                });
                batches[*index].add(&mesh);
            }
            meshes.extend(batches.iter().map(Scene::bake));
        }
        // All planes share a quad and bindings. Wgame batches their transforms
        // and materials into one instanced draw per buffer-sized chunk.
        let mut planes = Vec::new();
        for chunk in tile.planes.chunks(capacity) {
            let mut batch = Scene::default();
            for plane in chunk {
                batch.add(&shader.shaded_plane(
                    &shader.tile_plane,
                    plane,
                    plane.pose,
                    sun_in_tile,
                    &binding,
                )?);
            }
            planes.push(batch.bake());
        }
        Ok(Self {
            source: Rc::downgrade(tile),
            meshes,
            planes,
            frame_buffer,
        })
    }
}

#[derive(Default)]
pub(super) struct TileCache(RefCell<HashMap<*const GroundTile, Rc<BakedTile>>>);

impl TileCache {
    pub(super) fn prepare(
        &self,
        shader: &Shader,
        tiles: &[Rc<GroundTile>],
        view: Pose,
        air: AirFrame,
    ) -> Result<TileDrawing> {
        let device = shader.air.device();
        let frames_per_buffer =
            usize::try_from(device.limits().max_buffer_size / FRAME_SIZE).unwrap_or(usize::MAX);
        let mut cache = self.0.borrow_mut();
        cache.retain(|_, tile| tile.source.strong_count() > 0);
        let mut groups = Vec::new();
        for chunk in tiles.chunks(frames_per_buffer) {
            let mut baked = Vec::new();
            let mut frames = Vec::new();
            for tile in chunk
                .iter()
                .filter(|tile| !tile.meshes.is_empty() || !tile.planes.is_empty())
            {
                let surface = match cache.entry(Rc::as_ptr(tile)) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => entry.insert(Rc::new(BakedTile::new(shader, tile)?)),
                };
                let relative = projective_pair(view.chain(&tile.pose)?)?;
                frames.push(frame_uniform(air, relative));
                baked.push(Rc::clone(surface));
            }
            if !frames.is_empty() {
                let upload = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera-relative ground tile frames"),
                    contents: bytemuck::cast_slice(&frames),
                    usage: wgpu::BufferUsages::COPY_SRC,
                });
                groups.push(TileGroup { baked, upload });
            }
        }
        Ok(TileDrawing { groups })
    }
}

const FRAME_SIZE: u64 = size_of::<FrameUniform>() as u64;

struct TileGroup {
    baked: Vec<Rc<BakedTile>>,
    upload: wgpu::Buffer,
}

pub(super) struct TileDrawing {
    groups: Vec<TileGroup>,
}

impl TileDrawing {
    pub(super) fn upload(&self, encoder: &mut wgpu::CommandEncoder) {
        // The copy precedes this drawing's render pass in the same command stream.
        // Drawing a second view before submission cannot overwrite the first view.
        for group in &self.groups {
            for (i, tile) in group.baked.iter().enumerate() {
                encoder.copy_buffer_to_buffer(
                    &group.upload,
                    i as u64 * FRAME_SIZE,
                    &tile.frame_buffer,
                    0,
                    FRAME_SIZE,
                );
            }
        }
    }

    fn tiles(&self) -> impl Iterator<Item = &Rc<BakedTile>> {
        self.groups.iter().flat_map(|group| &group.baked)
    }

    pub(super) fn planes(&self) -> impl Iterator<Item = &BakedScene> {
        self.tiles().flat_map(|tile| &tile.planes)
    }

    pub(super) fn meshes(&self) -> impl Iterator<Item = &BakedScene> {
        self.tiles().flat_map(|tile| &tile.meshes)
    }
}
