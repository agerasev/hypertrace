use crate::Background;
use ccgeom::{Euclidean3, Geometry};
use vecmat::Vector;

/// Constant color background.
#[derive(Clone, Debug)]
pub struct ConstBg {
    pub color: Vector<f32, 3>,
}

impl ConstBg {
    pub fn new(color: Vector<f32, 3>) -> Self {
        Self { color }
    }
}

impl<G: Geometry> Background<G> for ConstBg {
    fn wgsl_background(&self) -> crate::wgsl::Result<crate::wgsl::Background> {
        Ok(crate::wgsl::Background::Constant(self.color.into_array()))
    }
}

/// Gradient background.
/// Available only for euclidean space because only that space preserves direction.
#[derive(Clone, Debug)]
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

impl Background<Euclidean3> for GradBg {
    fn wgsl_background(&self) -> crate::wgsl::Result<crate::wgsl::Background> {
        let mut axis = [0.0; 3];
        for (dst, src) in axis.iter_mut().zip(self.direction.into_array()) {
            *dst = crate::wgsl::finite_f32(src)?;
        }
        Ok(crate::wgsl::Background::Gradient {
            colors: self.colors.map(Vector::into_array),
            axis,
            power: self.power,
        })
    }
}
