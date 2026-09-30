//! Optional gallery and command-line tools for the standalone demonstrations.
//!
//! Each example owns its scene and application entry point in `examples/src/bin/<name>/`.
//! This gallery imports their scene files to render the same content. Example
//! binaries use the workspace libraries directly and never depend on this crate.

#[path = "../../src/bin/eu/scene.rs"]
pub mod eu;
#[path = "../../src/bin/eu-fog/scene.rs"]
pub mod eu_fog;
#[path = "../../src/bin/hy/scene.rs"]
pub mod hy;
#[path = "../../src/bin/sp/scene.rs"]
pub mod sp;

mod catalog;
pub use catalog::{EXAMPLES, Example, factories, find};

/// Native/WebGPU viewer for the optional gallery application.
#[cfg(feature = "viewer")]
pub mod viewer;
