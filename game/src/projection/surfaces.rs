//! Instanced surface materials and their object-local solar planes.
use anyhow::{Result, ensure};
use wgame::{
    glam::{Vec2, Vec3},
    shader::Attribute,
    shapes::{
        Polygon,
        material::{MaterialBinding, MaterialMesh, MeshShader},
        prelude::ShapeFill,
    },
};
use wgpu::util::DeviceExt;

use super::{Instance, Pose, Shader, plane_foot, projective_pair};
use crate::{
    config::AmbientLight,
    world::{InfinitePlane, LightingDomain, MAX_SUNS, Material, Object, Sun},
};

pub(super) fn instance_capacity(device: &wgpu::Device) -> u64 {
    let stride = wgame::shapes::shader::InstanceData::<Instance>::bindings().size();
    (device.limits().max_buffer_size / stride).min(SURFACES_PER_BATCH as u64)
}

// Wgame's stock instance layout leaves five vertex attribute locations. Keep
// each object's f64-prepared light planes in a batch uniform and index it with
// one attribute. 16 KiB is available even under Wgame's downlevel limits.
type PlaneRecord = [[f32; 4]; MAX_SUNS];
pub(super) const SURFACES_PER_BATCH: usize = (16 * 1024) / size_of::<PlaneRecord>();
const PLANE_BUFFER_SIZE: u64 = (SURFACES_PER_BATCH * size_of::<PlaneRecord>()) as u64;

pub(super) struct SurfaceLights {
    pub(super) poses: Vec<Pose>,
    pub(super) layout: wgpu::BindGroupLayout,
    parameters: wgpu::Buffer,
    pub(super) unlit: MaterialBinding,
}

impl SurfaceLights {
    pub(super) fn new(device: &wgpu::Device, suns: &[Sun], ambient: AmbientLight) -> Result<Self> {
        let mut words = [[0.0_f32; 4]; 1 + 2 * MAX_SUNS];
        let mut poses = Vec::new();
        for sun in suns.iter().filter(|s| s.surface_radiance != [0.0; 3]) {
            let i = poses.len();
            poses.push(sun.pose);
            words[1 + 2 * i] = Vec3::from_array(sun.surface_radiance)
                .extend(sun.ambient.response_factor)
                .to_array();
            words[2 + 2 * i] = Vec3::from_array(sun.ambient.color)
                .extend(sun.ambient.response_exponent)
                .to_array();
        }
        words[0] = Vec3::from_array(ambient.night)
            .extend(poses.len() as f32)
            .to_array();
        ensure!(
            words.iter().flatten().all(|&x| x.is_finite() && x >= 0.0),
            "invalid surface lighting parameters"
        );
        let uniform = |binding, visibility, size| wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new(size),
            },
            count: None,
        };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Surface lighting"),
            entries: &[
                uniform(0, wgpu::ShaderStages::FRAGMENT, size_of_val(&words) as u64),
                uniform(1, wgpu::ShaderStages::VERTEX, PLANE_BUFFER_SIZE),
            ],
        });
        let parameters = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Surface light radiances"),
            contents: bytemuck::cast_slice(&words),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let unlit = Self::bind(device, &layout, &parameters, &[])?;
        Ok(Self {
            poses,
            layout,
            parameters,
            unlit,
        })
    }

    fn bind(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        parameters: &wgpu::Buffer,
        records: &[PlaneRecord],
    ) -> Result<MaterialBinding> {
        let planes = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Object-local Sun planes"),
            size: PLANE_BUFFER_SIZE,
            usage: wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: !records.is_empty(),
        });
        if !records.is_empty() {
            let source = bytemuck::cast_slice(records);
            planes
                .slice(..source.len() as u64)
                .get_mapped_range_mut()?
                .copy_from_slice(source);
            planes.unmap();
        }
        Ok(MaterialBinding::Uniforms(device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Surface lighting"),
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: parameters.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: planes.as_entire_binding(),
                    },
                ],
            },
        )))
    }
}

