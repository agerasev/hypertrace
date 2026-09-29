//! Fullscreen presentation from the renderer's GPU accumulation buffer.
//!
//! No image readback or CPU conversion occurs here. Encode rendering before
//! [`Presenter::draw`] on the same queue, and call [`Presenter::rebind`] whenever
//! resizing the renderer replaces its buffers.

use std::borrow::Cow;

use crate::{Renderer, shader::Geometry};

/// Draws one renderer into a single-sample attachment of any positive size.
/// The image fills the attachment using nearest-neighbor scaling, preserving
/// its top-left orientation. Equal dimensions display each source pixel once.
///
/// The display transform is a gamma of 1/2.2. An sRGB attachment receives
/// the inverse sRGB transfer first, so its automatic encoding applies that
/// display transform exactly once. Output is opaque.
pub struct Presenter {
    layout: wgpu::BindGroupLayout,
    bindings: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl Presenter {
    pub fn new<G: Geometry>(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        renderer: &Renderer<G>,
    ) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("hypertrace presentation layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(128),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(16),
                    },
                    count: None,
                },
            ],
        });
        let bindings = Self::bindings(device, &layout, renderer);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("hypertrace presentation shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "shaders/presentation.wgsl"
            ))),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("hypertrace presentation pipeline layout"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("hypertrace presentation pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("fullscreen"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("display"),
                compilation_options: wgpu::PipelineCompilationOptions {
                    constants: &[("ATTACHMENT_SRGB", f64::from(format.is_srgb()))],
                    ..Default::default()
                },
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        Self {
            layout,
            bindings,
            pipeline,
        }
    }

    fn bindings<G: Geometry>(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        renderer: &Renderer<G>,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("hypertrace presentation bindings"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: renderer.params_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: renderer.accumulation_buffer().as_entire_binding(),
                },
            ],
        })
    }

    /// Refresh references after renderer resize; no pixels move through the CPU.
    pub fn rebind<G: Geometry>(&mut self, device: &wgpu::Device, renderer: &Renderer<G>) {
        self.bindings = Self::bindings(device, &self.layout, renderer);
    }

    /// Encode a fullscreen pass into a view of a positive-size mip-zero texture.
    /// The attachment must use the format supplied to [`Self::new`]. Its size
    /// may differ from the renderer; nearest-neighbor scaling fills the target.
    pub fn draw(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("hypertrace presentation"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bindings, &[]);
        pass.draw(0..3, 0..1);
    }
}
