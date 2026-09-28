use super::*;

/// A sphere of intrinsic radius one, centered at the geometry's origin.
#[derive(Clone, Default, Debug)]
pub struct Sphere;

impl<G: Geometry> Shape<G> for Sphere {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        crate::wgsl::geometry::<G>()?;
        Ok(crate::wgsl::ShapeSchema::Sphere)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        crate::wgsl::geometry::<G>()?;
        Ok(crate::wgsl::ShapeValue::sphere())
    }
}

/// A sphere centered at the origin with an intrinsic radius in physical units.
///
/// Radius must be finite and positive. Spherical space additionally requires
/// `radius < pi * scene.radius`; the scene compiler validates this relationship.
#[derive(Clone, Copy, Debug)]
pub struct GeodesicSphere {
    pub radius: f64,
}

impl GeodesicSphere {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl<G: Geometry> Shape<G> for GeodesicSphere {
    fn wgsl_shape_schema() -> crate::wgsl::Result<crate::wgsl::ShapeSchema> {
        crate::wgsl::geometry::<G>()?;
        Ok(crate::wgsl::ShapeSchema::GeodesicSphere)
    }

    fn wgsl_shape(&self) -> crate::wgsl::Result<crate::wgsl::ShapeValue> {
        crate::wgsl::geometry::<G>()?;
        crate::wgsl::ShapeValue::geodesic_sphere(self.radius)
    }
}
