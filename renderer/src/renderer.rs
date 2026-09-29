use bytemuck::Zeroable;
use wgpu::util::DeviceExt;

#[cfg(not(target_arch = "wasm32"))]
use crate::read_buffer;
use crate::{Camera, Result, Scene, scene::Params};
use scene_ir::{Geometry, GpuObject, MaterialRecord};

fn scene_shader_source<G: Geometry>(scene: &Scene<G>) -> String {
    [
        include_str!("shaders/math.wgsl"),
        include_str!("shaders/embedded.wgsl"),
        include_str!("shaders/transport.wgsl"),
        include_str!("shaders/abi.wgsl"),
        include_str!("shaders/trace.wgsl"),
        include_str!("shaders/tracing_common.wgsl"),
        &scene.compiled.source,
    ]
    .join("\n")
}

/// Progressive renderer sharing the caller's device/queue. Normal frames only
/// encode compute; snapshots are explicit blocking operations.
///
/// Scene updates preserve the renderer's geometry at compile time:
/// ```compile_fail
/// use ccgeom::{Flat3, Hyperboloid3};
/// use hypertrace_renderer::{Renderer, Scene, Result};
/// async fn replace(renderer: &mut Renderer<Flat3>, scene: Scene<Hyperboloid3>) -> Result<()> {
///     renderer.update_scene_async(scene).await
/// }
/// ```
pub struct Renderer<G: Geometry> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bindings: wgpu::BindGroup,
    params: wgpu::Buffer,
    objects: wgpu::Buffer,
    materials: wgpu::Buffer,
    words: wgpu::Buffer,
    accumulation: wgpu::Buffer,
    seeds: wgpu::Buffer,
    scene: Scene<G>,
    size: (u32, u32),
    seed: u32,
    samples: u32,
    source: String,
    pipeline_revision: u64,
}

