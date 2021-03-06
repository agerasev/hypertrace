use crate::Material;
use type_macros::*;
use types::{prelude::*, source::SourceTree, Config};

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct Refractive {
    pub index: f64,
}

impl Default for Refractive {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Refractive {
    pub fn new(index: f64) -> Self {
        Self { index }
    }
}

impl EntitySource for Refractive {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("material/refractive.hh")
    }
}

impl Material for Refractive {}
