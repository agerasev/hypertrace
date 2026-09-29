use super::*;
use vecmat::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Colored<M: Material> {
    pub color: Vector<f32, 3>,
    pub inner: M,
}
impl<M: Material> Colored<M> {
    pub fn new(material: M, color: Vector<f32, 3>) -> Self {
        Self {
            inner: material,
            color,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Emissive<M: Material> {
    pub emission: Vector<f32, 3>,
    pub inner: M,
}
impl<M: Material> Emissive<M> {
    pub fn new(material: M, emission: Vector<f32, 3>) -> Self {
        Self {
            inner: material,
            emission,
        }
    }
}

fn modifier_schema(inner: ShaderModule, emission: bool) -> ShaderModule {
    let (key, statement) = if emission {
        (
            "hypertrace.material.emissive",
            "(*sample).emission+=(*sample).attenuation*load_vec3(base);",
        )
    } else {
        (
            "hypertrace.material.colored",
            "(*sample).attenuation*=load_vec3(base);",
        )
    };
    let source=format!("fn {{{{self}}}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {{\n{statement}\n{{{{dep0}}}}(base+3u,ctx,sample,rng);\n}}\n");
    let mut module = ShaderModule::new(
        key,
        ShaderKind::Material,
        source,
        inner.parameter_words.and_then(|n| n.checked_add(3)),
    );
    module.dependencies = vec![inner];
    module.key = ShaderModule::specialized_key(&module.key, &module.dependencies);
    if module
        .dependencies
        .iter()
        .any(|child| child.kind != ShaderKind::Material)
    {
        module.validate_context = |_| Err(anyhow::anyhow!("expected a material dependency"));
    }
    module.validate_words = |module, ctx, words| {
        anyhow::ensure!(
            module
                .dependencies
                .iter()
                .all(|child| child.kind == ShaderKind::Material),
            "expected a material dependency"
        );
        anyhow::ensure!(words.len() >= 3, "material modifier payload is truncated");
        validate_rgb(&words[..3])?;
        module.dependencies[0].validate(ctx, &words[3..])
    };
    module
}
pub fn colored_schema(inner: ShaderModule) -> ShaderModule {
    modifier_schema(inner, false)
}
pub fn emissive_schema(inner: ShaderModule) -> ShaderModule {
    modifier_schema(inner, true)
}
fn validate_rgb(words: &[u32]) -> Result<()> {
    anyhow::ensure!(
        words.iter().all(|&w| {
            let x = f32::from_bits(w);
            x.is_finite() && x >= 0.0
        }),
        "material color must be finite and nonnegative"
    );
    Ok(())
}
fn modifier(value: MaterialValue, rgb: [f32; 3], emission: bool) -> Result<MaterialValue> {
    anyhow::ensure!(
        value.schema.kind == ShaderKind::Material,
        "expected a material module"
    );
    let mut words = rgb.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
    validate_rgb(&words)?;
    words.extend(value.words);
    MaterialValue::new(modifier_schema(value.schema, emission), words)
}
pub fn colored(value: MaterialValue, rgb: [f32; 3]) -> Result<MaterialValue> {
    modifier(value, rgb, false)
}
pub fn emissive(value: MaterialValue, rgb: [f32; 3]) -> Result<MaterialValue> {
    modifier(value, rgb, true)
}
impl<M: Material> Material for Colored<M> {
    fn shader() -> Result<ShaderModule> {
        Ok(colored_schema(M::shader()?))
    }
    fn encode(&self) -> Result<MaterialValue> {
        colored(self.inner.encode()?, self.color.into_array())
    }
}
impl<M: Material> Material for Emissive<M> {
    fn shader() -> Result<ShaderModule> {
        Ok(emissive_schema(M::shader()?))
    }
    fn encode(&self) -> Result<MaterialValue> {
        emissive(self.inner.encode()?, self.emission.into_array())
    }
}
