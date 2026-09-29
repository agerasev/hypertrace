//! Shared example scenes constructed through Hypertrace's generic Rust API.
//!
//! These factories only build scene data. They require neither a graphics
//! device nor a windowing runtime and lower to the portable scene representation.
//! The const parameter selects the maximum number of light bounces.
//!
//! ```
//! use hypertrace_examples::{eu, hy, sp};
//!
//! let euclidean: eu::ExampleScene<4> = eu::scene();
//! let hyperbolic: hy::ExampleScene<3> = hy::scene();
//! let spherical: sp::ExampleScene<6> = sp::scene();
//! ```

/// Identical physical layouts in each curvature, with adjustable curvature radius.
pub mod comparison;
/// Euclidean shapes with diffuse, specular and refractive materials.
pub mod eu;
/// Hyperbolic planes and horospheres with their original tilings and materials.
pub mod hy;
/// A light behind the camera, visible by travelling around spherical space.
pub mod recurrence;
/// Spherical planes and geodesic spheres lit by emissive objects.
pub mod sp;

mod catalog;
pub use catalog::{find, Example, EXAMPLES};
