#[cfg(test)]
mod macros_tests;
pub mod shape;
pub mod material;
pub mod object;
pub mod background;
pub mod view;
pub mod scene;

pub use shape::Shape;
pub use material::Material;
pub use object::Object;
pub use background::Background;
pub use view::View;
pub use scene::{Scene, SceneImpl};