/// A surface in its (possibly recentered) drawing chart. The relative pose
/// positions its vertices; the parent pose brings lights into the same chart.
struct Surface<'a> {
    polygon: &'a Polygon,
    relative: Pose,
    local_to_parent: Pose,
    material: Material,
    domain: LightingDomain,
}

impl Shader {
    pub(super) fn mesh_batch(
        &self,
        objects: &[Object<Polygon>],
        view: Pose,
        suns: &[Pose],
        pipeline: &MeshShader<Instance>,
        air: &MaterialBinding,
    ) -> Result<Vec<MaterialMesh<Instance>>> {
        self.batch(
            objects.iter().map(|object| {
                Ok(Surface {
                    polygon: &object.shape,
                    relative: view.chain(&object.pose)?,
                    local_to_parent: object.pose,
                    material: object.material,
                    domain: object.domain,
                })
            }),
            suns,
            pipeline,
            air,
        )
    }

    pub(super) fn plane_batch(
        &self,
        objects: &[Object<InfinitePlane>],
        view: Pose,
        suns: &[Pose],
        pipeline: &MeshShader<Instance>,
        air: &MaterialBinding,
    ) -> Result<Vec<MaterialMesh<Instance>>> {
        self.batch(
            objects.iter().map(|object| {
                let relative = view.chain(&object.pose)?;
                // Center the quad at the eye's foot for global planes, or the
                // tile center's foot when baking an immutable ground tile.
                let foot = plane_foot(relative)?;
                Ok(Surface {
                    polygon: &self.plane_quad,
                    relative: relative.chain(&foot)?,
                    local_to_parent: object.pose.chain(&foot)?,
                    material: object.material,
                    domain: object.domain,
                })
            }),
            suns,
            pipeline,
            air,
        )
    }

    fn batch<'a>(
        &self,
        surfaces: impl Iterator<Item = Result<Surface<'a>>>,
        suns: &[Pose],
        pipeline: &MeshShader<Instance>,
        air: &MaterialBinding,
    ) -> Result<Vec<MaterialMesh<Instance>>> {
        let mut records = Vec::new();
        let mut instances = Vec::new();
        for surface in surfaces {
            let surface = surface?;
            let material = surface.material;
            material.validate()?;
            let mut light_index = 0;
            if !suns.is_empty()
                && (material.reflection != [0.0; 3] || material.transmission != [0.0; 3])
            {
                let inverse = surface.local_to_parent.inverse()?;
                let mut record = [[0.0; 4]; MAX_SUNS];
                for (plane, sun) in record.iter_mut().zip(suns) {
                    let [w, x, y, z] = inverse.chain(sun)?.apply_vector([0.0, 0.0, 1.0, 0.0]);
                    *plane = [x as f32, y as f32, z as f32, w as f32];
                    ensure!(
                        plane.iter().all(|v| v.is_finite()),
                        "Sun plane exceeds shader coordinate range"
                    );
                }
                light_index = records.len();
                records.push(record);
            }
            let [real, imag] = projective_pair(surface.relative)?;
            let attributes = Instance {
                real,
                imag,
                lighting: Vec2::new(
                    light_index as f32,
                    f32::from(surface.domain == LightingDomain::Terrestrial),
                ),
                reflection: Vec3::from_array(material.reflection),
                transmission: Vec3::from_array(material.transmission),
            };
            instances.push((
                surface.polygon,
                Vec3::from_array(material.emission),
                attributes,
            ));
        }
        let lights = if records.is_empty() {
            self.light.unlit.clone()
        } else {
            SurfaceLights::bind(
                self.air.device(),
                &self.light.layout,
                &self.light.parameters,
                &records,
            )?
        };
        instances
            .into_iter()
            .map(|(polygon, emission, attributes)| {
                let material = pipeline.material(vec![lights.clone(), air.clone()], attributes)?;
                Ok(polygon.fill_color(emission).with_material(&material))
            })
            .collect()
    }
}
