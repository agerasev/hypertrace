use crate::shader::Geometry;
use crate::Background;
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
    fn background(&self) -> crate::shader::Result<crate::shader::Background<G>> {
        Ok(crate::shader::Background::constant(self.color.into_array()))
    }
}

/// Gradient background.
/// Available only for euclidean space because only that space preserves direction.
/// ```compile_fail
/// use ccgeom::Hyperboloid3;
/// use hypertrace_objects::{Background, background::GradBg};
/// let gradient = GradBg::new([0.0, 1.0, 0.0].into(), [[1.0; 3].into(), [0.0; 3].into()], 1.0);
/// let _ = <GradBg as Background<Hyperboloid3>>::background(&gradient);
/// ```
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

impl Background<ccgeom::Flat3> for GradBg {
    fn background(&self) -> crate::shader::Result<crate::shader::Background<ccgeom::Flat3>> {
        let mut axis = [0.0; 3];
        for (dst, src) in axis.iter_mut().zip(self.direction.into_array()) {
            *dst = crate::shader::finite_f32(src)?;
        }
        Ok(crate::shader::Background::gradient(
            self.colors.map(Vector::into_array),
            axis,
            self.power,
        ))
    }
}
