//! Intrinsic surface charts and statically selected tile families.
//!
//! `TileSurface` connects a shape to its intrinsic coordinate domain; `Tiling`
//! implements a pattern within that domain. Neither needs renderer registration.
use crate::shader::{Geometry, LibraryModule, MaterialModule, MaterialValue, Result};
use crate::Shape;
use ccgeom::{Flat3, Hyperboloid3, Spherical3};
use std::{convert::TryFrom, fmt::Write};

/// Flat coordinates occupy the first two components, in physical world units.
#[derive(Clone, Copy, Debug)]
pub struct Euclidean;
/// Scalar-first unit hyperboloid coordinates; widths are curvature-normalized.
#[derive(Clone, Copy, Debug)]
pub struct Hyperbolic;
/// Unit three-vectors occupy the first three components; widths are radians.
#[derive(Clone, Copy, Debug)]
pub struct Spherical;

/// A shape's intrinsic material coordinates, in that object's local frame.
/// Mapping the whole object carries this frame with it. A downstream shape can
/// implement this trait and provide its own chart without changing the renderer.
///
/// A mapped shape alone deliberately has no implicit chart: its geometry moves
/// within the parent material frame. Map the complete `Tiled` object instead.
/// ```compile_fail
/// use ccgeom::{Flat3, Geometry3};
/// use hypertrace_objects::{Mapped, object::tiling::TileSurface, shape::Plane};
/// fn intrinsic<S: TileSurface<Flat3>>(_: S) {}
/// intrinsic(Mapped::<Flat3, _, _>::new(Plane, Flat3::shift_x(1.0)));
/// ```
pub trait TileSurface<G: Geometry>: Shape<G> {
    type Domain;
    /// Library entry `(position: vec4<f32>) -> vec4<f32>`, with no parameters.
    fn chart() -> LibraryModule<G>;
}

fn chart<G: Geometry>(key: &str, expression: &str) -> LibraryModule<G> {
    LibraryModule::new(
        key,
        format!("fn {{{{self}}}}(position:vec4<f32>)->vec4<f32> {{ return {expression}; }}"),
        Some(0),
    )
}
impl TileSurface<Flat3> for crate::shape::Plane {
    type Domain = Euclidean;
    fn chart() -> LibraryModule<Flat3> {
        chart(
            "hypertrace.chart.euclidean_plane",
            "vec4<f32>(position.yz,0,0)",
        )
    }
}
impl TileSurface<Hyperboloid3> for crate::shape::Plane {
    type Domain = Hyperbolic;
    fn chart() -> LibraryModule<Hyperboloid3> {
        chart("hypertrace.chart.hyperbolic_plane", "position")
    }
}
impl TileSurface<Spherical3> for crate::shape::Plane {
    type Domain = Spherical;
    fn chart() -> LibraryModule<Spherical3> {
        // Local z=0 cuts S3 in the unit S2 with coordinates (w,x,y).
        chart(
            "hypertrace.chart.spherical_plane",
            "vec4<f32>(normalize(position.xyz),0)",
        )
    }
}
impl TileSurface<Hyperboloid3> for crate::shape::Horosphere {
    type Domain = Euclidean;
    fn chart() -> LibraryModule<Hyperboloid3> {
        // The origin horosphere has half-space height one. Its induced metric
        // is Euclidean, scaled by the physical curvature radius.
        chart(
            "hypertrace.chart.horosphere",
            "vec4<f32>(geo_to_half_space_pos(position).xy*params.misc.y,0,0)",
        )
    }
}
macro_rules! spherical_surface {
    ($shape:ty) => {
        impl<G: Geometry> TileSurface<G> for $shape {
            type Domain = Spherical;
            fn chart() -> LibraryModule<G> {
                chart(
                    "hypertrace.chart.sphere",
                    "vec4<f32>(normalize(position.yzw),0)",
                )
            }
        }
    };
}
spherical_surface!(crate::shape::Sphere);
spherical_surface!(crate::shape::GeodesicSphere);

