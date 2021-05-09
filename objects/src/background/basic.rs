use types::{Named, Sourced, Config, source::{SourceTree, SourceBuilder, include}, include_template};
use std::{marker::PhantomData};
use type_macros::{Named, SizedEntity};
use vecmat::Vector;
use ccgeom::{Geometry3, Euclidean3};
use crate::Background;


/// Constant color background.
#[derive(Clone, Debug, SizedEntity)]
pub struct ConstBg<G: Geometry3 + Named> {
    #[skip_entity] geometry: PhantomData<G>,
    pub color: Vector<f32, 3>,
}

impl<G: Geometry3 + Named> ConstBg<G> {
    pub fn new(color: Vector<f32, 3>) -> Self {
        Self { color, geometry: PhantomData }
    }
}

impl<G: Geometry3 + Named> Named for ConstBg<G> {
    fn type_name(cfg: &Config) -> String {
        format!("ConstBg{}", G::type_name(cfg))
    }
    fn type_prefix(cfg: &Config) -> String {
        format!("const_bg_{}", G::type_prefix(cfg))
    }
}

impl<G: Geometry3 + Sourced> Sourced for ConstBg<G> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/const_bg_{}.hh", &G::type_prefix(cfg)))
            .tree(G::source(cfg))
            .content(&include(&format!("render/light/{}.hh", &G::type_prefix(cfg))))
            .content(&include_template!(
                "background/constant.inl",
                "Geo": &G::type_name(cfg),
                "geo": &G::type_prefix(cfg),
            ))
            .build()
    }
}

impl<G: Geometry3 + Sourced> Background<G> for ConstBg<G> {}


/// Gradient background.
/// Available only for euclidean space because only that space preserves direction.
#[derive(Clone, Debug, Named, SizedEntity)]
pub struct GradBg {
    pub direction: Vector<f64, 3>,
    pub colors: [Vector<f32, 3>; 2],
}

impl GradBg {
    pub fn new(
        direction: Vector<f64, 3>,
        colors: [Vector<f32, 3>; 2],
    ) -> Self {
        Self { direction, colors }
    }
}

impl Sourced for GradBg {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("background/gradient.hh")
    }
}

impl Background<Euclidean3> for GradBg {}
