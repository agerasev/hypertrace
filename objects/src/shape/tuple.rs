use super::*;
use std::fmt::Write;

/// A heterogeneous union whose shader calls every statically known child.
pub fn tuple_schema<G: Geometry>(children: Vec<ShapeModule<G>>) -> Result<ShapeModule<G>> {
    let children: Vec<_> = children.into_iter().map(ShapeModule::into_source).collect();
    let mut source = String::from("fn {{self}}(base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {\nvar result=GeoTaggedHit(geo_miss(),0xffffffffu);\n");
    for index in 0..children.len() {
        writeln!(source,
            "let candidate{index}={{{{dep{index}}}}}(base+load_u32(base+{index}u),ray,previous_identity);\nif candidate{index}.hit.valid==2u {{return candidate{index};}}\nif candidate{index}.hit.valid!=0u {{\nif result.hit.valid==0u || candidate{index}.hit.distance<result.hit.distance {{result=candidate{index};}}\n}}")?;
    }
    source.push_str("return result;\n}\n");
    let mut module = ShapeModule::new(
        "hypertrace.shape.tuple",
        source,
        crate::parameters::schema_size(0, &children)?,
    );
    module.dependencies = children;
    module.key = ShapeModule::<G>::specialized_key(&module.key, &module.dependencies);
    module.validate_words = |module, ctx, words| {
        let payloads = crate::parameters::slices(words, 0, module.dependencies.len())?;
        for (child, payload) in module.dependencies.iter().zip(payloads) {
            child.validate(ctx, payload)?;
        }
        Ok(())
    };
    Ok(module)
}

pub fn tuple<G: Geometry>(children: Vec<ShapeValue<G>>) -> Result<ShapeValue<G>> {
    let mut modules = Vec::new();
    let mut words = Vec::new();
    for child in children {
        modules.push(child.schema);
        words.push(child.words);
    }
    ShapeValue::new(
        tuple_schema(modules)?,
        crate::parameters::pack(vec![], &words)?,
    )
}

impl<G: Geometry> Shape<G> for () {
    fn shader() -> Result<ShapeModule<G>> {
        tuple_schema(vec![])
    }

    fn encode(&self) -> Result<ShapeValue<G>> {
        tuple(vec![])
    }
}

macro_rules! tuple {
    ($( $type:ident : $index:tt ),+) => {
        impl<G: Geometry, $( $type: Shape<G> ),+> Shape<G> for ($( $type, )+) {
            fn shader() -> Result<ShapeModule<G>> {
                tuple_schema(vec![$($type::shader()?,)+])
            }

            fn encode(&self) -> Result<ShapeValue<G>> {
                tuple(vec![$(self.$index.encode()?,)+])
            }
        }
    };
}

tuple!(A:0);
tuple!(A:0, B:1);
tuple!(A:0, B:1, C:2);
tuple!(A:0, B:1, C:2, D:3);
tuple!(A:0, B:1, C:2, D:3, E:4);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6, I:7);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6, I:7, J:8);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6, I:7, J:8, K:9);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6, I:7, J:8, K:9, L:10);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6, I:7, J:8, K:9, L:10, M:11);
