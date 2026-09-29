//! Link component-owned shader modules and pack instance values for the GPU.
use crate::{
    Geometry, GeometryContext, MaterialValue, ObjectNode, Result, SceneDefinition, ShaderKind,
    ShaderModule, ShapeValue, Transform, finite_f32,
};
use anyhow::{Context as _, ensure};
use bytemuck::{Pod, Zeroable};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

/// Object transforms remain in the material's coordinate frame. Mapped shapes
/// are generated independently and transform their hit back into that frame.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GpuObject {
    pub map0: [f32; 4],
    pub map1: [f32; 4],
    /// Shape function, material record, shape word offset, reserved.
    pub info: [u32; 4],
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

/// A linked module graph. All identifiers and ordering depend only on structure.
struct Linker<'a> {
    modules: BTreeMap<String, &'a ShaderModule>,
    visiting: BTreeSet<String>,
}
impl<'a> Linker<'a> {
    fn collect(&mut self, module: &'a ShaderModule, context: GeometryContext) -> Result<()> {
        ensure!(
            !module.key.is_empty(),
            "shader module key must not be empty"
        );
        ensure!(
            !self.visiting.contains(&module.key),
            "shader dependency cycle at {}",
            module.key
        );
        module
            .validate_geometry(context)
            .with_context(|| format!("shader module {}", module.key))?;
        if let Some(previous) = self.modules.get(&module.key) {
            ensure!(
                previous.same_implementation(module),
                "shader module key has conflicting implementations: {}",
                module.key
            );
        }
        // Equal shader structure does not imply equal validation callbacks.
        // Validate every dependency instance, including those of a duplicate
        // parent, so acceptance cannot depend on object/module traversal order.
        self.visiting.insert(module.key.clone());
        for child in &module.dependencies {
            self.collect(child, context)?;
        }
        self.visiting.remove(&module.key);
        self.modules.entry(module.key.clone()).or_insert(module);
        Ok(())
    }
    fn object(&mut self, node: &'a ObjectNode, context: GeometryContext) -> Result<()> {
        match node {
            ObjectNode::Covered { shape, material } => {
                ensure!(
                    shape.schema.kind == ShaderKind::Shape,
                    "expected a shape module"
                );
                ensure!(
                    material.schema.kind == ShaderKind::Material,
                    "expected a material module"
                );
                self.collect(&shape.schema, context)?;
                self.collect(&material.schema, context)?;
            }
            ObjectNode::Mapped { inner, .. } => self.object(inner, context)?,
            ObjectNode::Vector(children) => {
                for child in children {
                    self.object(child, context)?;
                }
            }
        }
        Ok(())
    }
    fn link(&self, geometry: Geometry) -> Result<(String, BTreeMap<String, u32>)> {
        let ids = self
            .modules
            .keys()
            .enumerate()
            .map(|(i, key)| Ok((key.clone(), u32::try_from(i)?)))
            .collect::<Result<BTreeMap<_, _>>>()?;
        let mut source = format!(
            "// Component shader interface: embedded geometry.\nconst GEO_K:f32 = {}.0;\n",
            geometry.sign()
        );
        for (key, module) in &self.modules {
            let prefix = format!("ht_module_{}", ids[key]);
            ensure!(
                module.source.contains("{{self}}"),
                "module {} source must use its {{{{self}}}} namespace",
                key
            );
            let mut body = module.source.replace("{{self}}", &prefix);
            for (i, dep) in module.dependencies.iter().enumerate() {
                body = body.replace(
                    &format!("{{{{dep{i}}}}}"),
                    &format!("ht_module_{}", ids[&dep.key]),
                );
            }
            ensure!(
                !body.contains("{{"),
                "module {} has an unresolved shader placeholder",
                key
            );
            // Debug formatting escapes newlines in user keys, preserving this comment.
            writeln!(source, "// module {key:?}\n{body}")?;
        }
        source.push_str("fn ht_shape_dispatch(kind:u32,base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {\nswitch kind {\n");
        for (key, module) in &self.modules {
            if module.kind == ShaderKind::Shape {
                let id = ids[key];
                writeln!(
                    source,
                    "case {id}u: {{return ht_module_{id}(base,ray,previous_identity);}}"
                )?;
            }
        }
        source.push_str("default: {return GeoTaggedHit(geo_miss(),0xffffffffu);}\n}}\n");
        source.push_str("fn ht_material_dispatch(kind:u32,base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {\nswitch kind {\n");
        for (key, module) in &self.modules {
            if module.kind == ShaderKind::Material {
                let id = ids[key];
                writeln!(
                    source,
                    "case {id}u: {{ht_module_{id}(base,ctx,sample,rng);}}"
                )?;
            }
        }
        source.push_str("default: {(*sample).alive=0u;}\n}}\n");
        Ok((source, ids))
    }
}

