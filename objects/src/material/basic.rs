use crate::Material;
use type_macros::*;
use types::{prelude::*, source::SourceTree, Config};

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct Absorbing;

impl EntitySource for Absorbing {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("material/basic.hh")
    }
}

impl Material for Absorbing {}

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct Transparent;

impl EntitySource for Transparent {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("material/basic.hh")
    }
}

impl Material for Transparent {}

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct Specular;

impl EntitySource for Specular {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("material/basic.hh")
    }
}

impl Material for Specular {}

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct Lambertian;

impl EntitySource for Lambertian {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("material/basic.hh")
    }
}

impl Material for Lambertian {}
