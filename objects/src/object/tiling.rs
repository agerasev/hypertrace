//! Material selection components; the renderer does not interpret tile families.
use crate::shader::{Geometry, GeometryContext, MaterialValue, Result, ShaderKind, ShaderModule};
use std::{convert::TryFrom, fmt::Write};

pub trait Tiling: 'static {
    /// A library entry returning a material index, or `count` for the border.
    fn shader() -> ShaderModule;
    fn uses_cell_size() -> bool {
        false
    }
}

fn validate_hyperbolic(ctx: GeometryContext) -> Result<()> {
    anyhow::ensure!(
        ctx.geometry == Geometry::Hyperbolic,
        "this tiling requires hyperbolic geometry"
    );
    Ok(())
}
fn validate_width(words: &[u32]) -> Result<()> {
    let width = f32::from_bits(words[1]);
    anyhow::ensure!(
        width.is_finite() && width >= 0.0,
        "tile border width must be finite and nonnegative"
    );
    Ok(())
}
fn selector(key: &str, source: &str, hyperbolic: bool, cell: bool) -> ShaderModule {
    let mut module = ShaderModule::new(key, ShaderKind::Library, source, Some(2));
    if hyperbolic {
        module.validate_context = validate_hyperbolic;
    }
    module.validate_words = if cell {
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

macro_rules! tiling {
    ($name:ident,$file:literal,$hyperbolic:literal,$cell:literal) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name;
        impl Tiling for $name {
            fn shader() -> ShaderModule {
                selector(
                    concat!("hypertrace.tiling.", $file),
                    include_str!(concat!("shaders/", $file, ".wgsl")),
                    $hyperbolic,
                    $cell,
                )
            }
            fn uses_cell_size() -> bool {
                $cell
            }
        }
    };
}
tiling!(Uniform, "uniform", false, false);
tiling!(Square, "square", true, true);
tiling!(Hexagonal, "hexagonal", true, true);
tiling!(Pentagonal, "pentagonal", true, false);
tiling!(Pentastar, "pentastar", true, false);

/// Select among child materials with independently sized parameter payloads.
/// Cell size and border width precede an offset table; the border is the last child.
pub fn tiled_schema(
    selector: ShaderModule,
    materials: Vec<ShaderModule>,
    border: ShaderModule,
) -> Result<ShaderModule> {
    anyhow::ensure!(
        !materials.is_empty(),
        "tiled material collection must not be empty"
    );
    anyhow::ensure!(
        materials.len() <= i32::MAX as usize,
        "too many tiled materials"
    );
    anyhow::ensure!(
        selector.kind == ShaderKind::Library,
        "tiling selector must be a library module"
    );
    let count = materials.len();
    let mut children = vec![selector];
    children.extend(materials);
    children.push(border);
    let mut source=format!("fn {{{{self}}}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {{\nlet index={{{{dep0}}}}(geo_to_chart_pos(ctx.position),load_f32(base),load_f32(base+1u),{count}u);\nswitch index {{\n");
    let parameter_words = crate::parameters::schema_size(2, &children[1..])?;
    for (index, child) in children[1..].iter().enumerate() {
        anyhow::ensure!(
            child.kind == ShaderKind::Material,
            "tiling child must be a material module"
        );
        let slot = 2 + index;
        let dependency = index + 1;
        writeln!(source,
            "case {index}u: {{ {{{{dep{dependency}}}}}(base+load_u32(base+{slot}u),ctx,sample,rng); }}")?;
    }
    source.push_str("default: {(*sample).alive=0u;}\n}\n}\n");
    let mut module = ShaderModule::new(
        "hypertrace.material.tiled.offsets",
        ShaderKind::Material,
        source,
        parameter_words,
    );
    module.dependencies = children;
    module.key = ShaderModule::specialized_key(&module.key, &module.dependencies);
    module.validate_words = |module, ctx, words| {
        anyhow::ensure!(words.len() >= 2, "tiling parameters are truncated");
        let selector = module
            .dependencies
            .first()
            .ok_or_else(|| anyhow::anyhow!("missing tiling selector"))?;
        anyhow::ensure!(
            selector.kind == ShaderKind::Library,
            "tiling selector must be a library module"
        );
        selector.validate(ctx, &words[..2])?;
        let children = &module.dependencies[1..];
        let payloads = crate::parameters::slices(words, 2, children.len())?;
        for (child, payload) in children.iter().zip(payloads) {
            anyhow::ensure!(
                child.kind == ShaderKind::Material,
                "tiling child must be a material module"
            );
            child.validate(ctx, payload)?;
        }
        Ok(())
    };
    Ok(module)
}

pub fn tiled(
    selector: ShaderModule,
    materials: Vec<MaterialValue>,
    border: MaterialValue,
    cell_size: f64,
    border_width: f64,
) -> Result<MaterialValue> {
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
