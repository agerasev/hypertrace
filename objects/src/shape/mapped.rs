use super::*;
use crate::Mapped;

pub fn mapped_schema(geometry: RenderGeometry, inner: ShaderModule) -> ShaderModule {
    let mut module = ShaderModule::new(
        format!("hypertrace.shape.mapped.{}", geometry.sign()),
        ShaderKind::Shape,
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
    module.dependencies = vec![inner];
    module.key = ShaderModule::specialized_key(&module.key, &module.dependencies);
    module.validate_context = match geometry {
        RenderGeometry::Euclidean => |ctx| {
            anyhow::ensure!(
                ctx.geometry == RenderGeometry::Euclidean,
                "shape map geometry does not match the scene"
            );
            Ok(())
        },
        RenderGeometry::Hyperbolic => |ctx| {
            anyhow::ensure!(
                ctx.geometry == RenderGeometry::Hyperbolic,
                "shape map geometry does not match the scene"
            );
            Ok(())
        },
        RenderGeometry::Spherical => |ctx| {
            anyhow::ensure!(
                ctx.geometry == RenderGeometry::Spherical,
                "shape map geometry does not match the scene"
            );
            Ok(())
        },
    };
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
        anyhow::ensure!(words.len() >= 8, "mapped shape payload is truncated");
        let rows =
            std::array::from_fn(|r| std::array::from_fn(|c| f32::from_bits(words[4 * r + c])));
        crate::shader::validate_embedded_rows(rows, ctx.geometry)?;
        module.dependencies[0].validate(ctx, &words[8..])
    };
    module
}

pub fn mapped(value: ShapeValue, map: Transform) -> Result<ShapeValue> {
    anyhow::ensure!(
        value.schema.kind == ShaderKind::Shape,
        "expected a shape module"
    );
    let mut words = map.words()?;
    words.extend(value.words);
    ShapeValue::new(mapped_schema(map.geometry(), value.schema), words)
}

impl<G: crate::shader::RenderGeometry, T: Shape<G>, M: crate::shader::RenderMap<G>> Shape<G>
    for Mapped<G, T, M>
{
    fn shader() -> Result<ShaderModule> {
        Ok(mapped_schema(crate::shader::geometry::<G>()?, T::shader()?))
    }
    fn encode(&self) -> Result<ShapeValue> {
        mapped(
            self.inner.encode()?,
            crate::shader::transform::<G, M>(&self.map)?,
        )
    }
}