impl<G: Geometry> Renderer<G> {
    /// Blocking native convenience wrapper for [`Self::new_async`].
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        scene: Scene<G>,
        seed: u32,
    ) -> Result<Self> {
        futures::executor::block_on(Self::new_async(device, queue, size, scene, seed))
    }

    /// Create a renderer, yielding while the GPU validates the scene program.
    /// Browser hosts must await this rather than block the JavaScript event loop.
    pub async fn new_async(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        scene: Scene<G>,
        seed: u32,
    ) -> Result<Self> {
        validate_size(device, size)?;
        validate_scene(device, &scene)?;
        let prepared_objects = scene.prepare_objects(scene.camera)?;
        let source = scene_shader_source(&scene);
        let pipeline = create_pipeline(device, &source).await?;
        let params = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("trace params"),
            contents: bytemuck::bytes_of(&Params::new(&scene, size, 1)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let objects = scene_buffer(device, "objects", &prepared_objects, GpuObject::zeroed());
        let materials = material_buffer(device, &scene);
        let words = scene_buffer(device, "scene words", scene.words(), 0u32);
        let (accumulation, seeds) = pixel_buffers(device, size);
        let bindings = bind_group(
            device,
            &pipeline,
            [&params, &objects, &materials, &accumulation, &seeds, &words],
        );
        let mut renderer = Self {
            device: device.clone(),
            queue: queue.clone(),
            pipeline,
            bindings,
            params,
            objects,
            materials,
            words,
            accumulation,
            seeds,
            scene,
            size,
            seed,
            samples: 1,
            source,
            pipeline_revision: 1,
        };
        renderer.reset(seed);
        Ok(renderer)
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }
    pub fn scene(&self) -> &Scene<G> {
        &self.scene
    }
    /// Flattened source used by the current pipeline, including extension code.
    pub fn shader_source(&self) -> &str {
        &self.source
    }
    /// Increments only when a different shader program is successfully installed.
    pub fn pipeline_revision(&self) -> u64 {
        self.pipeline_revision
    }
    pub fn accumulation_buffer(&self) -> &wgpu::Buffer {
        &self.accumulation
    }
    pub fn params_buffer(&self) -> &wgpu::Buffer {
        &self.params
    }

    /// Blocking native convenience wrapper for [`Self::update_scene_async`].
    #[cfg(not(target_arch = "wasm32"))]
    pub fn update_scene(&mut self, scene: Scene<G>) -> Result<()> {
        futures::executor::block_on(self.update_scene_async(scene))
    }

    /// Update data, compiling only if the scene's shader structure changes.
    /// A failed compilation leaves the previous renderer usable.
    /// Buffers grow only when needed.
    /// A geometry/camera/material change always invalidates accumulated samples.
    pub async fn update_scene_async(&mut self, scene: Scene<G>) -> Result<()> {
        validate_scene(&self.device, &scene)?;
        let prepared_objects = scene.prepare_objects(scene.camera)?;
        let source = scene_shader_source(&scene);
        let new_pipeline = if source != self.source {
            Some(create_pipeline(&self.device, &source).await?)
        } else {
            None
        };
        let grow_objects = scene.objects().len().max(1) as u64 * size_of::<GpuObject>() as u64
            > self.objects.size();
        let grow_materials = scene
            .material_bytes()
            .len()
            .max(size_of::<MaterialRecord>()) as u64
            > self.materials.size();
        let grow_words = std::mem::size_of_val(scene.words()).max(4) as u64 > self.words.size();
        if grow_objects {
            self.objects = scene_buffer(
                &self.device,
                "objects",
                &prepared_objects,
                GpuObject::zeroed(),
            );
        } else if !prepared_objects.is_empty() {
            self.queue
                .write_buffer(&self.objects, 0, bytemuck::cast_slice(&prepared_objects));
        }
        if grow_materials {
            self.materials = material_buffer(&self.device, &scene);
        } else if !scene.material_bytes().is_empty() {
            self.queue
                .write_buffer(&self.materials, 0, scene.material_bytes());
        }
        if grow_words {
            self.words = scene_buffer(&self.device, "scene words", scene.words(), 0u32);
        } else if !scene.words().is_empty() {
            self.queue
                .write_buffer(&self.words, 0, bytemuck::cast_slice(scene.words()));
        }
        let recompile = new_pipeline.is_some();
        if let Some(pipeline) = new_pipeline {
            self.pipeline = pipeline;
            self.source = source;
            self.pipeline_revision += 1;
        }
        self.scene = scene;
        if grow_objects || grow_materials || grow_words || recompile {
            self.rebind();
        }
        self.write_params();
        self.reset(self.seed);
        Ok(())
    }

    pub fn update_camera(&mut self, camera: Camera<G>, fov: f32) -> Result<()> {
        anyhow::ensure!(fov.is_finite() && fov > 0.0, "invalid camera");
        let prepared_objects = self.scene.prepare_objects(camera)?;
        if !prepared_objects.is_empty() {
            self.queue
                .write_buffer(&self.objects, 0, bytemuck::cast_slice(&prepared_objects));
        }
        self.scene.camera = camera;
        self.scene.fov = fov;
        self.write_params();
        self.reset(self.seed);
        Ok(())
    }

    /// Reject zero dimensions; a window host should skip minimized frames.
    /// Existing resources remain usable if validation fails.
    pub fn resize(&mut self, size: (u32, u32)) -> Result<bool> {
        validate_size(&self.device, size)?;
        if size == self.size {
            return Ok(false);
        }
        (self.accumulation, self.seeds) = pixel_buffers(&self.device, size);
        self.size = size;
        self.rebind();
        self.write_params();
        self.reset(self.seed);
        Ok(true)
    }

    pub fn set_samples_per_dispatch(&mut self, samples: u32) -> Result<()> {
        anyhow::ensure!(
            (1..=1024).contains(&samples),
            "samples per dispatch must be 1..=1024"
        );
        self.samples = samples;
        self.write_params();
        Ok(())
    }

    /// Queue a reset independently of a window frame, so discarding that frame
    /// cannot leave stale accumulation paired with a changed camera.
    pub fn reset(&mut self, seed: u32) {
        self.seed = seed;
        let count = self.size.0 as usize * self.size.1 as usize;
        self.queue
            .write_buffer(&self.accumulation, 0, &vec![0u8; count * 16]);
        let seeds: Vec<_> = (0..count as u32).map(|i| pixel_seed(seed, i)).collect();
        self.queue
            .write_buffer(&self.seeds, 0, bytemuck::cast_slice(&seeds));
    }

    /// Encode after all parameter writes for this submission. Queue writes take
    /// effect before submission: submit already encoded work before changing
    /// parameters or resetting. This method does not submit or block.
    pub fn encode(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("hypertrace sample"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bindings, &[]);
        pass.dispatch_workgroups(self.size.0.div_ceil(8), self.size.1.div_ceil(8), 1);
    }

    pub fn render(&self) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        self.encode(&mut encoder);
        self.queue.submit([encoder.finish()]);
    }

    /// Top-to-bottom linear RGBA; RGB is averaged, alpha is one after at least
    /// one sample (zero before any samples). No display transfer is applied.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn snapshot(&self) -> Result<Vec<[f32; 4]>> {
        let bytes = read_buffer(
            &self.device,
            &self.queue,
            &self.accumulation,
            self.accumulation.size(),
        )?;
        bytes
            .as_chunks::<16>()
            .0
            .iter()
            .map(|chunk| {
                let p: [f32; 4] = bytemuck::pod_read_unaligned(chunk);
                anyhow::ensure!(p.into_iter().all(f32::is_finite), "non-finite GPU pixel");
                Ok(if p[3] > 0.0 {
                    [p[0] / p[3], p[1] / p[3], p[2] / p[3], 1.0]
                } else {
                    [0.0; 4]
                })
            })
            .collect()
    }

    fn write_params(&self) {
        self.queue.write_buffer(
            &self.params,
            0,
            bytemuck::bytes_of(&Params::new(&self.scene, self.size, self.samples)),
        );
    }
    fn rebind(&mut self) {
        self.bindings = bind_group(
            &self.device,
            &self.pipeline,
            [
                &self.params,
                &self.objects,
                &self.materials,
                &self.accumulation,
                &self.seeds,
                &self.words,
            ],
        );
    }
}

