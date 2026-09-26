//! Lower schema/value descriptions into a stable storage ABI and WGSL functions.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use anyhow::{Context as _, ensure};
use bytemuck::{Pod, Zeroable};

use crate::{
    Geometry, MaterialSchema, MaterialValue, ObjectNode, Result, SceneDefinition, ShaderLeaf,
    ShapeSchema, ShapeValue, Tiling, Transform, finite_f32,
};

/// Object transforms remain in the material's coordinate frame. Mapped shapes
/// are generated independently and transform their hit back into that frame.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GpuObject {
    pub map0: [f32; 4],
    pub map1: [f32; 4],
    /// Shape function, first material, material count, tiling selector.
    pub info: [u32; 4],
    /// Border material, shape word offset, reserved, reserved.
    pub extra: [u32; 4],
    pub props: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MaterialRecord {
    /// Material function, word offset, reserved, reserved.
    pub data: [u32; 4],
}

#[derive(Clone, Debug)]
pub struct CompiledScene {
    pub geometry: Geometry,
    pub objects: Vec<GpuObject>,
    pub materials: Vec<MaterialRecord>,
    pub words: Vec<u32>,
    /// Functions only. Concatenate after the shared math and generated runtime.
    /// Values, vector lengths and object counts do not occur in this string.
    pub source: String,
}

struct Compiler {
    geometry: Geometry,
    shapes: BTreeMap<ShapeSchema, u32>,
    materials: BTreeMap<MaterialSchema, u32>,
    result: CompiledScene,
}

/// Compile the active values and all registered alternatives. Registration makes
/// changing a choice variant or adding vector elements a buffer-only update.
pub fn compile(scene: &SceneDefinition) -> Result<CompiledScene> {
    let geometry = scene.view.map.geometry();
    let mut shapes = BTreeSet::new();
    let mut materials = BTreeSet::new();
    for schema in &scene.shape_schemas {
        collect_shape(schema, &mut shapes);
    }
    for schema in &scene.material_schemas {
        collect_material(schema, &mut materials);
    }
    collect_object(&scene.object, &mut shapes, &mut materials);
    let mut compiler = Compiler {
        geometry,
        shapes: shapes
            .into_iter()
            .enumerate()
            .map(|(i, s)| Ok((s, u32::try_from(i)?)))
            .collect::<Result<_>>()?,
        materials: materials
            .into_iter()
            .enumerate()
            .map(|(i, s)| Ok((s, u32::try_from(i)?)))
            .collect::<Result<_>>()?,
        result: CompiledScene {
            geometry,
            objects: vec![],
            materials: vec![],
            words: vec![],
            source: String::new(),
        },
    };
    for schema in compiler.shapes.keys() {
        validate_geometry(schema, geometry)?;
    }
    compiler.result.source = compiler.source()?;
    compiler.object(&scene.object, Transform::identity(geometry))?;
    // wgpu storage bindings cannot be empty. Leaf identities still start at 0.
    if compiler.result.words.is_empty() {
        compiler.result.words.push(0);
    }
    Ok(compiler.result)
}

