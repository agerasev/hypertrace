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
    /// Validated physical radius used by primitive compilation.
    pub radius: f32,
    /// Canonical f64 isometries; GPU rows are prepared relative to the camera.
    pub transforms: Vec<Transform>,
    pub objects: Vec<GpuObject>,
    pub materials: Vec<MaterialRecord>,
    pub words: Vec<u32>,
    /// Functions only. Concatenate after the shared math and generated runtime.
    /// Values, vector lengths and object counts do not occur in this string.
    pub source: String,
}

struct Compiler {
    geometry: Geometry,
    radius: f32,
    shapes: BTreeMap<ShapeSchema, u32>,
    materials: BTreeMap<MaterialSchema, u32>,
    result: CompiledScene,
}

/// Compile the active values and all registered alternatives. Registration makes
/// changing a choice variant or adding vector elements a buffer-only update.
pub fn compile(scene: &SceneDefinition) -> Result<CompiledScene> {
    let geometry = scene.view.map.geometry();
    let radius = validate_scene_parameters(scene, geometry)?;
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
        radius,
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
            radius,
            transforms: vec![],
            objects: vec![],
            materials: vec![],
            words: vec![],
            source: String::new(),
        },
    };
    for schema in compiler.shapes.keys() {
        validate_geometry(schema, geometry, radius)?;
    }
    for schema in compiler.materials.keys() {
        ensure!(
            geometry != Geometry::Spherical || !matches!(schema, MaterialSchema::Custom(_)),
            "legacy material leaves have no spherical chart; use embedded_custom with GeoMaterialContext"
        );
    }
    compiler.result.source = compiler.source()?;
    compiler.object(&scene.object, Transform::identity(geometry))?;
    let inverse_camera = scene.view.map.inverse()?;
    for map in &compiler.result.transforms {
        inverse_camera
            .chain(map)?
            .rows()
            .context("camera-relative object transform cannot be represented safely")?;
    }
    // wgpu storage bindings cannot be empty. Leaf identities still start at 0.
    if compiler.result.words.is_empty() {
        compiler.result.words.push(0);
    }
    Ok(compiler.result)
}

