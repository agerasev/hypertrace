use crate::Material;
use type_macros::*;
use types::{prelude::*, source::SourceTree, Config};

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct Absorbing;

impl EntitySource for Absorbing {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("material/basic/absorbing.hh")
    }
}

impl Material for Absorbing {}
