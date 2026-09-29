//! Material selection components; the renderer does not interpret tile families.
use crate::shader::{Geometry, LibraryModule, MaterialModule, MaterialValue, Result};
use ccgeom::Hyperboloid3;
use std::{convert::TryFrom, fmt::Write};

/// A selector whose coordinate geometry is part of its type.
/// ```compile_fail
/// use ccgeom::Spherical3;
/// use hypertrace_objects::object::tiling::{Square, Tiling};
/// let _ = <Square as Tiling<Spherical3>>::shader();
/// ```
pub trait Tiling<G: Geometry>: 'static {
    /// An embedded-position library entry returning a material index, or
    /// `count` for the border. A selector explicitly chooses its chart if needed.
    fn shader() -> LibraryModule<G>;
    const USES_CELL_SIZE: bool = false;
}

fn validate_width(words: &[u32]) -> Result<()> {
    let width = f32::from_bits(words[1]);
    anyhow::ensure!(
        width.is_finite() && width >= 0.0,
        "tile border width must be finite and nonnegative"
    );
    Ok(())
}
fn selector<G: Geometry, const CELL: bool>(key: &str, source: &str) -> LibraryModule<G> {
    let mut module = LibraryModule::new(key, source, Some(2));
    module.validate_words = if CELL {
        |_, _, words| {
            validate_width(words)?;
            let cell = f32::from_bits(words[0]);
            anyhow::ensure!(
                cell.is_finite() && cell > 0.0,
                "tile cell size must be finite and positive"
            );
            Ok(())
        }
    } else {
        |_, _, words| validate_width(words)
    };
    module
}

#[derive(Clone, Copy, Debug)]
pub struct Uniform;
impl<G: Geometry> Tiling<G> for Uniform {
    fn shader() -> LibraryModule<G> {
        selector::<G, false>(
            "hypertrace.tiling.uniform",
            include_str!("shaders/uniform.wgsl"),
        )
    }
}
macro_rules! tiling {
    ($name:ident,$file:literal) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name;
        impl Tiling<Hyperboloid3> for $name {
            fn shader() -> LibraryModule<Hyperboloid3> {
                selector::<Hyperboloid3, true>(
                    concat!("hypertrace.tiling.", $file),
                    include_str!(concat!("shaders/", $file, ".wgsl")),
                )
            }
            const USES_CELL_SIZE: bool = true;
        }
    };
}
tiling!(Square, "square");
tiling!(Hexagonal, "hexagonal");
fn pentagon_module() -> LibraryModule<Hyperboloid3> {
    LibraryModule::new(
        "hypertrace.tiling.pentagon",
        include_str!("shaders/pentagon.wgsl"),
        None,
    )
}
macro_rules! pentagon_tiling {
    ($name:ident, $file:literal) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name;
        impl Tiling<Hyperboloid3> for $name {
            fn shader() -> LibraryModule<Hyperboloid3> {
                let mut module = selector::<Hyperboloid3, false>(
                    concat!("hypertrace.tiling.", $file),
                    include_str!(concat!("shaders/", $file, ".wgsl")),
                );
                module.dependencies.push(pentagon_module().into_source());
                module
            }
        }
    };
}
pentagon_tiling!(Pentagonal, "pentagonal");
pentagon_tiling!(Pentastar, "pentastar");

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
