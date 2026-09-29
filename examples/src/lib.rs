//! Shared example scenes constructed through Hypertrace's generic Rust API.
//!
//! The factories build scene data without a graphics device. With the `viewer`
//! feature, a shared application host runs any independently supplied factory.
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
/// Hyperbolic planes and horospheres with pentagonal, square, and hexagonal tilings.
pub mod hy;
/// A light behind the camera, visible by travelling around spherical space.
pub mod recurrence;
/// Spherical planes and geodesic spheres lit by emissive objects.
pub mod sp;

mod catalog;
pub use catalog::{EXAMPLES, Example, factories, find};

/// Native/WebGPU application host for independent example factories.
#[cfg(feature = "viewer")]
pub mod viewer;
