//! Camera-relative data shared by the sky and every opaque surface.
use anyhow::{Result, ensure};
use wgame::{
    glam::{Vec3, Vec4},
    shapes::material::MaterialBinding,
};
use wgpu::util::DeviceExt;

use crate::{projection::Pose, world::Sun};

#[derive(Clone, Copy)]
pub(crate) struct Atmosphere {
    pub(crate) ground: Pose,
    /// Altitude and RGB scattering coefficients in curvature-radius units.
    pub(crate) height: f64,
    pub(crate) scattering: [f32; 3],
    pub(crate) background: [f32; 3],
}

pub(crate) struct AirBindings {
    pub(crate) layout: wgpu::BindGroupLayout,
    device: wgpu::Device,
    atmosphere: Option<Atmosphere>,
    sun_pose: Pose,
    scattering: [f32; 3],
}

pub(crate) type AirFrame = [[f32; 4]; 3];
pub(crate) type FrameUniform = [[f32; 4]; 5];

pub(crate) fn frame_uniform(air: AirFrame, [real, imag]: [Vec4; 2]) -> FrameUniform {
    let [earth, sun, scattering] = air;
    [earth, sun, scattering, real.to_array(), imag.to_array()]
}

impl AirBindings {
    pub(crate) fn new(
        device: &wgpu::Device,
        atmosphere: Option<Atmosphere>,
        sun: &Sun,
    ) -> Result<Self> {
        let mut scattering = [0.0; 3];
        if let Some(atmosphere) = atmosphere {
            let normal = atmosphere
                .ground
                .inverse()?
                .chain(&sun.pose)?
                .apply_vector([0.0, 0.0, 1.0, 0.0]);
            ensure!(
                atmosphere.height > 0.0
                    && (atmosphere.height.sinh() as f32).is_finite()
                    && normal[2].abs() > atmosphere.height.cosh()
                    && normal[2].is_sign_positive() == normal[0].is_sign_positive(),
                "the Sun must lie above the entire atmospheric layer"
            );
            for (i, value) in scattering.iter_mut().enumerate() {
                *value = sun.radiance[i] * atmosphere.scattering[i];
                ensure!(
                    atmosphere.scattering[i] >= 0.0 && value.is_finite(),
                    "invalid atmospheric scattering coefficient"
                );
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
            sun_pose: sun.pose,
            scattering,
        })
    }

    pub(crate) fn sky_background(&self) -> Option<Vec3> {
        self.atmosphere.map(|air| Vec3::from_array(air.background))
    }

    pub(crate) fn words(&self, view: Pose) -> Result<AirFrame> {
        let mut words = [[0.0_f32; 4]; 3];
        if let Some(atmosphere) = self.atmosphere {
            for (out, pose) in words[..2]
                .iter_mut()
                .zip([atmosphere.ground, self.sun_pose])
            {
                let [w, x, y, z] = view.chain(&pose)?.apply_vector([0.0, 0.0, 1.0, 0.0]);
                let plane = Vec4::new(x as f32, y as f32, z as f32, -w as f32);
                ensure!(
                    plane.is_finite(),
                    "atmospheric plane exceeds shader coordinate range"
                );
                *out = plane.to_array();
            }
            let [r, g, b] = self.scattering;
            words[2] = [r, g, b, atmosphere.height.sinh() as f32];
        }
        Ok(words)
    }

    pub(crate) fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub(crate) fn binding(&self, air: &AirFrame) -> MaterialBinding {
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera-relative atmosphere"),
                contents: bytemuck::cast_slice(&frame_uniform(*air, [Vec4::X, Vec4::ZERO])),
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
