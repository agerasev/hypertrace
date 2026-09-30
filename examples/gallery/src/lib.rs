//! Optional gallery and command-line tools for the standalone demonstrations.
//!
//! Each example owns its scene and application entry point in `examples/src/bin/<name>/`.
//! This gallery imports their scene files to render the same content. Example
//! binaries use the workspace libraries directly and never depend on this crate.

#[path = "../../src/bin/ball-tilings/scene.rs"]
pub mod ball_tilings;
#[path = "../../src/bin/euclidean/scene.rs"]
pub mod euclidean;
#[path = "../../src/bin/fog/scene.rs"]
pub mod fog;
#[path = "../../src/bin/hyperbolic/scene.rs"]
pub mod hyperbolic;
#[path = "../../src/bin/spherical/scene.rs"]
pub mod spherical;

mod catalog;
pub use catalog::{EXAMPLES, Example, factories, find};

/// Native/WebGPU viewer for the optional gallery application.
#[cfg(feature = "viewer")]
pub mod viewer;