pub fn pixel_seed(seed: u32, index: u32) -> u32 {
    let mut x = seed ^ (index.wrapping_add(1).wrapping_mul(0x9e3779b9));
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846ca68b);
    x ^ (x >> 16)
}

fn validate_size(device: &wgpu::Device, size: (u32, u32)) -> Result<()> {
    crate::resolution::validate_render_size(&device.limits(), size)
}
fn validate_scene<G: Geometry>(device: &wgpu::Device, scene: &Scene<G>) -> Result<()> {
    scene.validate()?;
    let limits = device.limits();
    let limit = limits
        .max_storage_buffer_binding_size
        .min(limits.max_buffer_size) as usize;
    anyhow::ensure!(
        scene.objects().len().max(1) <= limit / size_of::<GpuObject>()
            && scene
                .material_bytes()
                .len()
                .max(size_of::<MaterialRecord>())
                <= limit
            && std::mem::size_of_val(scene.words()).max(4) <= limit,
        "scene exceeds device buffer limit"
    );
    Ok(())
}
fn material_buffer<G: Geometry>(device: &wgpu::Device, scene: &Scene<G>) -> wgpu::Buffer {
    // An empty scene still binds one valid storage element.
    let mut bytes = scene.material_bytes().to_vec();
    bytes.resize(bytes.len().max(size_of::<MaterialRecord>()), 0);
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("materials"),
        contents: &bytes,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    })
}

async fn create_pipeline(device: &wgpu::Device, source: &str) -> Result<wgpu::ComputePipeline> {
    let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    // Specialization can remove every access to some bindings; an explicit
    // layout preserves the ABI even for empty scenes and parameterless leaves.
    let entries: Vec<_> = (0..6)
        .map(|binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: if binding == 0 {
                    wgpu::BufferBindingType::Uniform
                } else {
                    wgpu::BufferBindingType::Storage {
                        read_only: binding != 3 && binding != 4,
                    }
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
        .collect();
    let bindings = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("trace ABI"),
        entries: &entries,
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("trace ABI"),
        bind_group_layouts: &[Some(&bindings)],
        immediate_size: 0,
    });
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("hypertrace generated WGSL"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("hypertrace path tracing"),
        layout: Some(&layout),
        module: &shader,
        entry_point: Some("render"),
        compilation_options: Default::default(),
        cache: None,
    });
    if let Some(error) = scope.pop().await {
        anyhow::bail!("WGSL pipeline: {error}");
    }
    Ok(pipeline)
}
fn scene_buffer<T: bytemuck::Pod>(
    device: &wgpu::Device,
    label: &str,
    data: &[T],
    dummy: T,
) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::cast_slice(if data.is_empty() {
            std::slice::from_ref(&dummy)
        } else {
            data
        }),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    })
}
fn pixel_buffers(device: &wgpu::Device, size: (u32, u32)) -> (wgpu::Buffer, wgpu::Buffer) {
    let count = u64::from(size.0) * u64::from(size.1);
    let buffer = |label, bytes| {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: bytes,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        })
    };
    (
        buffer("accumulation", count * 16),
        buffer("random seeds", count * 4),
    )
}
fn bind_group(
    device: &wgpu::Device,
    pipeline: &wgpu::ComputePipeline,
    buffers: [&wgpu::Buffer; 6],
) -> wgpu::BindGroup {
    let entries: Vec<_> = buffers
        .into_iter()
        .enumerate()
        .map(|(binding, b)| wgpu::BindGroupEntry {
            binding: binding as u32,
            resource: b.as_entire_binding(),
        })
        .collect();
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("trace buffers"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &entries,
    })
}
