//! Native WGPU path tracing with GPU presentation and explicit snapshots.
//!
//! Generic Rust builders lower through a CPU-only scene compiler to generated
//! WGSL and explicit storage records. Camera transforms remain f64 on the CPU
//! and are converted to f32 only when uploaded.

#![forbid(unsafe_code)]

mod gpu;
pub use gpu::{Gpu, read_buffer};
pub mod scene;
pub use scene::{Background, Camera, Material, Object, Scene};
mod renderer;
pub use renderer::{Renderer, pixel_seed, shader_source};
mod resolution;
pub use resolution::fit_render_size;
pub mod presentation;
pub use presentation::Presenter;

pub type Result<T> = anyhow::Result<T>;

/// CPU scene descriptions, extension leaf ABI and WGSL compiler.
pub use scene_ir as wgsl;
