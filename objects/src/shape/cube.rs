use super::*;

/// An origin-centered Euclidean cube with a physical half-extent on each axis.
/// Unsupported geometries are rejected by the type system.
/// ```compile_fail
/// use ccgeom::Spherical3;
/// use hypertrace_objects::{Shape, shape::Cube};
/// let _ = <Cube as Shape<Spherical3>>::shader();
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Cube {
    pub half_extent: f64,
}

impl Cube {
    pub fn new(half_extent: f64) -> Self {
        Self { half_extent }
    }
}

pub fn cube_schema() -> ShapeModule<ccgeom::Flat3> {
    let mut module = ShapeModule::new(
        "hypertrace.shape.cube",
        include_str!("shaders/cube.wgsl"),
        Some(1),
    );
    module.validate_words = |_, _, words| {
        let extent = f32::from_bits(words[0]);
        anyhow::ensure!(
            extent.is_normal() && extent > 0.0,
            "cube half-extent must be finite, positive and normal in f32"
        );
        Ok(())
    };
    module
}

impl Shape<ccgeom::Flat3> for Cube {
    fn shader() -> Result<ShapeModule<ccgeom::Flat3>> {
        Ok(cube_schema())
    }
    fn encode(&self) -> Result<ShapeValue<ccgeom::Flat3>> {
        let extent = crate::shader::finite_f32(self.half_extent)?;
        ShapeValue::new(
            <Self as Shape<ccgeom::Flat3>>::shader()?,
            vec![extent.to_bits()],
        )
    }
}

pub fn cube(half_extent: f64) -> Result<ShapeValue<ccgeom::Flat3>> {
    <Cube as Shape<ccgeom::Flat3>>::encode(&Cube::new(half_extent))
}
