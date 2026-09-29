use super::*;
use std::{convert::TryFrom, fmt::Write};

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

pub fn mixture_schema(children: Vec<ShaderModule>) -> Result<ShaderModule> {
    anyhow::ensure!(
        children
            .iter()
            .all(|child| child.kind == ShaderKind::Material),
        "expected a material dependency"
    );
    let mut source=String::from("fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {\nvar choice=uniform_random(rng);\n");
    let mut offset = u32::try_from(children.len())?;
    for (index, child) in children.iter().enumerate() {
        writeln!(source,"choice-=load_f32(base+{index}u);\nif choice<0 {{ {{{{dep{index}}}}}(base+{offset}u,ctx,sample,rng);return;}}")?;
        offset = offset
            .checked_add(child.parameter_words.ok_or_else(|| {
                anyhow::anyhow!("mixture components require fixed-size material parameters")
            })?)
            .ok_or_else(|| anyhow::anyhow!("material parameters overflow"))?;
    }
    source.push_str("(*sample).alive=0u;\n}\n");
    let mut module = ShaderModule::new(
        "hypertrace.material.mixture",
        ShaderKind::Material,
        source,
        Some(offset),
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
        let mut offset = count;
        for child in &module.dependencies {
            let end = offset
                + child.parameter_words.ok_or_else(|| {
                    anyhow::anyhow!("mixture components require fixed-size material parameters")
                })? as usize;
            anyhow::ensure!(end <= words.len(), "mixture child payload is truncated");
            child.validate(ctx, &words[offset..end])?;
            offset = end;
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
    for (_, value) in components {
        children.push(value.schema);
        words.extend(value.words);
    }
    MaterialValue::new(mixture_schema(children)?, words)
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
