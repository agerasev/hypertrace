use crate::Background;
use ccgeom::Euclidean3;
use type_macros::*;
use types::{
    include_template,
    prelude::*,
    source::{include, SourceBuilder, SourceTree},
    Config,
};
use vecmat::Vector;

/// Constant color background.
#[derive(Clone, Debug, Named, Entity, SizedEntity)]
pub struct ConstBg {
    pub color: Vector<f32, 3>,
}

impl ConstBg {
    pub fn new(color: Vector<f32, 3>) -> Self {
        Self { color }
    }
}

impl Sourced for ConstBg {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("background/constant.hh")
    }
}

impl<G: Geometry> Background<G> for ConstBg {
    fn background_prefix(cfg: &Config) -> String {
        format!("const_bg_{}", G::geometry_prefix(cfg))
    }
    fn background_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!(
            "generated/const_bg_{}.hh",
            &G::geometry_prefix(cfg)
        ))
        .tree(G::geometry_source(cfg))
        .content(&include(&format!(
            "render/light/{}.hh",
            &G::geometry_prefix(cfg)
        )))
        .content(&include_template!(
            "background/constant.inl",
            "Geo": &G::type_name(cfg),
            "geo": &G::geometry_prefix(cfg),
        ))
        .build()
    }
}

/// Gradient background.
/// Available only for euclidean space because only that space preserves direction.
#[derive(Clone, Debug, Named, Entity, SizedEntity)]
pub struct GradBg {
    pub direction: Vector<f64, 3>,
    pub colors: [Vector<f32, 3>; 2],
    pub power: f32,
}

impl GradBg {
    pub fn new(direction: Vector<f64, 3>, colors: [Vector<f32, 3>; 2], power: f32) -> Self {
        Self {
            direction,
            colors,
            power,
        }
    }
}

impl Sourced for GradBg {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("background/gradient.hh")
    }
}

impl Background<Euclidean3> for GradBg {}
