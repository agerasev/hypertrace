use crate::Material;
use type_macros::*;
use types::{prelude::*, source::SourceTree, Config};

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct Lambertian;

impl EntitySource for Lambertian {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("material/lambertian.hh")
    }
}

impl Material for Lambertian {}