fn collect_shape(schema: &ShapeSchema, set: &mut BTreeSet<ShapeSchema>) {
    if !set.insert(schema.clone()) {
        return;
    }
    match schema {
        ShapeSchema::Mapped { inner, .. } | ShapeSchema::Vector(inner) => collect_shape(inner, set),
        ShapeSchema::Choice(variants) => {
            for variant in variants {
                collect_shape(variant, set);
            }
        }
        _ => {}
    }
}
fn collect_material(schema: &MaterialSchema, set: &mut BTreeSet<MaterialSchema>) {
    if !set.insert(schema.clone()) {
        return;
    }
    match schema {
        MaterialSchema::Colored(inner) | MaterialSchema::Emissive(inner) => {
            collect_material(inner, set)
        }
        MaterialSchema::Mixture(children) => {
            for child in children {
                collect_material(child, set);
            }
        }
        _ => {}
    }
}
fn collect_object(
    node: &ObjectNode,
    shapes: &mut BTreeSet<ShapeSchema>,
    materials: &mut BTreeSet<MaterialSchema>,
) {
    match node {
        ObjectNode::Covered { shape, material } => {
            collect_shape(&shape.schema, shapes);
            collect_material(&material.schema, materials);
        }
        ObjectNode::Mapped { inner, .. } => collect_object(inner, shapes, materials),
        ObjectNode::Vector(children) => {
            for child in children {
                collect_object(child, shapes, materials);
            }
        }
        ObjectNode::Tiled {
            shape,
            materials: values,
            border_material,
            ..
        } => {
            collect_shape(&shape.schema, shapes);
            for value in values.iter().chain([border_material]) {
                collect_material(&value.schema, materials);
            }
        }
    }
}
fn validate_geometry(schema: &ShapeSchema, geometry: Geometry) -> Result<()> {
    match schema {
        ShapeSchema::Sphere | ShapeSchema::Cube => ensure!(
            geometry == Geometry::Euclidean,
            "sphere and cube WGSL shapes require Euclidean geometry"
        ),
        ShapeSchema::Horosphere => ensure!(
            geometry == Geometry::Hyperbolic,
            "horosphere requires hyperbolic geometry"
        ),
        ShapeSchema::Mapped {
            geometry: map_geometry,
            inner,
        } => {
            ensure!(
                *map_geometry == geometry,
                "shape map geometry does not match the scene"
            );
            validate_geometry(inner, geometry)?;
        }
        ShapeSchema::Vector(inner) => validate_geometry(inner, geometry)?,
        ShapeSchema::Choice(children) => {
            for child in children {
                validate_geometry(child, geometry)?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn validate_material(schema: &MaterialSchema, words: &[u32]) -> Result<()> {
    ensure!(
        words.len() == schema.word_len(),
        "material payload length mismatch"
    );
    match schema {
        MaterialSchema::Refractive => {
            let value = f32::from_bits(words[0]);
            ensure!(
                value.is_finite() && value > 0.0,
                "refractive index must be finite and positive"
            );
        }
        MaterialSchema::Colored(inner) | MaterialSchema::Emissive(inner) => {
            ensure!(
                words[..3].iter().all(|&x| {
                    let x = f32::from_bits(x);
                    x.is_finite() && x >= 0.0
                }),
                "material color must be finite and nonnegative"
            );
            validate_material(inner, &words[3..])?;
        }
        MaterialSchema::Mixture(children) => {
            let mut total = 0.0f64;
            for &word in &words[..children.len()] {
                let weight = f32::from_bits(word);
                ensure!(
                    weight.is_finite() && weight >= 0.0,
                    "mixture weight must be finite and nonnegative"
                );
                total += f64::from(weight);
            }
            ensure!(total <= 1.00001, "mixture portions exceed one");
            let mut offset = children.len();
            for child in children {
                let end = offset + child.word_len();
                validate_material(child, &words[offset..end])?;
                offset = end;
            }
        }
        _ => {}
    }
    Ok(())
}
fn validate_map_words(words: &[u32], geometry: Geometry) -> Result<()> {
    let values: Vec<_> = words
        .iter()
        .map(|&x| f64::from(f32::from_bits(x)))
        .collect();
    ensure!(
        values.len() == 8 && values.iter().all(|x| x.is_finite()),
        "mapped shape needs eight finite transform words"
    );
    match geometry {
        Geometry::Euclidean => ensure!(
            (values[4..].iter().map(|x| x * x).sum::<f64>() - 1.0).abs() < 1e-4,
            "mapped shape rotation must be a unit quaternion"
        ),
        Geometry::Hyperbolic => {
            let real = values[0] * values[6] - values[1] * values[7] - values[2] * values[4]
                + values[3] * values[5];
            let imag = values[0] * values[7] + values[1] * values[6]
                - values[2] * values[5]
                - values[3] * values[4];
            ensure!(
                real != 0.0 || imag != 0.0,
                "mapped shape becomes singular in f32"
            );
            let magnitude = |a: usize| values[a].hypot(values[a + 1]);
            let roundoff = 8.0
                * f64::from(f32::EPSILON)
                * (magnitude(0) * magnitude(6) + magnitude(2) * magnitude(4));
            ensure!(
                (real - 1.0).abs() <= roundoff && imag.abs() <= roundoff,
                "mapped shape must have determinant one to preserve the upper half-space"
            );
        }
    }
    Ok(())
}
fn validate_shape(schema: &ShapeSchema, words: &[u32]) -> Result<()> {
    match schema {
        ShapeSchema::Plane | ShapeSchema::Sphere | ShapeSchema::Cube | ShapeSchema::Horosphere => {
            ensure!(
                words.len() == 1,
                "parameterless shape must reserve one identity word"
            )
        }
        ShapeSchema::Custom(leaf) => ensure!(
            words.len() == (leaf.parameter_words as usize).max(1),
            "custom shape payload length mismatch"
        ),
        ShapeSchema::Mapped { geometry, inner } => {
            ensure!(words.len() >= 8, "mapped shape payload is truncated");
            validate_map_words(&words[..8], *geometry)?;
            validate_shape(inner, &words[8..])?;
        }
        ShapeSchema::Choice(children) => {
            let index = *words.first().context("shape choice payload is empty")? as usize;
            let child = children
                .get(index)
                .context("shape choice index is outside its variants")?;
            validate_shape(child, &words[1..])?;
        }
        ShapeSchema::Vector(inner) => {
            let count = *words.first().context("shape vector payload is empty")? as usize;
            ensure!(
                count < words.len(),
                "shape vector offset table is truncated"
            );
            if count == 0 {
                ensure!(words.len() == 1, "empty shape vector has excess payload");
            }
            for index in 0..count {
                let start = words[index + 1] as usize;
                let end = if index + 1 < count {
                    words[index + 2] as usize
                } else {
                    words.len()
                };
                ensure!(
                    start > count && start < end && end <= words.len(),
                    "invalid shape vector element range"
                );
                if index == 0 {
                    ensure!(start == 1 + count, "shape vector payload has a gap");
                }
                validate_shape(inner, &words[start..end])?;
            }
        }
    }
    Ok(())
}

impl Compiler {
    fn words(&mut self, words: &[u32]) -> Result<u32> {
        let base = u32::try_from(self.result.words.len())?;
        let length = self
            .result
            .words
            .len()
            .checked_add(words.len())
            .context("scene word count overflow")?;
        ensure!(
            length < u32::MAX as usize,
            "scene word arena exceeds u32 addressing"
        );
        self.result.words.extend_from_slice(words);
        Ok(base)
    }
    fn material(&mut self, value: &MaterialValue) -> Result<u32> {
        validate_material(&value.schema, &value.words)?;
        let index = u32::try_from(self.result.materials.len())?;
        let base = self.words(&value.words)?;
        self.result.materials.push(MaterialRecord {
            data: [self.materials[&value.schema], base, 0, 0],
        });
        Ok(index)
    }
    fn object(&mut self, node: &ObjectNode, map: Transform) -> Result<()> {
        match node {
            ObjectNode::Mapped {
                map: inner,
                inner: object,
            } => self.object(object, map.chain(inner)?),
            ObjectNode::Vector(children) => {
                for child in children {
                    self.object(child, map)?;
                }
                Ok(())
            }
            ObjectNode::Covered { shape, material } => {
                let material = self.material(material)?;
                self.push_object(shape, map, [material, 1, 0], material, [1.0, 0.0])
            }
            ObjectNode::Tiled {
                shape,
                materials,
                border_material,
                tiling,
                cell_size,
                border_width,
            } => {
                ensure!(
                    !materials.is_empty(),
                    "tiled material collection must not be empty"
                );
                ensure!(
                    materials.len() <= i32::MAX as usize,
                    "too many tiled materials"
                );
                let count = materials.len() as u32;
                let first = u32::try_from(self.result.materials.len())?;
                for material in materials {
                    self.material(material)?;
                }
                let border = self.material(border_material)?;
                let width = finite_f32(*border_width)?;
                ensure!(width >= 0.0, "tile border width must be nonnegative");
                let cell = match tiling {
                    Tiling::Square | Tiling::Hexagonal => {
                        ensure!(
                            self.geometry == Geometry::Hyperbolic,
                            "square and hexagonal horosphere tilings require hyperbolic geometry"
                        );
                        let value = finite_f32(*cell_size)?;
                        ensure!(value > 0.0, "tile cell size must be positive");
                        value
                    }
                    Tiling::Pentagonal | Tiling::Pentastar => {
                        ensure!(
                            self.geometry == Geometry::Hyperbolic,
                            "pentagonal tilings require hyperbolic geometry"
                        );
                        1.0
                    }
                    Tiling::Uniform => 1.0,
                };
                self.push_object(
                    shape,
                    map,
                    [first, count, tiling.tag()],
                    border,
                    [cell, width],
                )
            }
        }
    }
    fn push_object(
        &mut self,
        shape: &ShapeValue,
        map: Transform,
        material: [u32; 3],
        border: u32,
        props: [f32; 2],
    ) -> Result<()> {
        validate_shape(&shape.schema, &shape.words)?;
        ensure!(
            self.result.objects.len() < u32::MAX as usize,
            "too many objects"
        );
        let base = self.words(&shape.words)?;
        let [map0, map1] = map.rows()?;
        self.result.objects.push(GpuObject {
            map0,
            map1,
            info: [
                self.shapes[&shape.schema],
                material[0],
                material[1],
                material[2],
            ],
            extra: [border, base, 0, 0],
            props: [props[0], props[1], 0.0, 0.0],
        });
        Ok(())
    }
    fn source(&self) -> Result<String> {
        let mut source = String::from(
            "// Generated from structural schemas; runtime values live in scene_words.\n",
        );
        let mut leaves = BTreeMap::<&str, (&ShaderLeaf, bool)>::new();
        for schema in self.shapes.keys() {
            if let ShapeSchema::Custom(leaf) = schema {
                register_leaf(&mut leaves, leaf, true)?;
            }
        }
        for schema in self.materials.keys() {
            if let MaterialSchema::Custom(leaf) = schema {
                register_leaf(&mut leaves, leaf, false)?;
            }
        }
        let mut entry_points = BTreeSet::new();
        for (leaf, _) in leaves.values() {
            ensure!(
                entry_points.insert(&leaf.entry_point),
                "custom leaf entry point is shared by different keys"
            );
            source.push_str(&leaf.source);
            source.push('\n');
        }
        for (schema, id) in &self.shapes {
            writeln!(
                source,
                "fn ht_shape_{id}(base:u32,ray:Ray,previous_identity:u32)->TaggedHit {{"
            )?;
            match schema {
                ShapeSchema::Plane
                | ShapeSchema::Sphere
                | ShapeSchema::Cube
                | ShapeSchema::Horosphere => {
                    let function = match (schema, self.geometry) {
                        (ShapeSchema::Plane, Geometry::Euclidean) => "eu_plane",
                        (ShapeSchema::Plane, Geometry::Hyperbolic) => "hy_plane",
                        (ShapeSchema::Sphere, _) => "eu_sphere",
                        (ShapeSchema::Cube, _) => "eu_cube",
                        (ShapeSchema::Horosphere, _) => "hy_horosphere",
                        _ => unreachable!(),
                    };
                    writeln!(
                        source,
                        "return TaggedHit({function}(ray,base==previous_identity),base);"
                    )?;
                }
                ShapeSchema::Custom(leaf) => writeln!(
                    source,
                    "let result = {}(base,ray,previous_identity); return TaggedHit(result.hit,base);",
                    leaf.entry_point
                )?,
                ShapeSchema::Mapped { geometry, inner } => {
                    let child = self.shapes[inner.as_ref()];
                    let hy = *geometry == Geometry::Hyperbolic;
                    writeln!(
                        source,
                        "let map0=load_vec4(base); let map1=load_vec4(base+4u);\nlet result=ht_shape_{child}(base+8u,shape_ray_to_local(map0,map1,{hy},ray),previous_identity);\nreturn TaggedHit(shape_hit_to_parent(map0,map1,{hy},result.hit),result.identity);"
                    )?;
                }
                ShapeSchema::Vector(inner) => {
                    let child = self.shapes[inner.as_ref()];
                    writeln!(
                        source,
                        "var result=TaggedHit(miss(),0xffffffffu);\nfor(var index=0u;index<load_u32(base);index+=1u) {{\nlet candidate=ht_shape_{child}(base+load_u32(base+1u+index),ray,previous_identity);\nif candidate.hit.valid!=0u {{ if result.hit.valid==0u || candidate.hit.distance<result.hit.distance {{result=candidate;}} }}\n}}\nreturn result;"
                    )?;
                }
                ShapeSchema::Choice(children) => {
                    writeln!(source, "switch load_u32(base) {{")?;
                    for (variant, child) in children.iter().enumerate() {
                        writeln!(
                            source,
                            "case {variant}u: {{return ht_shape_{}(base+1u,ray,previous_identity);}}",
                            self.shapes[child]
                        )?;
                    }
                    writeln!(
                        source,
                        "default: {{return TaggedHit(miss(),0xffffffffu);}}\n}}"
                    )?;
                }
            }
            source.push_str("}\n");
        }
        for (schema, id) in &self.materials {
            writeln!(
                source,
                "fn ht_material_{id}(base:u32,ctx:MaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {{"
            )?;
            match schema {
                MaterialSchema::Absorbing => source.push_str("(*sample).alive=0u;\n"),
                MaterialSchema::Transparent => {},
                MaterialSchema::Lambertian => source.push_str("var normal=ctx.normal;\nif dot((*sample).direction,normal)>0 {normal=-normal;}\nlet phi=2*PI*uniform_random(rng);\nlet square_cosine=uniform_random(rng);\nlet sine=sqrt(max(0,1-square_cosine));\nlet local=vec3<f32>(cos(phi)*sine,sin(phi)*sine,sqrt(square_cosine));\n(*sample).direction=eu_rotate(rotation_look(-normal),local);\n"),
                MaterialSchema::Specular => source.push_str("(*sample).direction-=2*dot((*sample).direction,ctx.normal)*ctx.normal;\n"),
                MaterialSchema::Refractive => source.push_str("let direction=(*sample).direction;\nvar ratio=load_f32(base);\nlet a=dot(direction,ctx.normal);\nif a < -EPS {ratio=1/ratio;}\nlet x=(a*a-1)*(ratio*ratio)+1;\nif x>EPS {(*sample).direction=direction*ratio+(sign(a)*sqrt(x)-a*ratio)*ctx.normal;}\nelse {(*sample).direction=direction-2*a*ctx.normal;}\n"),
                MaterialSchema::Colored(inner) => writeln!(source,"(*sample).attenuation*=load_vec3(base);\nht_material_{}(base+3u,ctx,sample,rng);",self.materials[inner.as_ref()])?,
                MaterialSchema::Emissive(inner) => writeln!(source,"(*sample).emission+=(*sample).attenuation*load_vec3(base);\nht_material_{}(base+3u,ctx,sample,rng);",self.materials[inner.as_ref()])?,
                MaterialSchema::Mixture(children) => {
                    source.push_str("var choice=uniform_random(rng);\n");
                    let mut offset=children.len();
                    for (index,child) in children.iter().enumerate() {
                        writeln!(source,"choice-=load_f32(base+{index}u);\nif choice<0 {{ht_material_{}(base+{offset}u,ctx,sample,rng);return;}}",self.materials[child])?;
                        offset+=child.word_len();
                    }
                    source.push_str("(*sample).alive=0u;\n");
                },
                MaterialSchema::Custom(leaf) => writeln!(source,"{}(base,ctx,sample,rng);",leaf.entry_point)?,
            }
            source.push_str("}\n");
        }
        source.push_str("fn ht_shape_dispatch(kind:u32,base:u32,ray:Ray,previous_identity:u32)->TaggedHit {\nswitch kind {\n");
        for id in self.shapes.values() {
            writeln!(
                source,
                "case {id}u: {{return ht_shape_{id}(base,ray,previous_identity);}}"
            )?;
        }
        source.push_str("default: {return TaggedHit(miss(),0xffffffffu);}\n}}\n");
        source.push_str("fn ht_material_dispatch(kind:u32,base:u32,ctx:MaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {\nswitch kind {\n");
        for id in self.materials.values() {
            writeln!(
                source,
                "case {id}u: {{ht_material_{id}(base,ctx,sample,rng);}}"
            )?;
        }
        source.push_str("default: {(*sample).alive=0u;}\n}}\n");
        Ok(source)
    }
}
fn register_leaf<'a>(
    leaves: &mut BTreeMap<&'a str, (&'a ShaderLeaf, bool)>,
    leaf: &'a ShaderLeaf,
    shape: bool,
) -> Result<()> {
    ensure!(!leaf.key.is_empty(), "custom shader key must not be empty");
    let mut chars = leaf.entry_point.chars();
    ensure!(
        chars
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
            && chars.all(|c| c.is_ascii_alphanumeric() || c == '_'),
        "custom shader entry point is not an identifier"
    );
    if let Some(&(previous, previous_shape)) = leaves.get(leaf.key.as_str()) {
        ensure!(
            previous == leaf && previous_shape == shape,
            "custom shader key has conflicting implementations or signatures: {}",
            leaf.key
        );
    } else {
        leaves.insert(&leaf.key, (leaf, shape));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Background, View};

    fn scene(shape: ShapeValue, material: MaterialValue) -> SceneDefinition {
        SceneDefinition {
            view: View {
                map: Transform::identity(Geometry::Euclidean),
                fov: 1.0,
            },
            background: Background::Constant([0.0; 3]),
            bounces: 4,
            object: ObjectNode::Covered { shape, material },
            material_schemas: vec![],
            shape_schemas: vec![],
        }
    }

    #[test]
    fn storage_records_have_exact_word_layout() {
        assert_eq!(std::mem::size_of::<GpuObject>(), 80);
        assert_eq!(std::mem::size_of::<MaterialRecord>(), 16);
        assert_eq!(std::mem::offset_of!(GpuObject, info), 32);
        assert_eq!(std::mem::offset_of!(GpuObject, extra), 48);
        assert_eq!(std::mem::offset_of!(GpuObject, props), 64);
    }

    #[test]
    fn values_and_vector_lengths_do_not_change_shader_source() -> Result<()> {
        let a = scene(
            ShapeValue::vector(ShapeSchema::Plane, vec![])?,
            MaterialValue::absorbing().emissive([1.0; 3])?,
        );
        let b = scene(
            ShapeValue::vector(
                ShapeSchema::Plane,
                vec![ShapeValue::plane(), ShapeValue::plane()],
            )?,
            MaterialValue::absorbing().emissive([2.0, 3.0, 4.0])?,
        );
        let mut c = b.clone();
        c.object = ObjectNode::Vector(vec![b.object.clone(), b.object.clone()]);
        let (a, b, c) = (compile(&a)?, compile(&b)?, compile(&c)?);
        assert_eq!(a.source, b.source);
        assert_eq!(b.source, c.source);
        assert_ne!(a.words, b.words);
        assert_eq!(c.objects.len(), 2);
        assert_ne!(c.objects[0].extra[1], c.objects[1].extra[1]);
        Ok(())
    }

    #[test]
    fn malformed_public_payloads_are_rejected() -> Result<()> {
        let mut value = scene(
            ShapeValue::vector(ShapeSchema::Plane, vec![ShapeValue::plane()])?,
            MaterialValue::transparent(),
        );
        let ObjectNode::Covered { shape, .. } = &mut value.object else {
            unreachable!()
        };
        shape.words[1] = u32::MAX;
        assert!(compile(&value).is_err());
        let invalid = MaterialValue {
            schema: MaterialSchema::Colored(Box::new(MaterialSchema::Absorbing)),
            words: vec![0, 0],
        };
        assert!(compile(&scene(ShapeValue::plane(), invalid)).is_err());
        let mut mapped = ShapeValue::plane().mapped(Transform::identity(Geometry::Euclidean))?;
        mapped.words[4] = 0;
        assert!(compile(&scene(mapped, MaterialValue::transparent())).is_err());
        let mut hyperbolic = scene(ShapeValue::plane(), MaterialValue::transparent());
        hyperbolic.view.map = Transform::identity(Geometry::Hyperbolic);
        hyperbolic.object = ObjectNode::Covered {
            shape: ShapeValue {
                schema: ShapeSchema::Mapped {
                    geometry: Geometry::Hyperbolic,
                    inner: Box::new(ShapeSchema::Plane),
                },
                // diag(i,i) is nonsingular but does not preserve z > 0 under
                // the renderer's normalized SL(2,C) quaternion action.
                words: [0.0f32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]
                    .map(f32::to_bits)
                    .into(),
            },
            material: MaterialValue::transparent(),
        };
        assert!(compile(&hyperbolic).is_err());
        Ok(())
    }

    #[test]
    fn a_custom_key_cannot_name_two_implementations() -> Result<()> {
        let leaf = ShaderLeaf {
            key: "example.glow".into(),
            entry_point: "example_glow".into(),
            source: "fn example_glow() {}".into(),
            parameter_words: 0,
        };
        let material = MaterialValue::custom(leaf.clone(), vec![])?;
        let mut value = scene(ShapeValue::plane(), material);
        let mut conflicting = leaf;
        conflicting.source.push_str("// Different implementation");
        value
            .material_schemas
            .push(MaterialSchema::Custom(conflicting));
        assert!(compile(&value).is_err());
        Ok(())
    }
}
