//! Camera-relative data shared by the sky and every opaque surface.
use anyhow::{Result, ensure};
use bytemuck::{Pod, Zeroable};
use wgame::{
    glam::{Vec3, Vec4},
    shapes::material::MaterialBinding,
};
use wgpu::util::DeviceExt;

use crate::{
    projection::Pose,
    world::{MAX_SUNS, Sun},
};

#[derive(Clone, Copy)]
pub(crate) struct Atmosphere {
    pub(crate) ground: Pose,
    /// Altitude in curvature-radius units.
    pub(crate) height: f64,
    pub(crate) background: [f32; 3],
}

pub(crate) struct AirBindings {
    pub(crate) layout: wgpu::BindGroupLayout,
    device: wgpu::Device,
    atmosphere: Option<Atmosphere>,
    /// Only Suns with a nonzero contribution to scattering are uploaded.
    sources: Vec<(Pose, Vec3)>,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct AirSun {
    plane: [f32; 4],
    scattering: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub(crate) struct AirFrame {
    earth: [f32; 4],
    sinh_height: f32,
    count: u32,
    padding: [u32; 2],
    suns: [AirSun; MAX_SUNS],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub(crate) struct FrameUniform {
    air: AirFrame,
    tile_real: [f32; 4],
    tile_imag: [f32; 4],
}

pub(crate) fn frame_uniform(air: AirFrame, [real, imag]: [Vec4; 2]) -> FrameUniform {
    FrameUniform {
        air,
        tile_real: real.to_array(),
        tile_imag: imag.to_array(),
    }
}

impl AirBindings {
    pub(crate) fn new(
        device: &wgpu::Device,
        atmosphere: Option<Atmosphere>,
        suns: &[Sun],
    ) -> Result<Self> {
        ensure!(
            suns.len() <= MAX_SUNS,
            "too many Suns for the atmospheric shader"
        );
        let mut sources = Vec::new();
        if let Some(atmosphere) = atmosphere {
            ensure!(
                atmosphere.height > 0.0 && (atmosphere.height.sinh() as f32).is_finite(),
                "invalid atmospheric layer height"
            );
            let world_to_ground = atmosphere.ground.inverse()?;
            for sun in suns {
                ensure!(
                    sun.scattering_radiance
                        .iter()
                        .all(|&v| v.is_finite() && v >= 0.0),
                    "invalid solar scattering radiance"
                );
                let scattering = Vec3::from_array(sun.scattering_radiance);
                if scattering == Vec3::ZERO {
                    continue;
                }
                let normal = world_to_ground
                    .chain(&sun.pose)?
                    .apply_vector([0.0, 0.0, 1.0, 0.0]);
                ensure!(
                    normal[2].abs() > atmosphere.height.cosh()
                        && normal[2].is_sign_positive() == normal[0].is_sign_positive(),
                    "each scattering Sun must lie above the entire atmospheric layer"
                );
                sources.push((sun.pose, scattering));
            }
        }
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera-relative frame"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(size_of::<FrameUniform>() as u64),
                },
                count: None,
            }],
        });
        Ok(Self {
            device: device.clone(),
            layout,
            atmosphere,
            sources,
        })
    }

    pub(crate) fn sky_background(&self) -> Option<Vec3> {
        self.atmosphere.map(|air| Vec3::from_array(air.background))
    }

    pub(crate) fn words(&self, view: Pose) -> Result<AirFrame> {
        let mut frame = AirFrame::zeroed();
        if let Some(atmosphere) = self.atmosphere {
            let plane = |pose| -> Result<[f32; 4]> {
                let [w, x, y, z] = view.chain(pose)?.apply_vector([0.0, 0.0, 1.0, 0.0]);
                let plane = Vec4::new(x as f32, y as f32, z as f32, -w as f32);
                ensure!(
                    plane.is_finite(),
                    "atmospheric plane exceeds shader coordinate range"
                );
                Ok(plane.to_array())
            };
            frame.earth = plane(&atmosphere.ground)?;
            frame.sinh_height = atmosphere.height.sinh() as f32;
            frame.count = self.sources.len() as u32;
            for (out, (pose, scattering)) in frame.suns.iter_mut().zip(&self.sources) {
                out.plane = plane(pose)?;
                out.scattering = scattering.extend(0.0).to_array();
            }
        }
        Ok(frame)
    }

    pub(crate) fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub(crate) fn binding(&self, air: &AirFrame) -> MaterialBinding {
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera-relative atmosphere"),
                contents: bytemuck::bytes_of(&frame_uniform(*air, [Vec4::X, Vec4::ZERO])),
                usage: wgpu::BufferUsages::UNIFORM,
            });
        self.bind_buffer(&buffer)
    }

    pub(crate) fn tile_binding(&self) -> (wgpu::Buffer, MaterialBinding) {
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera-relative ground tile"),
            size: size_of::<FrameUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let binding = self.bind_buffer(&buffer);
        (buffer, binding)
    }

    fn bind_buffer(&self, buffer: &wgpu::Buffer) -> MaterialBinding {
        MaterialBinding::Uniforms(self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera-relative frame"),
            layout: &self.layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        }))
    }
}
