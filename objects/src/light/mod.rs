//! Component-owned directional proposals for explicitly sampled emitters.
use crate::shader::{Geometry, LightModule, LightValue, Result};

/// Propose a direction and evaluate its density in the same local tangent frame.
///
/// A sampler retains geometry through compilation, just like a shape:
/// ```compile_fail
/// use ccgeom::{Flat3, Spherical3};
/// use hypertrace_objects::shader::{LightModule, LightValue};
/// fn wrong_geometry(module: LightModule<Flat3>) {
///     let _ = LightValue::<Spherical3>::new(module, vec![]);
/// }
/// ```
pub trait LightSampler<G: Geometry>: Sized {
    fn shader() -> Result<LightModule<G>>;
    fn encode(&self) -> Result<LightValue<G>>;
}

/// An origin-centered geodesic bounding sphere, with a physical radius.
/// Map the sampled object as a whole to carry the bound with it. Directions that
/// miss the actual emitter contribute zero; they are never resampled.
#[derive(Clone, Copy, Debug)]
pub struct SphereBound {
    pub radius: f64,
}
impl SphereBound {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}
impl<G: Geometry> LightSampler<G> for SphereBound {
    fn shader() -> Result<LightModule<G>> {
        let mut module = LightModule::new(
            "hypertrace.light.sphere-bound",
            include_str!("sphere.wgsl"),
            Some(1),
        );
        module.validate_words = |_, ctx, words| {
            let radius = f32::from_bits(words[0]);
            anyhow::ensure!(
                radius.is_normal() && radius > 0.0,
                "light bound radius must be positive, finite and normal"
            );
            if G::SIGN != 0 {
                let angle = radius / ctx.radius;
                anyhow::ensure!(
                    angle.is_normal() && angle > 0.0,
                    "light bound radius/curvature radius must be positive, finite and normal"
                );
                if G::SIGN > 0 {
                    anyhow::ensure!(
                        angle <= std::f32::consts::PI,
                        "spherical light bound exceeds pi times the curvature radius"
                    );
                } else {
                    anyhow::ensure!(
                        angle.sinh().is_finite(),
                        "hyperbolic light bound is outside finite f32 range"
                    );
                }
            }
            Ok(())
        };
        Ok(module)
    }
    fn encode(&self) -> Result<LightValue<G>> {
        LightValue::new(
            <Self as LightSampler<G>>::shader()?,
            vec![crate::shader::finite_f32(self.radius)?.to_bits()],
        )
    }
}
