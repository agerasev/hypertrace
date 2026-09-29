use super::*;

/// A sphere of intrinsic radius one, centered at the geometry's origin.
#[derive(Clone, Default, Debug)]
pub struct Sphere;

fn intersection_module<G: Geometry>() -> crate::shader::LibraryModule<G> {
    crate::shader::LibraryModule::new(
        "hypertrace.shape.sphere.intersection",
        include_str!("shaders/sphere.wgsl"),
        None,
    )
}

pub fn sphere_schema<G: Geometry>() -> ShapeModule<G> {
    let mut module = ShapeModule::new(
        "hypertrace.shape.sphere",
        include_str!("shaders/unit_sphere.wgsl"),
        Some(1),
    );
    module
        .dependencies
        .push(intersection_module::<G>().into_source());
    module.validate_context = |ctx| validate_sphere_radius::<G>(1.0, ctx.radius);
    module
}

impl<G: Geometry> Shape<G> for Sphere {
    fn shader() -> Result<ShapeModule<G>> {
        Ok(sphere_schema())
    }
    fn encode(&self) -> Result<ShapeValue<G>> {
        ShapeValue::new(<Self as Shape<G>>::shader()?, vec![0])
    }
}

/// An origin-centered sphere with an intrinsic radius in physical units.
#[derive(Clone, Copy, Debug)]
pub struct GeodesicSphere {
    pub radius: f64,
}
impl GeodesicSphere {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

pub fn geodesic_sphere_schema<G: Geometry>() -> ShapeModule<G> {
    let mut module = ShapeModule::new(
        "hypertrace.shape.geodesic_sphere",
        include_str!("shaders/geodesic_sphere.wgsl"),
        Some(1),
    );
    module
        .dependencies
        .push(intersection_module::<G>().into_source());
    module.validate_words =
        |_, ctx, words| validate_sphere_radius::<G>(f32::from_bits(words[0]), ctx.radius);
    module
}
impl<G: Geometry> Shape<G> for GeodesicSphere {
    fn shader() -> Result<ShapeModule<G>> {
        Ok(geodesic_sphere_schema())
    }
    fn encode(&self) -> Result<ShapeValue<G>> {
        let radius = crate::shader::finite_f32(self.radius)?;
        anyhow::ensure!(radius > 0.0, "sphere radius must be positive");
        ShapeValue::new(<Self as Shape<G>>::shader()?, vec![radius.to_bits()])
    }
}
pub fn sphere<G: Geometry>() -> ShapeValue<G> {
    <Sphere as Shape<G>>::encode(&Sphere).expect("valid unit sphere")
}
pub fn geodesic_sphere<G: Geometry>(radius: f64) -> Result<ShapeValue<G>> {
    <GeodesicSphere as Shape<G>>::encode(&GeodesicSphere::new(radius))
}

fn validate_sphere_radius<G: Geometry>(radius: f32, space_radius: f32) -> Result<()> {
    anyhow::ensure!(
        radius.is_normal() && radius > 0.0,
        "geodesic sphere needs a finite positive normal radius"
    );
    let angle = radius / space_radius;
    // Match geo_classify_section_discriminant's coefficient uncertainty at
    // the sphere center; do not admit a radius already inside that zero band.
    let resolved =
        |sine: f32, cosine: f32| sine * sine > 8.0 * f32::EPSILON * (1.0 + cosine * cosine);
    match G::SIGN {
        1 => {
            anyhow::ensure!(
                radius < std::f32::consts::PI * space_radius,
                "spherical sphere radius must be strictly below pi times the curvature radius"
            );
            let (sine, cosine) = angle.sin_cos();
            anyhow::ensure!(
                angle.is_finite()
                    && cosine.abs() < 1.0
                    && sine.is_normal()
                    && sine > 0.0
                    && resolved(sine, cosine),
                "spherical sphere radius is outside the f32 section resolver range: cos(radius/space_radius) must be distinct from both 1 and -1 and resolve the coefficient uncertainty band"
            );
        }
        -1 => {
            let cosine = angle.cosh();
            let sine = angle.sinh();
            anyhow::ensure!(
                angle.is_finite()
                    && (cosine * cosine).is_finite()
                    && cosine > 1.0
                    && sine.is_normal()
                    && sine > 0.0
                    && resolved(sine, cosine),
                "hyperbolic sphere radius is outside the f32 section resolver range: cosh(radius/space_radius) must be greater than one with a finite square and resolve the coefficient uncertainty band"
            );
        }
        0 => anyhow::ensure!(
            (radius * radius).is_finite(),
            "Euclidean sphere squared radius is outside f32 range"
        ),
        _ => unreachable!("Geometry only admits the three curvature signs"),
    }
    Ok(())
}
