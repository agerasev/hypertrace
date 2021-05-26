use crate::Material;
use type_macros::*;
use types::{prelude::*, source::SourceTree, Config};

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct Transparent;

impl EntitySource for Transparent {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("material/transparent.hh")
    }
}

impl Material for Transparent {}
