use super::*;
use std::fmt::Write;

#[derive(Clone, Copy, Debug)]
pub struct Component<M: Material> {
    pub material: M,
    pub portion: f64,
}
impl<M: Material> From<(M, f64)> for Component<M> {
    fn from((material, portion): (M, f64)) -> Self {
        Self { material, portion }
    }
}

/// Describe a mixture independently of child parameter lengths.
/// Weights precede a relative offset table and the concatenated child payloads.
pub fn mixture_schema(children: Vec<ShaderModule>) -> Result<ShaderModule> {
    anyhow::ensure!(
        children
            .iter()
            .all(|child| child.kind == ShaderKind::Material),
        "expected a material dependency"
    );
    let mut source=String::from("fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {\nvar choice=uniform_random(rng);\n");
    let count = children.len();
    let parameter_words = crate::parameters::schema_size(count, &children)?;
    for index in 0..count {
        let slot = count + index;
        writeln!(source,"choice-=load_f32(base+{index}u);\nif choice<0 {{ {{{{dep{index}}}}}(base+load_u32(base+{slot}u),ctx,sample,rng);return;}}")?;
    }
    source.push_str("(*sample).alive=0u;\n}\n");
    let mut module = ShaderModule::new(
        "hypertrace.material.mixture.offsets",
        ShaderKind::Material,
        source,
        parameter_words,
    );
    module.dependencies = children;
    module.key = ShaderModule::specialized_key(&module.key, &module.dependencies);
    module.validate_words = |module, ctx, words| {
        anyhow::ensure!(
            module
                .dependencies
                .iter()
                .all(|child| child.kind == ShaderKind::Material),
            "expected a material dependency"
        );
        let count = module.dependencies.len();
        anyhow::ensure!(words.len() >= count, "mixture weight payload is truncated");
        let mut total = 0.0f64;
        for word in &words[..count] {
            let weight = f32::from_bits(*word);
            anyhow::ensure!(
                weight.is_finite() && weight >= 0.0,
                "mixture weight must be finite and nonnegative"
            );
            total += f64::from(weight);
        }
        anyhow::ensure!(total <= 1.00001, "mixture portions exceed one");
        let payloads = crate::parameters::slices(words, count, count)?;
        for (child, payload) in module.dependencies.iter().zip(payloads) {
            child.validate(ctx, payload)?;
        }
        Ok(())
    };
    Ok(module)
}

pub fn mixture(components: Vec<(f64, MaterialValue)>) -> Result<MaterialValue> {
    let mut words = Vec::new();
    let mut total = 0.0;
    for (portion, _) in &components {
        let weight = crate::shader::finite_f32(*portion)?;
        anyhow::ensure!(weight >= 0.0, "mixture portions must be nonnegative");
        total += *portion;
        words.push(weight.to_bits());
    }
    anyhow::ensure!(total <= 1.00001, "mixture portions exceed one");
    let mut children = Vec::new();
    let mut payloads = Vec::new();
    for (_, value) in components {
        children.push(value.schema);
        payloads.push(value.words);
    }
    MaterialValue::new(
        mixture_schema(children)?,
        crate::parameters::pack(words, &payloads)?,
    )
}

#[macro_export]
macro_rules! mixture {
    { $self:ident { $( $component:ident : $mtype:ty ),* $(,)? } } => {
        #[derive(Clone)] pub struct $self {$(pub $component:$crate::material::Component<$mtype>,)*}
        #[allow(dead_code)] impl $self {
            pub fn new($($component:$crate::material::Component<$mtype>,)*)->Self {Self {$($component,)*}}
        }
        impl $crate::Material for $self where $($mtype:$crate::Material,)* {
            fn shader()->$crate::shader::Result<$crate::shader::ShaderModule> {
                $crate::material::mixture_schema(vec![$(<$mtype as $crate::Material>::shader()?,)*])
            }
            fn encode(&self)->$crate::shader::Result<$crate::shader::MaterialValue> {
                $crate::material::mixture(vec![$((self.$component.portion,<$mtype as $crate::Material>::encode(&self.$component.material)?),)*])
            }
        }
    };
}
