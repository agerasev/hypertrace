use super::*;
use vecmat::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Colored<M> {
    pub color: Vector<f32, 3>,
    pub inner: M,
}
impl<M> Colored<M> {
    pub fn new(material: M, color: Vector<f32, 3>) -> Self {
        Self {
            inner: material,
            color,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Emissive<M> {
    pub emission: Vector<f32, 3>,
    pub inner: M,
}
impl<M> Emissive<M> {
    pub fn new(material: M, emission: Vector<f32, 3>) -> Self {
        Self {
            inner: material,
            emission,
        }
    }
}

fn modifier_schema<G: Geometry, const EMISSION: bool>(
    inner: MaterialModule<G>,
) -> MaterialModule<G> {
    let (key, statement) = if EMISSION {
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
    let source = format!(
        "fn {{{{self}}}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {{\n{statement}\n{{{{dep0}}}}(base+3u,ctx,sample,rng);\n}}\n"
    );
    let mut module = MaterialModule::new(
        key,
        source,
        inner.parameter_words.and_then(|n| n.checked_add(3)),
    );
    module.dependencies = vec![inner.into_source()];
    module.key = MaterialModule::<G>::specialized_key(&module.key, &module.dependencies);
    module.validate_words = |module, ctx, words| {
        anyhow::ensure!(words.len() >= 3, "material modifier payload is truncated");
        validate_rgb(&words[..3])?;
        module.dependencies[0].validate(ctx, &words[3..])
    };
    module
}
pub fn colored_schema<G: Geometry>(inner: MaterialModule<G>) -> MaterialModule<G> {
    modifier_schema::<G, false>(inner)
}
pub fn emissive_schema<G: Geometry>(inner: MaterialModule<G>) -> MaterialModule<G> {
    modifier_schema::<G, true>(inner)
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
fn modifier<G: Geometry, const EMISSION: bool>(
    value: MaterialValue<G>,
    rgb: [f32; 3],
) -> Result<MaterialValue<G>> {
    let mut words = rgb.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
    validate_rgb(&words)?;
    words.extend(value.words);
    MaterialValue::new(modifier_schema::<G, EMISSION>(value.schema), words)
}
pub fn colored<G: Geometry>(value: MaterialValue<G>, rgb: [f32; 3]) -> Result<MaterialValue<G>> {
    modifier::<G, false>(value, rgb)
}
pub fn emissive<G: Geometry>(value: MaterialValue<G>, rgb: [f32; 3]) -> Result<MaterialValue<G>> {
    modifier::<G, true>(value, rgb)
}
impl<G: Geometry, M: Material<G>> Material<G> for Colored<M> {
    fn shader() -> Result<MaterialModule<G>> {
        Ok(colored_schema(M::shader()?))
    }
    fn encode(&self) -> Result<MaterialValue<G>> {
        colored(self.inner.encode()?, self.color.into_array())
    }
}
impl<G: Geometry, M: Material<G>> Material<G> for Emissive<M> {
    fn shader() -> Result<MaterialModule<G>> {
        Ok(emissive_schema(M::shader()?))
    }
    fn encode(&self) -> Result<MaterialValue<G>> {
        emissive(self.inner.encode()?, self.emission.into_array())
    }
}
