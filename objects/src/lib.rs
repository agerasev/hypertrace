pub mod background;
#[cfg(test)]
mod macros_tests;
pub mod material;
pub mod object;
pub mod scene;
pub mod shape;
pub mod view;

pub use background::Background;
pub use material::Material;
pub use object::Object;
pub use scene::{Scene, SceneImpl};
pub use shape::Shape;
pub use view::View;
