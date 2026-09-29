use super::*;
use std::{convert::TryFrom, fmt::Write};

pub fn choice_schema(variants: Vec<ShaderModule>) -> ShaderModule {
    let mut source=String::from("fn {{self}}(base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {\nswitch load_u32(base) {\n");
    for index in 0..variants.len() {
        writeln!(
            source,
            "case {index}u: {{return {{{{dep{index}}}}}(base+1u,ray,previous_identity);}}"
        )
        .unwrap();
    }
    source.push_str("default: {return GeoTaggedHit(geo_miss(),0xffffffffu);}\n}\n}\n");
    let mut module = ShaderModule::new("hypertrace.shape.choice", ShaderKind::Shape, source, None);
    module.dependencies = variants;
    module.key = ShaderModule::specialized_key(&module.key, &module.dependencies);
    if module
        .dependencies
        .iter()
        .any(|child| child.kind != ShaderKind::Shape)
    {
        module.validate_context = |_| Err(anyhow::anyhow!("expected a shape dependency"));
    }
    module.validate_words = |module, ctx, words| {
        anyhow::ensure!(
            module
                .dependencies
                .iter()
                .all(|child| child.kind == ShaderKind::Shape),
            "expected a shape dependency"
        );
        let index = *words
            .first()
            .ok_or_else(|| anyhow::anyhow!("shape choice payload is empty"))?
            as usize;
        let child = module
            .dependencies
            .get(index)
            .ok_or_else(|| anyhow::anyhow!("shape choice index is outside its variants"))?;
        child.validate(ctx, &words[1..])
    };
    module
}
pub fn choice(variants: Vec<ShaderModule>, index: usize, value: ShapeValue) -> Result<ShapeValue> {
    anyhow::ensure!(
        variants.iter().all(|v| v.kind == ShaderKind::Shape),
        "expected a shape module"
    );
    anyhow::ensure!(
        variants
            .get(index)
            .is_some_and(|s| s.same_implementation(&value.schema)),
        "shape choice schema mismatch"
    );
    let mut words = vec![u32::try_from(index)?];
    words.extend(value.words);
    ShapeValue::new(choice_schema(variants), words)
}

#[macro_export]
macro_rules! shape_choice {
    { $self:ident { $( $variant:ident($vtype:ty) ),* $(,)? } } => {
        $crate::choice! { $self { $( $variant($vtype), )* } }
        impl<G:$crate::Geometry> $crate::Shape<G> for $self
        where $( $vtype:$crate::Shape<G>, )* {
            fn shader()->$crate::shader::Result<$crate::shader::ShaderModule> {
                Ok($crate::shape::choice_schema(vec![$(<$vtype as $crate::Shape<G>>::shader()?,)*]))
            }
            fn encode(&self)->$crate::shader::Result<$crate::shader::ShapeValue> {
                #[allow(non_camel_case_types)] enum VariantIndex { $($variant,)* }
                let variants=vec![$(<$vtype as $crate::Shape<G>>::shader()?,)*];
                match self {$(Self::$variant(value)=>$crate::shape::choice(variants,VariantIndex::$variant as usize,<$vtype as $crate::Shape<G>>::encode(value)?),)*}
            }
        }
    };
}