fn validate_scene_parameters(scene: &SceneDefinition, geometry: Geometry) -> Result<f32> {
    let radius = finite_f32(scene.radius)?;
    ensure!(
        radius.is_normal()
            && radius > 0.0
            && (geometry != Geometry::Euclidean || scene.radius == 1.0),
        "curvature radius must be positive and normal in f32; Euclidean radius must be one"
    );
    if geometry == Geometry::Spherical {
        ensure!(
            (radius * std::f32::consts::TAU).is_finite(),
            "spherical circumference is outside finite f32 range"
        );
    }
    ensure!(
        finite_f32(scene.view.fov)? > 0.0,
        "camera field of view must be positive"
    );
    scene.medium.validate_for_radius(radius)?;
    scene.view.map.components()?;
    Ok(radius)
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
fn validate_geometry(schema: &ShapeSchema, geometry: Geometry, radius: f32) -> Result<()> {
    match schema {
        ShapeSchema::Sphere => validate_sphere_radius(1.0, geometry, radius)?,
        ShapeSchema::Cube => ensure!(
            geometry == Geometry::Euclidean,
            "cube WGSL shape requires Euclidean geometry"
        ),
        ShapeSchema::Custom(_) => ensure!(
            geometry != Geometry::Spherical,
            "legacy shape leaves have no spherical chart; use embedded_custom with GeoRay and GeoTaggedHit"
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
            validate_geometry(inner, geometry, radius)?;
        }
        ShapeSchema::Vector(inner) => validate_geometry(inner, geometry, radius)?,
        ShapeSchema::Choice(children) => {
            for child in children {
                validate_geometry(child, geometry, radius)?;
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
    crate::validate_embedded_rows(
        std::array::from_fn(|r| std::array::from_fn(|c| values[4 * r + c] as f32)),
        geometry,
    )?;
    Ok(())
}
fn validate_sphere_radius(radius: f32, geometry: Geometry, space_radius: f32) -> Result<()> {
    ensure!(
        radius.is_normal() && radius > 0.0,
        "geodesic sphere needs a finite positive normal radius"
    );
    let angle = radius / space_radius;
    // Match geo_classify_section_discriminant's coefficient uncertainty at
    // the sphere center; do not admit a radius already inside that zero band.
    let resolved =
        |sine: f32, cosine: f32| sine * sine > 8.0 * f32::EPSILON * (1.0 + cosine * cosine);
    match geometry {
        Geometry::Spherical => {
            ensure!(
                radius < std::f32::consts::PI * space_radius,
                "spherical sphere radius must be strictly below pi times the curvature radius"
            );
            let (sine, cosine) = angle.sin_cos();
            ensure!(
                angle.is_finite()
                    && cosine.abs() < 1.0
                    && sine.is_normal()
                    && sine > 0.0
                    && resolved(sine, cosine),
                "spherical sphere radius is outside the f32 section resolver range: cos(radius/space_radius) must be distinct from both 1 and -1 and resolve the coefficient uncertainty band"
            );
        }
        Geometry::Hyperbolic => {
            let cosine = angle.cosh();
            let sine = angle.sinh();
            ensure!(
                angle.is_finite()
                    && (cosine * cosine).is_finite()
                    && cosine > 1.0
                    && sine.is_normal()
                    && sine > 0.0
                    && resolved(sine, cosine),
                "hyperbolic sphere radius is outside the f32 section resolver range: cosh(radius/space_radius) must be greater than one with a finite square and resolve the coefficient uncertainty band"
            );
        }
        Geometry::Euclidean => ensure!(
            (radius * radius).is_finite(),
            "Euclidean sphere squared radius is outside f32 range"
        ),
    }
    Ok(())
}

fn validate_shape(
    schema: &ShapeSchema,
    words: &[u32],
    scene_geometry: Geometry,
    space_radius: f32,
) -> Result<()> {
    match schema {
        ShapeSchema::Plane | ShapeSchema::Cube | ShapeSchema::Horosphere => {
            ensure!(
                words.len() == 1,
                "parameterless shape must reserve one identity word"
            )
        }
        ShapeSchema::Sphere | ShapeSchema::GeodesicSphere => {
            ensure!(words.len() == 1, "sphere payload must have one word");
            let radius = if matches!(schema, ShapeSchema::Sphere) {
                1.0
            } else {
                f32::from_bits(words[0])
            };
            validate_sphere_radius(radius, scene_geometry, space_radius)?;
        }
        ShapeSchema::Custom(leaf) | ShapeSchema::EmbeddedCustom(leaf) => ensure!(
            words.len() == (leaf.parameter_words as usize).max(1),
            "custom shape payload length mismatch"
        ),
        ShapeSchema::Mapped { geometry, inner } => {
            ensure!(words.len() >= 8, "mapped shape payload is truncated");
            validate_map_words(&words[..8], *geometry)?;
            validate_shape(inner, &words[8..], scene_geometry, space_radius)?;
        }
        ShapeSchema::Choice(children) => {
            let index = *words.first().context("shape choice payload is empty")? as usize;
            let child = children
                .get(index)
                .context("shape choice index is outside its variants")?;
            validate_shape(child, &words[1..], scene_geometry, space_radius)?;
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
                validate_shape(inner, &words[start..end], scene_geometry, space_radius)?;
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
                    self.geometry != Geometry::Spherical || *tiling == Tiling::Uniform,
                    "spherical space supports only uniform tiling"
                );
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
        validate_shape(&shape.schema, &shape.words, self.geometry, self.radius)?;
        ensure!(
            self.result.objects.len() < u32::MAX as usize,
            "too many objects"
        );
        let base = self.words(&shape.words)?;
        map.components()?;
        self.result.transforms.push(map);
        let [map0, map1] = Transform::identity(self.geometry).rows()?;
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
        let mut source = format!(
            "// Embedded shader contract v2; parameters live in scene_words.\nconst GEO_K:f32 = {}.0;\n",
            self.geometry.sign()
        );
        let mut leaves = BTreeMap::<&str, (&ShaderLeaf, u8)>::new();
        for schema in self.shapes.keys() {
            match schema {
                ShapeSchema::Custom(leaf) => register_leaf(&mut leaves, leaf, 0)?,
                ShapeSchema::EmbeddedCustom(leaf) => register_leaf(&mut leaves, leaf, 2)?,
                _ => {}
            }
        }
        for schema in self.materials.keys() {
            match schema {
                MaterialSchema::Custom(leaf) => register_leaf(&mut leaves, leaf, 1)?,
                MaterialSchema::EmbeddedCustom(leaf) => register_leaf(&mut leaves, leaf, 3)?,
                _ => {}
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
                "fn ht_shape_{id}(base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {{"
            )?;
            match schema {
                ShapeSchema::Plane
                | ShapeSchema::Sphere
                | ShapeSchema::GeodesicSphere
                | ShapeSchema::Cube
                | ShapeSchema::Horosphere => {
                    let function = match schema {
                        ShapeSchema::Plane => "geo_plane",
                        ShapeSchema::Sphere | ShapeSchema::GeodesicSphere => "geo_sphere",
                        ShapeSchema::Cube => "geo_cube",
                        ShapeSchema::Horosphere => "geo_horosphere",
                        _ => unreachable!(),
                    };
                    let sphere_radius = match schema {
                        ShapeSchema::Sphere => ",1.0",
                        ShapeSchema::GeodesicSphere => ",load_f32(base)",
                        _ => "",
                    };
                    writeln!(
                        source,
                        "return GeoTaggedHit({function}(ray,select(0.0,8.0*EPS*params.misc.y,base==previous_identity),geo_infinity(),params.misc.y{sphere_radius}),base);"
                    )?;
                }
                ShapeSchema::Custom(leaf) => writeln!(
                    source,
                    "let result = {}(base,geo_legacy_ray(ray),previous_identity); return GeoTaggedHit(geo_from_legacy_hit(result.hit),base);",
                    leaf.entry_point
                )?,
                ShapeSchema::EmbeddedCustom(leaf) => writeln!(
                    source,
                    "let result = {}(base,ray,previous_identity); return GeoTaggedHit(result.hit,base);",
                    leaf.entry_point
                )?,
                ShapeSchema::Mapped { geometry, inner } => {
                    let child = self.shapes[inner.as_ref()];
                    let _ = geometry;
                    writeln!(
                        source,
                        "let map=GeoMap(load_vec4(base),load_vec4(base+4u));\nlet local=geo_map_ray(geo_inverse(map),ray);\nif !geo_ray_supported(local) {{return GeoTaggedHit(geo_failure(),base);}}\nlet result=ht_shape_{child}(base+8u,local,previous_identity);\nreturn GeoTaggedHit(geo_map_hit(map,result.hit),result.identity);"
                    )?;
                }
                ShapeSchema::Vector(inner) => {
                    let child = self.shapes[inner.as_ref()];
                    writeln!(
                        source,
                        "var result=GeoTaggedHit(geo_miss(),0xffffffffu);\nfor(var index=0u;index<load_u32(base);index+=1u) {{\nlet candidate=ht_shape_{child}(base+load_u32(base+1u+index),ray,previous_identity);\nif candidate.hit.valid==2u {{return candidate;}}\nif candidate.hit.valid!=0u {{ if result.hit.valid==0u || candidate.hit.distance<result.hit.distance {{result=candidate;}} }}\n}}\nreturn result;"
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
                        "default: {{return GeoTaggedHit(geo_miss(),0xffffffffu);}}\n}}"
                    )?;
                }
            }
            source.push_str("}\n");
        }
        for (schema, id) in &self.materials {
            writeln!(
                source,
                "fn ht_material_{id}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {{"
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
                MaterialSchema::Custom(leaf) => writeln!(source,"let legacy_ctx=MaterialContext(geo_to_chart_pos(ctx.position),geo_to_chart_dir(ctx.position,geo_from_local(ctx.position,ctx.normal)));\n(*sample).direction=geo_to_chart_dir(ctx.position,geo_from_local(ctx.position,(*sample).direction));\n{}(base,legacy_ctx,sample,rng);\n(*sample).direction=geo_to_local(ctx.position,geo_from_chart_dir(ctx.position,(*sample).direction));",leaf.entry_point)?,
                MaterialSchema::EmbeddedCustom(leaf) => writeln!(source,"{}(base,ctx,sample,rng);",leaf.entry_point)?,
            }
            source.push_str("}\n");
        }
        source.push_str("fn ht_shape_dispatch(kind:u32,base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {\nswitch kind {\n");
        for id in self.shapes.values() {
            writeln!(
                source,
                "case {id}u: {{return ht_shape_{id}(base,ray,previous_identity);}}"
            )?;
        }
        source.push_str("default: {return GeoTaggedHit(geo_miss(),0xffffffffu);}\n}}\n");
        source.push_str("fn ht_material_dispatch(kind:u32,base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {\nswitch kind {\n");
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
    leaves: &mut BTreeMap<&'a str, (&'a ShaderLeaf, u8)>,
    leaf: &'a ShaderLeaf,
    shape: u8,
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
            radius: 1.0,
            medium: Default::default(),
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
        mapped.words[0] = 0;
        assert!(compile(&scene(mapped, MaterialValue::transparent())).is_err());
        let mut hyperbolic = scene(ShapeValue::plane(), MaterialValue::transparent());
        hyperbolic.view.map = Transform::identity(Geometry::Hyperbolic);
        hyperbolic.object = ObjectNode::Covered {
            shape: ShapeValue {
                schema: ShapeSchema::Mapped {
                    geometry: Geometry::Hyperbolic,
                    inner: Box::new(ShapeSchema::Plane),
                },
                // Equal real/unreal quaternion norms make this embedded
                // hyperbolic map singular, despite finite components.
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
    fn curved_scene(geometry: Geometry, radius: f64, shape: ShapeValue) -> SceneDefinition {
        let mut value = scene(shape, MaterialValue::transparent());
        value.view.map = Transform::identity(geometry);
        value.radius = radius;
        value
    }

    #[test]
    fn spherical_sphere_radii_are_validated_through_nested_payloads() -> Result<()> {
        let radius = 2.0;
        let valid = ShapeValue::geodesic_sphere(1.8 * radius)?;
        let nested = ShapeValue::vector(valid.schema.clone(), vec![valid])?
            .mapped(Transform::identity(Geometry::Spherical))?;
        compile(&curved_scene(Geometry::Spherical, radius, nested))?;
        for shape_radius in [std::f64::consts::PI * radius, 4.0 * radius] {
            let shape = ShapeValue::geodesic_sphere(shape_radius)?;
            let nested = ShapeValue::choice(vec![shape.schema.clone()], 0, shape)?;
            let error = compile(&curved_scene(Geometry::Spherical, radius, nested)).unwrap_err();
            assert!(error.to_string().contains("strictly below pi"));
        }
        assert!(
            compile(&curved_scene(
                Geometry::Spherical,
                0.25,
                ShapeValue::sphere()
            ))
            .is_err()
        );
        // Parameterless sphere semantics also apply to inactive registered leaves.
        let mut inactive = curved_scene(Geometry::Spherical, 0.25, ShapeValue::plane());
        inactive.shape_schemas.push(ShapeSchema::Sphere);
        assert!(compile(&inactive).is_err());
        let mut malformed = ShapeValue::geodesic_sphere(1.0)?;
        malformed.words[0] = f32::NAN.to_bits();
        assert!(compile(&curved_scene(Geometry::Spherical, radius, malformed)).is_err());
        Ok(())
    }

    #[test]
    fn invalid_physical_parameters_fail_before_shader_generation() {
        for geometry in [
            Geometry::Euclidean,
            Geometry::Hyperbolic,
            Geometry::Spherical,
        ] {
            for radius in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::MIN_POSITIVE] {
                assert!(compile(&curved_scene(geometry, radius, ShapeValue::plane())).is_err());
            }
            for fov in [0.0, -1.0, f64::NAN, f64::INFINITY] {
                let mut value = curved_scene(geometry, 1.0, ShapeValue::plane());
                value.view.fov = fov;
                assert!(compile(&value).is_err());
            }
            for (extinction, albedo) in [
                (-1.0, [1.0; 3]),
                (f32::NAN, [1.0; 3]),
                (0.1, [-0.1, 0.0, 0.0]),
                (0.1, [1.1, 0.0, 0.0]),
                (f32::from_bits(1), [1.0; 3]),
            ] {
                let mut value = curved_scene(geometry, 1.0, ShapeValue::plane());
                value.medium = crate::Medium::Homogeneous { extinction, albedo };
                assert!(compile(&value).is_err());
            }
        }
        assert!(compile(&curved_scene(Geometry::Euclidean, 2.0, ShapeValue::plane())).is_err());
        assert!(
            compile(&curved_scene(
                Geometry::Spherical,
                f64::from(f32::MAX),
                ShapeValue::plane()
            ))
            .is_err()
        );
    }

    fn test_leaf(key: &str) -> ShaderLeaf {
        ShaderLeaf {
            key: key.into(),
            entry_point: key.into(),
            source: format!("fn {key}() {{}}"),
            parameter_words: 0,
        }
    }
    #[test]
    fn embedded_custom_contract_supports_all_signs_and_legacy_rejects_spherical() -> Result<()> {
        for geometry in [
            Geometry::Euclidean,
            Geometry::Hyperbolic,
            Geometry::Spherical,
        ] {
            let shape = ShapeValue::embedded_custom(test_leaf("test_shape"), vec![])?;
            let material = MaterialValue::embedded_custom(test_leaf("test_material"), vec![])?;
            let mut value = curved_scene(geometry, 1.0, shape);
            value.object = ObjectNode::Covered {
                shape: ShapeValue::embedded_custom(test_leaf("test_shape"), vec![])?,
                material,
            };
            let compiled = compile(&value)?;
            assert!(compiled.source.contains("Embedded shader contract v2"));
        }
        let old_shape = ShapeValue::custom(test_leaf("legacy_shape"), vec![])?;
        let error = compile(&curved_scene(Geometry::Spherical, 1.0, old_shape)).unwrap_err();
        assert!(error.to_string().contains("legacy shape leaves"));
        let mut old_material = curved_scene(Geometry::Spherical, 1.0, ShapeValue::plane());
        old_material
            .material_schemas
            .push(MaterialSchema::Colored(Box::new(MaterialSchema::Custom(
                test_leaf("legacy_material"),
            ))));
        let error = compile(&old_material).unwrap_err();
        assert!(error.to_string().contains("legacy material leaves"));
        Ok(())
    }

    #[test]
    fn object_geometry_and_relative_precision_are_validated_on_cpu() -> Result<()> {
        use ccgeom::{Geometry3, Hyperbolic3};
        let mut value = curved_scene(Geometry::Hyperbolic, 1.0, ShapeValue::plane());
        value.view.map = Transform::Hyperbolic(Hyperbolic3::shift_z(12.0));
        value.object = ObjectNode::Mapped {
            map: Transform::Hyperbolic(Hyperbolic3::shift_z(12.2)),
            inner: Box::new(value.object),
        };
        let compiled = compile(&value)?;
        assert_eq!(compiled.radius, 1.0);
        assert_eq!(compiled.transforms.len(), 1);
        value.view.map = Transform::identity(Geometry::Hyperbolic);
        assert!(compile(&value).is_err());
        let mut mismatch = curved_scene(Geometry::Spherical, 1.0, ShapeValue::plane());
        mismatch.object = ObjectNode::Mapped {
            map: Transform::identity(Geometry::Euclidean),
            inner: Box::new(mismatch.object),
        };
        assert!(compile(&mismatch).is_err());
        let mismatched_shape =
            ShapeValue::plane().mapped(Transform::identity(Geometry::Hyperbolic))?;
        assert!(compile(&curved_scene(Geometry::Spherical, 1.0, mismatched_shape)).is_err());
        Ok(())
    }

    #[test]
    fn spherical_tilings_require_explicitly_uniform_semantics() -> Result<()> {
        for tiling in [
            Tiling::Uniform,
            Tiling::Square,
            Tiling::Hexagonal,
            Tiling::Pentagonal,
            Tiling::Pentastar,
        ] {
            let mut value = curved_scene(Geometry::Spherical, 1.0, ShapeValue::plane());
            value.object = ObjectNode::Tiled {
                shape: ShapeValue::plane(),
                materials: vec![MaterialValue::transparent()],
                border_material: MaterialValue::absorbing(),
                tiling,
                cell_size: 1.0,
                border_width: 0.01,
            };
            assert_eq!(compile(&value).is_ok(), tiling == Tiling::Uniform);
        }
        Ok(())
    }

    #[test]
    fn physical_parameters_update_payloads_without_changing_shader_structure() -> Result<()> {
        let mut a = curved_scene(Geometry::Spherical, 1.0, ShapeValue::geodesic_sphere(0.3)?);
        let mut b = curved_scene(Geometry::Spherical, 2.0, ShapeValue::geodesic_sphere(0.7)?);
        a.medium = crate::Medium::Homogeneous {
            extinction: 0.1,
            albedo: [0.7; 3],
        };
        b.medium = crate::Medium::Homogeneous {
            extinction: 0.3,
            albedo: [0.2; 3],
        };
        let (a, b) = (compile(&a)?, compile(&b)?);
        assert_eq!(a.source, b.source);
        assert_ne!(a.radius, b.radius);
        assert_ne!(a.words, b.words);
        Ok(())
    }
    #[test]
    fn curved_spheres_reject_unresolvable_f32_sections() -> Result<()> {
        for (geometry, radius) in [
            (Geometry::Spherical, 1e-5),
            (
                Geometry::Spherical,
                f64::from(std::f32::consts::PI - f32::EPSILON),
            ),
            (Geometry::Hyperbolic, 1e-5),
            (Geometry::Spherical, 0.001),
            (Geometry::Hyperbolic, 0.001),
            (Geometry::Hyperbolic, 100.0),
        ] {
            let shape = ShapeValue::geodesic_sphere(radius)?;
            let error = compile(&curved_scene(geometry, 1.0, shape)).unwrap_err();
            assert!(
                error.to_string().contains("f32 section resolver range"),
                "{error}"
            );
        }
        for geometry in [Geometry::Spherical, Geometry::Hyperbolic] {
            let error = compile(&curved_scene(geometry, 1e6, ShapeValue::sphere())).unwrap_err();
            assert!(error.to_string().contains("f32 section resolver range"));
            compile(&curved_scene(
                geometry,
                1.0,
                ShapeValue::geodesic_sphere(0.002)?,
            ))?;
        }
        Ok(())
    }
}
