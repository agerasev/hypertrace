use super::*;
use std::convert::TryFrom;

pub fn vector_schema(inner: ShaderModule) -> ShaderModule {
    let mut module = ShaderModule::new(
        "hypertrace.shape.vector",
        ShaderKind::Shape,
        r#"
fn {{self}}(base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {
    var result=GeoTaggedHit(geo_miss(),0xffffffffu);
    for(var index=0u;index<load_u32(base);index+=1u) {
        let candidate={{dep0}}(base+load_u32(base+1u+index),ray,previous_identity);
        if candidate.hit.valid==2u {return candidate;}
        if candidate.hit.valid!=0u {
            if result.hit.valid==0u || candidate.hit.distance<result.hit.distance {result=candidate;}
        }
    }
    return result;
}
"#,
        None,
    );
    module.dependencies = vec![inner];
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
        let count = *words
            .first()
            .ok_or_else(|| anyhow::anyhow!("shape vector payload is empty"))?
            as usize;
        anyhow::ensure!(
            count < words.len(),
            "shape vector offset table is truncated"
        );
        if count == 0 {
            anyhow::ensure!(words.len() == 1, "empty shape vector has excess payload");
        }
        for index in 0..count {
            let start = words[index + 1] as usize;
            let end = if index + 1 < count {
                words[index + 2] as usize
            } else {
                words.len()
            };
            anyhow::ensure!(
                start > count && start < end && end <= words.len(),
                "invalid shape vector element range"
            );
            if index == 0 {
                anyhow::ensure!(start == 1 + count, "shape vector payload has a gap");
            }
            module.dependencies[0].validate(ctx, &words[start..end])?;
        }
        Ok(())
    };
    module
}

pub fn vector(element: ShaderModule, values: Vec<ShapeValue>) -> Result<ShapeValue> {
    anyhow::ensure!(element.kind == ShaderKind::Shape, "expected a shape module");
    anyhow::ensure!(values.len() < u32::MAX as usize, "too many shapes");
    let mut words = vec![values.len() as u32];
    words.resize(1 + values.len(), 0);
    for (index, value) in values.into_iter().enumerate() {
        anyhow::ensure!(
            value.schema.same_implementation(&element),
            "shape vector schema mismatch"
        );
        words[index + 1] = u32::try_from(words.len())?;
        words.extend(value.words);
    }
    ShapeValue::new(vector_schema(element), words)
}

impl<G: Geometry, T: Shape<G>> Shape<G> for Vec<T> {
    fn shader() -> Result<ShaderModule> {
        Ok(vector_schema(T::shader()?))
    }
    fn encode(&self) -> Result<ShapeValue> {
        vector(
            T::shader()?,
            self.iter().map(T::encode).collect::<Result<_>>()?,
        )
    }
}
