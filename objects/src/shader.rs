//! Typed scene construction and component-owned shader modules.
//!
//! Geometry and shader calling conventions remain Rust type parameters through
//! compilation. Map adapters return a canonical isometry of that same geometry.
//!
//! ```
//! use ccgeom::Flat3;
//! use hypertrace_objects::{background::ConstBg, material::Lambertian,
//!     object::Covered, shape::Sphere, view::PointView, Scene, SceneImpl};
//! # fn main() -> hypertrace_objects::shader::Result<()> {
//! let scene = SceneImpl::<Flat3, _, _, _, 4>::new(
//!     PointView::new(1.0), vec![Covered::new(Sphere, Lambertian)],
//!     ConstBg::new([0.1, 0.2, 0.3].into()));
//! assert_eq!(scene.definition()?.bounces, 4);
//! # Ok(())
//! # }
//! ```

pub use ::scene::*;
use ccgeom::{Embedded3, EmbeddedIsometry};

/// Encode a construction map as a checked canonical f64 isometry.
pub trait RenderMap<G: Geometry>: ccgeom::Map<G::Pos, G::Dir> {
    fn render_transform(&self) -> Result<Transform<G>>;
}

impl<const K: i8> RenderMap<Embedded3<f64, K>> for EmbeddedIsometry<f64, K>
where
    Embedded3<f64, K>: Geometry<Map = EmbeddedIsometry<f64, K>>,
{
    fn render_transform(&self) -> Result<Transform<Embedded3<f64, K>>> {
        Transform::from_isometry(*self)
    }
}

pub fn transform<G: Geometry, M: RenderMap<G>>(map: &M) -> Result<Transform<G>> {
    map.render_transform()
}
