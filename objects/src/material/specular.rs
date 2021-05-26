use crate::Material;
use type_macros::*;
use types::{prelude::*, source::SourceTree, Config};

#[derive(Clone, Copy, Debug, EntityId, Entity, SizedEntity)]
pub struct Specular;

impl EntitySource for Specular {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("material/specular.hh")
    }
}

impl Material for Specular {}