/// A tile selector defined on an intrinsic surface domain.
///
/// Library entry `(coordinate:vec4<f32>, cell:f32, width:f32, count:u32)->u32`
/// returns a tile material index, `count` for the border, or `count+1` to stop
/// an unresolved numerical sample. Its two parameter
/// words are cell size and width, validated by the returned module.
///
/// Invalid surface/pattern combinations fail to compile:
/// ```compile_fail
/// use ccgeom::Spherical3;
/// use hypertrace_objects::{object::tiling::{selector, Square}, shape::Plane};
/// let _ = selector::<Spherical3, Plane, Square>();
/// ```
/// ```compile_fail
/// use ccgeom::Flat3;
/// use hypertrace_objects::{object::tiling::{selector, RegularSpherical}, shape::Plane};
/// let _ = selector::<Flat3, Plane, RegularSpherical<4, 3>>();
/// ```
pub trait Tiling<D>: 'static {
    fn shader<G: Geometry>() -> Result<LibraryModule<G>>;
    fn parameters(&self) -> (f64, f64);
}

fn validate_width(words: &[u32]) -> Result<()> {
    let width = f32::from_bits(words[1]);
    anyhow::ensure!(
        width.is_finite() && width >= 0.0,
        "tile border width must be finite and nonnegative"
    );
    Ok(())
}
fn pattern<G: Geometry, const CELL: bool>(key: &str, source: &str) -> LibraryModule<G> {
    let mut module = LibraryModule::new(key, source, Some(2));
    module.validate_words = if CELL {
        |_, _, words| {
            validate_width(words)?;
            let cell = f32::from_bits(words[0]);
            anyhow::ensure!(
                cell.is_normal() && cell > 0.0,
                "tile cell size must be finite, positive and normal in f32"
            );
            Ok(())
        }
    } else {
        |_, _, words| validate_width(words)
    };
    module
}

