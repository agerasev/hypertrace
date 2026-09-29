use super::*;
use crate::Mapped;

pub fn mapped_schema<G: Geometry>(inner: ShapeModule<G>) -> ShapeModule<G> {
    let mut module = ShapeModule::new(
        format!("hypertrace.shape.mapped.{}", G::SIGN),
        r#"
fn {{self}}(base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {
    let map=GeoMap(load_vec4(base),load_vec4(base+4u));
    let local=geo_map_ray(geo_inverse(map),ray);
    if !geo_ray_supported(local) {return GeoTaggedHit(geo_failure(),base);}
    let result={{dep0}}(base+8u,local,previous_identity);
    return GeoTaggedHit(geo_map_hit(map,result.hit),result.identity);
}
"#,
        inner.parameter_words.and_then(|n| n.checked_add(8)),
    );
    module.dependencies = vec![inner.into_source()];
    module.key = ShapeModule::<G>::specialized_key(&module.key, &module.dependencies);
    module.validate_words = |module, ctx, words| {
        anyhow::ensure!(words.len() >= 8, "mapped shape payload is truncated");
        let rows =
            std::array::from_fn(|r| std::array::from_fn(|c| f32::from_bits(words[4 * r + c])));
        crate::shader::validate_embedded_rows::<G>(rows)?;
        module.dependencies[0].validate(ctx, &words[8..])
    };
    module
}

pub fn mapped<G: Geometry>(value: ShapeValue<G>, map: Transform<G>) -> Result<ShapeValue<G>> {
    let mut words = map.words()?;
    words.extend(value.words);
    ShapeValue::new(mapped_schema(value.schema), words)
}

impl<G: Geometry, T: Shape<G>, M: crate::shader::RenderMap<G>> Shape<G> for Mapped<G, T, M> {
    fn shader() -> Result<ShapeModule<G>> {
        Ok(mapped_schema(T::shader()?))
    }
    fn encode(&self) -> Result<ShapeValue<G>> {
        mapped(
            self.inner.encode()?,
            crate::shader::transform::<G, M>(&self.map)?,
        )
    }
}