/// Compile type dependencies and current values without a device or global registry.
pub fn compile(scene: &SceneDefinition) -> Result<CompiledScene> {
    let geometry = scene.view.map.geometry();
    let radius = validate_scene_parameters(scene, geometry)?;
    let context = GeometryContext { geometry, radius };
    let mut linker = Linker {
        modules: BTreeMap::new(),
        visiting: BTreeSet::new(),
    };
    for module in &scene.modules {
        linker.collect(module, context)?;
    }
    linker.object(&scene.object, context)?;
    let (source, ids) = linker.link(geometry)?;
    let mut compiler = Compiler {
        context,
        ids,
        result: CompiledScene {
            geometry,
            radius,
            transforms: vec![],
            objects: vec![],
            materials: vec![],
            words: vec![],
            source,
        },
    };
    compiler.object(&scene.object, Transform::identity(geometry))?;
    let inverse_camera = scene.view.map.inverse()?;
    for map in &compiler.result.transforms {
        inverse_camera
            .chain(map)?
            .rows()
            .context("camera-relative object transform cannot be represented safely")?;
    }
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

struct Compiler {
    context: GeometryContext,
    ids: BTreeMap<String, u32>,
    result: CompiledScene,
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
        value
            .schema
            .validate(self.context, &value.words)
            .with_context(|| format!("material {}", value.schema.key))?;
        let index = u32::try_from(self.result.materials.len())?;
        let base = self.words(&value.words)?;
        self.result.materials.push(MaterialRecord {
            data: [self.ids[&value.schema.key], base, 0, 0],
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
                self.push_object(shape, map, material)
            }
        }
    }
    fn push_object(&mut self, shape: &ShapeValue, map: Transform, material: u32) -> Result<()> {
        ensure!(
            !shape.words.is_empty(),
            "shapes must reserve an identity word"
        );
        shape
            .schema
            .validate(self.context, &shape.words)
            .with_context(|| format!("shape {}", shape.schema.key))?;
        ensure!(
            self.result.objects.len() < u32::MAX as usize,
            "too many objects"
        );
        let base = self.words(&shape.words)?;
        map.components()?;
        self.result.transforms.push(map);
        let [map0, map1] = Transform::identity(self.context.geometry).rows()?;
        self.result.objects.push(GpuObject {
            map0,
            map1,
            info: [self.ids[&shape.schema.key], material, base, 0],
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Background, View};
    fn shape() -> ShapeValue {
        ShapeValue::new(ShaderModule::new("test.shape", ShaderKind::Shape,
            "fn {{self}}(base:u32,ray:GeoRay,previous:u32)->GeoTaggedHit {return GeoTaggedHit(geo_miss(),base);}", Some(1)), vec![0]).unwrap()
    }
    fn material() -> MaterialValue {
        MaterialValue::new(ShaderModule::new("test.material", ShaderKind::Material,
            "fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {(*sample).alive=0u;}", Some(1)), vec![0]).unwrap()
    }
    fn scene() -> SceneDefinition {
        SceneDefinition {
            view: View {
                map: Transform::identity(Geometry::Euclidean),
                fov: 1.0,
            },
            background: Background::Constant([0.0; 3]),
            bounces: 4,
            radius: 1.0,
            medium: Default::default(),
            object: ObjectNode::Covered {
                shape: shape(),
                material: material(),
            },
            modules: vec![],
        }
    }
    #[test]
    fn storage_records_have_exact_word_layout() {
        assert_eq!(std::mem::size_of::<GpuObject>(), 48);
        assert_eq!(std::mem::size_of::<MaterialRecord>(), 16);
        assert_eq!(std::mem::offset_of!(GpuObject, info), 32);
    }
    #[test]
    fn linking_is_deterministic_and_values_are_buffer_only() -> Result<()> {
        let a = scene();
        let mut b = a.clone();
        let ObjectNode::Covered { shape, material } = &mut b.object else {
            unreachable!()
        };
        shape.words[0] = 42;
        material.words[0] = 91;
        b.modules = vec![material.schema.clone(), shape.schema.clone()];
        let x = compile(&a)?;
        let y = compile(&b)?;
        assert_eq!(x.source, y.source);
        assert_ne!(x.words, y.words);
        b.modules.reverse();
        assert_eq!(x.source, compile(&b)?.source);
        Ok(())
    }
    #[test]
    fn dependencies_are_namespaced_and_conflicts_cycles_missing_imports_fail() -> Result<()> {
        let mut a = scene();
        let mut helper = ShaderModule::new(
            "helpers.constant",
            ShaderKind::Library,
            "fn {{self}}()->f32{return 1.0;}",
            None,
        );
        let mut root = ShaderModule::new(
            "helpers.user",
            ShaderKind::Library,
            "fn {{self}}()->f32{return {{dep0}}();}",
            None,
        );
        root.dependencies.push(helper.clone());
        a.modules.push(root.clone());
        let source = compile(&a)?.source;
        assert!(source.contains("// module \"helpers.constant\""));
        assert!(!source.contains("{{"));
        helper.source.push_str("// changed");
        a.modules.push(helper);
        assert!(format!("{:#}", compile(&a).unwrap_err()).contains("conflicting implementations"));
        a.modules = vec![root.clone()];
        a.modules[0].dependencies[0].dependencies.push(root);
        assert!(format!("{:#}", compile(&a).unwrap_err()).contains("dependency cycle"));
        a.modules = vec![ShaderModule::new(
            "bad",
            ShaderKind::Library,
            "fn {{self}}()->f32{return {{dep0}}();}",
            None,
        )];
        assert!(format!("{:#}", compile(&a).unwrap_err()).contains("unresolved"));
        Ok(())
    }
    #[test]
    fn duplicate_parent_dependencies_are_validated_independently_of_order() -> Result<()> {
        let child = ShaderModule::new(
            "test.context-child",
            ShaderKind::Library,
            "fn {{self}}()->f32{return 1.0;}",
            None,
        );
        let mut parent = ShaderModule::new(
            "test.context-parent",
            ShaderKind::Library,
            "fn {{self}}()->f32{return {{dep0}}();}",
            None,
        );
        parent.dependencies.push(child);
        let mut rejected = parent.clone();
        rejected.dependencies[0].validate_context =
            |_| anyhow::bail!("dependency rejects the current geometry");
        assert!(parent.same_implementation(&rejected));
        let mut definition = scene();
        definition.modules = vec![parent.clone()];
        compile(&definition)?;
        for modules in [
            vec![parent.clone(), rejected.clone()],
            vec![rejected.clone(), parent.clone()],
        ] {
            definition.modules = modules;
            let error = compile(&definition).unwrap_err();
            assert!(format!("{error:#}").contains("dependency rejects the current geometry"));
        }
        Ok(())
    }
    #[test]
    fn malformed_values_and_component_validators_are_checked() {
        let mut a = scene();
        let ObjectNode::Covered { material, .. } = &mut a.object else {
            unreachable!()
        };
        material.schema.validate_words = |_, _, words| {
            ensure!(words[0] == 0, "invalid component value");
            Ok(())
        };
        material.words[0] = 1;
        assert!(compile(&a).is_err());
        let ObjectNode::Covered { shape, .. } = &mut a.object else {
            unreachable!()
        };
        shape.words.clear();
        assert!(compile(&a).is_err());
    }
    #[test]
    fn invalid_scene_context_and_relative_precision_fail() {
        for geometry in [
            Geometry::Euclidean,
            Geometry::Hyperbolic,
            Geometry::Spherical,
        ] {
            for radius in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::MIN_POSITIVE] {
                let mut a = scene();
                a.radius = radius;
                a.view.map = Transform::identity(geometry);
                assert!(compile(&a).is_err());
            }
            for fov in [0.0, -1.0, f64::NAN, f64::INFINITY] {
                let mut a = scene();
                a.view.fov = fov;
                a.view.map = Transform::identity(geometry);
                assert!(compile(&a).is_err());
            }
        }
        use ccgeom::{Geometry3, Hyperboloid3};
        let mut a = scene();
        a.view.map = Transform::from_isometry(Hyperboloid3::shift_z(12.0)).unwrap();
        a.object = ObjectNode::Mapped {
            map: Transform::from_isometry(Hyperboloid3::shift_z(12.2)).unwrap(),
            inner: Box::new(a.object),
        };
        assert!(compile(&a).is_ok());
        a.view.map = Transform::identity(Geometry::Hyperbolic);
        assert!(compile(&a).is_err());
        a.view.map = Transform::identity(Geometry::Spherical);
        assert!(compile(&a).is_err());
    }
}
