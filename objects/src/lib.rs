pub mod background;
mod macros;
mod mapped;
pub mod material;
pub mod object;
pub mod scene;
pub mod shader;
pub mod shape;
pub mod view;

pub use background::Background;
pub use mapped::Mapped;
pub use material::Material;
pub use object::Object;
pub use scene::{Scene, SceneImpl};
pub use shape::Shape;
pub use view::View;

/// Geometry contract used by scene builders and choice macros.
pub use ccgeom::Geometry;

#[cfg(test)]
mod tests;
