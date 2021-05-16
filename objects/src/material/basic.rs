use crate::Material;
use type_macros::*;
use types::{prelude::*, source::SourceTree, Config};

#[derive(Clone, Copy, Debug, Named, Entity, SizedEntity)]
pub struct Absorbing;

impl Sourced for Absorbing {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("material/basic.hh")
    }
}

impl Material for Absorbing {}

#[derive(Clone, Copy, Debug, Named, Entity, SizedEntity)]
pub struct Transparent;

impl Sourced for Transparent {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("material/basic.hh")
    }
}

impl Material for Transparent {}

#[derive(Clone, Copy, Debug, Named, Entity, SizedEntity)]
pub struct Specular;

impl Sourced for Specular {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("material/basic.hh")
    }
}

impl Material for Specular {}

#[derive(Clone, Copy, Debug, Named, Entity, SizedEntity)]
pub struct Lambertian;

impl Sourced for Lambertian {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("material/basic.hh")
    }
}

impl Material for Lambertian {}
