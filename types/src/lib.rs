#[macro_use]
mod macros;

mod map;
#[cfg(test)]
mod tests;

pub mod config;
pub mod containers;
pub mod entity;
pub mod geometry;
pub mod hash;
pub mod io;
pub mod math;
pub mod primitive;
mod ray;
pub mod source;

pub use config::Config;
pub use containers::*;
pub use entity::*;
pub use geometry::Geometry;
pub use map::Map;
pub use primitive::*;
pub use ray::*;

pub mod prelude {
    pub use crate::{
        source::{SourceBuilder, SourceTree},
        Config, Entity, EntityId, EntitySource, Geometry, SizedEntity,
    };
}
