//! Shared example scenes constructed through Hypertrace's generic Rust API.
//!
//! These factories only build scene data. They require neither a graphics
//! device nor an OpenCL or SDL runtime, and can be consumed by either backend.
//! The const parameter selects the maximum number of light bounces.
//!
//! ```
//! use hypertrace_scenes::{eu, hy};
//!
//! let euclidean: eu::ExampleScene<4> = eu::scene();
//! let hyperbolic: hy::ExampleScene<3> = hy::scene();
//! ```

/// Euclidean shapes with diffuse, specular and refractive materials.
pub mod eu;
/// Hyperbolic planes and horospheres with their original tilings and materials.
pub mod hy;
