use crate::View;
use std::marker::PhantomData;
use type_macros::{Entity, SizedEntity};
use types::{
    prelude::*,
    source::{SourceBuilder, SourceTree},
    Config,
};

#[derive(Clone, Debug, Entity, SizedEntity)]
pub struct PointView<G: Geometry> {
    pub fov: f64,
    phantom: PhantomData<G>,
}

impl<G: Geometry> PointView<G> {
    pub fn new(fov: f64) -> Self {
        Self {
            fov,
            phantom: PhantomData,
        }
    }
}

impl<G: Geometry> Named for PointView<G> {
    fn type_name(cfg: &Config) -> String {
        format!("PointView{}", G::type_name(cfg))
    }
    fn type_prefix(cfg: &Config) -> String {
        format!("point_view_{}", G::type_prefix(cfg))
    }
}

impl<G: Geometry> Sourced for PointView<G> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("view/point/{}.hh", &G::type_prefix(cfg)))
            .tree(G::source(cfg))
            .build()
    }
}

impl<G: Geometry> View<G> for PointView<G> {}
