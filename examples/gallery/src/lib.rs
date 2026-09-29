//! Optional gallery and command-line tools for the standalone demonstrations.
//!
//! Each example owns its scene and application entry point in `examples/src/bin/<name>/`.
//! This gallery imports their scene files to render the same content. Example
//! binaries use the workspace libraries directly and never depend on this crate.

#[path = "../../src/bin/compare-eu/scene.rs"]
pub mod compare_eu;
#[path = "../../src/bin/compare-hy/scene.rs"]
pub mod compare_hy;
#[path = "../../src/bin/compare-hy-flat/scene.rs"]
pub mod compare_hy_flat;
#[path = "../../src/bin/compare-sp/scene.rs"]
pub mod compare_sp;
#[path = "../../src/bin/compare-sp-flat/scene.rs"]
pub mod compare_sp_flat;
#[path = "../../src/bin/eu/scene.rs"]
pub mod eu;
#[path = "../../src/bin/hy/scene.rs"]
pub mod hy;
#[path = "../../src/bin/sp/scene.rs"]
pub mod sp;
#[path = "../../src/bin/sp-fog/scene.rs"]
pub mod sp_fog;
#[path = "../../src/bin/sp-loop/scene.rs"]
pub mod sp_loop;
#[path = "../../src/bin/sp-loop-fog/scene.rs"]
pub mod sp_loop_fog;

mod catalog;
pub use catalog::{EXAMPLES, Example, factories, find};

/// Native/WebGPU viewer for the optional gallery application.
#[cfg(feature = "viewer")]
pub mod viewer;
