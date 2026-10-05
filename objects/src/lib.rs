pub mod background;
mod mapped;
pub mod material;
pub mod object;
mod parameters;
pub mod scene;
pub mod shader;
pub mod shape;
pub mod view;

pub use background::Background;
pub use mapped::Mapped;
pub use material::Material;
pub mod light;
pub use object::Object;
pub use scene::{Scene, SceneImpl};
pub use shape::Shape;
pub use view::View;

/// Statically selected constant-curvature geometry used by scene builders.
pub use shader::Geometry;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod construction_tests;