/// Compose a surface chart with a compatible selector, preserving both modules'
/// validation. The entry has the selector signature and accepts embedded points.
pub fn selector<G, S, P>() -> Result<LibraryModule<G>>
where
    G: Geometry,
    S: TileSurface<G>,
    P: Tiling<S::Domain>,
{
    let mut module = LibraryModule::new("hypertrace.tiling.surface",
        "fn {{self}}(position:vec4<f32>,cell:f32,width:f32,count:u32)->u32 { return {{dep1}}({{dep0}}(position),cell,width,count); }", Some(2));
    module.dependencies = vec![S::chart().into_source(), P::shader::<G>()?.into_source()];
    module.key = LibraryModule::<G>::specialized_key(&module.key, &module.dependencies);
    module.validate_words = |module, context, words| {
        module.dependencies[0].validate(context, &[])?;
        module.dependencies[1].validate(context, words)
    };
    Ok(module)
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Uniform;
impl<D> Tiling<D> for Uniform {
    fn shader<G: Geometry>() -> Result<LibraryModule<G>> {
        Ok(pattern::<G, false>(
            "hypertrace.tiling.uniform",
            include_str!("shaders/uniform.wgsl"),
        ))
    }
    fn parameters(&self) -> (f64, f64) {
        (1.0, 0.0)
    }
}
macro_rules! flat_tiling {
    ($name:ident,$file:literal) => {
        /// A Euclidean pattern with cell size and border half-width in physical units.
        #[derive(Clone, Copy, Debug)]
        pub struct $name {
            pub cell_size: f64,
            pub border_width: f64,
        }
        impl $name {
            pub fn new(cell_size: f64, border_width: f64) -> Self {
                Self {
                    cell_size,
                    border_width,
                }
            }
        }
        impl Tiling<Euclidean> for $name {
            fn shader<G: Geometry>() -> Result<LibraryModule<G>> {
                Ok(pattern::<G, true>(
                    concat!("hypertrace.tiling.", $file),
                    include_str!(concat!("shaders/", $file, ".wgsl")),
                ))
            }
            fn parameters(&self) -> (f64, f64) {
                (self.cell_size, self.border_width)
            }
        }
    };
}
flat_tiling!(Square, "square");
flat_tiling!(Hexagonal, "hexagonal");
fn pentagon_module<G: Geometry>() -> LibraryModule<G> {
    LibraryModule::new(
        "hypertrace.tiling.pentagon",
        include_str!("shaders/pentagon.wgsl"),
        None,
    )
}
macro_rules! pentagon_tiling {
    ($name:ident, $file:literal) => {
        /// A regular hyperbolic pentagon pattern; width is curvature-normalized.
        #[derive(Clone, Copy, Debug)]
        pub struct $name {
            pub border_width: f64,
        }
        impl $name {
            pub fn new(border_width: f64) -> Self {
                Self { border_width }
            }
        }
        impl Tiling<Hyperbolic> for $name {
            fn shader<G: Geometry>() -> Result<LibraryModule<G>> {
                let mut module = pattern::<G, false>(
                    concat!("hypertrace.tiling.", $file),
                    include_str!(concat!("shaders/", $file, ".wgsl")),
                );
                module
                    .dependencies
                    .push(pentagon_module::<G>().into_source());
                Ok(module)
            }
            fn parameters(&self) -> (f64, f64) {
                (1.0, self.border_width)
            }
        }
    };
}
pentagon_tiling!(Pentagonal, "pentagonal");
pentagon_tiling!(Pentastar, "pentastar");

mod spherical;
pub use spherical::RegularSpherical;

/// Select among child materials with independently sized parameter payloads.
/// Cell size and border width precede an offset table; the border is the last child.
pub fn tiled_schema<G: Geometry>(
    selector: LibraryModule<G>,
    materials: Vec<MaterialModule<G>>,
    border: MaterialModule<G>,
) -> Result<MaterialModule<G>> {
    anyhow::ensure!(
        !materials.is_empty(),
        "tiled material collection must not be empty"
    );
    anyhow::ensure!(
        materials.len() <= i32::MAX as usize,
        "too many tiled materials"
    );
    let count = materials.len();
    let mut children = vec![selector.into_source()];
    children.extend(materials.into_iter().map(MaterialModule::into_source));
    children.push(border.into_source());
    let mut source = format!(
        "fn {{{{self}}}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {{\nlet index={{{{dep0}}}}(ctx.position,load_f32(base),load_f32(base+1u),{count}u);\nswitch index {{\n"
    );
    let parameter_words = crate::parameters::schema_size(2, &children[1..])?;
    for index in 0..children.len() - 1 {
        let slot = 2 + index;
        let dependency = index + 1;
        writeln!(
            source,
            "case {index}u: {{ {{{{dep{dependency}}}}}(base+load_u32(base+{slot}u),ctx,sample,rng); }}"
        )?;
    }
    source.push_str("default: {(*sample).alive=0u;}\n}\n}\n");
    let mut module =
        MaterialModule::new("hypertrace.material.tiled.offsets", source, parameter_words);
    module.dependencies = children;
    module.key = MaterialModule::<G>::specialized_key(&module.key, &module.dependencies);
    module.validate_words = |module, ctx, words| {
        anyhow::ensure!(words.len() >= 2, "tiling parameters are truncated");
        let selector = module
            .dependencies
            .first()
            .ok_or_else(|| anyhow::anyhow!("missing tiling selector"))?;
        selector.validate(ctx, &words[..2])?;
        let children = &module.dependencies[1..];
        let payloads = crate::parameters::slices(words, 2, children.len())?;
        for (child, payload) in children.iter().zip(payloads) {
            child.validate(ctx, payload)?;
        }
        Ok(())
    };
    Ok(module)
}

pub fn tiled<G: Geometry>(
    selector: LibraryModule<G>,
    materials: Vec<MaterialValue<G>>,
    border: MaterialValue<G>,
    cell_size: f64,
    border_width: f64,
) -> Result<MaterialValue<G>> {
    let _ = u32::try_from(materials.len())?;
    let cell = crate::shader::finite_f32(cell_size)?;
    let width = crate::shader::finite_f32(border_width)?;
    anyhow::ensure!(width >= 0.0, "tile border width must be nonnegative");
    let prefix = vec![cell.to_bits(), width.to_bits()];
    let mut schemas = Vec::new();
    let mut payloads = Vec::new();
    for material in materials {
        schemas.push(material.schema);
        payloads.push(material.words);
    }
    payloads.push(border.words);
    MaterialValue::new(
        tiled_schema(selector, schemas, border.schema)?,
        crate::parameters::pack(prefix, &payloads)?,
    )
}
