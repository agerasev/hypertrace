//! Downstream extension acceptance: this integration-test crate owns both new
//! components. Neither the scene compiler nor renderer imports their types.
use ccgeom::{Embedded3, EmbeddedIsometry, Space3};
use hypertrace_renderer::{Gpu, Renderer, Scene};
use objects::{
    Mapped, Material, Scene as _, SceneImpl, Shape,
    background::ConstBg,
    material::{Absorbing, Colored},
    object::Covered,
    shader::{self, Geometry, MaterialModule, MaterialValue, Result, ShapeModule, ShapeValue},
    shape::{self, Plane},
    view::PointView,
};

/// A plane clipped to a disk in its embedded spatial x/y coordinates. `extent`
/// deliberately describes that coordinate aperture, not a geodesic radius.
#[derive(Clone)]
pub struct Aperture {
    pub extent: f32,
}

impl Aperture {
    fn module<G: Geometry>() -> ShapeModule<G> {
        let mut module = ShapeModule::new(
            "downstream.aperture",
            r#"
fn {{self}}(base:u32,ray:GeoRay,previous:u32)->GeoTaggedHit {
    let result = {{dep0}}(base,ray,previous);
    if result.hit.valid != 1u { return result; }
    let extent = load_f32(base);
    if dot(result.hit.position.yz,result.hit.position.yz) > extent*extent {
        return GeoTaggedHit(geo_miss(),base);
    }
    return result;
}
"#,
            Some(1),
        );
        // The primitive is an explicit component dependency, never an assumed
        // renderer-provided function. Its entry point ignores its identity word.
        module
            .dependencies
            .push(shape::plane_schema::<G>().into_source());
        module.validate_words = |_, _, words| {
            let extent = f32::from_bits(words[0]);
            anyhow::ensure!(
                extent.is_finite() && extent > 0.0 && (extent * extent).is_finite(),
                "aperture extent must have a finite positive square"
            );
            Ok(())
        };
        module
    }
}

impl<G: Geometry> Shape<G> for Aperture {
    fn shader() -> Result<ShapeModule<G>> {
        Ok(Self::module::<G>())
    }

    fn encode(&self) -> Result<ShapeValue<G>> {
        ShapeValue::new(Self::module::<G>(), vec![self.extent.to_bits()])
    }
}

#[derive(Clone)]
pub struct Radiance(pub [f32; 3]);

impl<G: Geometry> Material<G> for Radiance {
    fn shader() -> Result<MaterialModule<G>> {
        let mut module = MaterialModule::new(
            "downstream.radiance",
            r#"
fn {{self}}(base:u32,context:GeoMaterialContext,
            sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
    (*sample).emission += (*sample).attenuation*load_vec3(base);
    (*sample).alive = 0u;
}

fn {{self}}_evaluate(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>,outgoing:vec3<f32>)->MaterialEvaluation {
    return MaterialEvaluation(vec3<f32>(0),0,1u);
}
fn {{self}}_emission(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>)->MaterialEmission {
    return MaterialEmission(load_vec3(base),1u);
}
"#,
            Some(3),
        );
        module.validate_words = |_, _, words| {
            anyhow::ensure!(
                words.iter().all(|&word| {
                    let value = f32::from_bits(word);
                    value.is_finite() && value >= 0.0
                }),
                "radiance must be finite and nonnegative"
            );
            Ok(())
        };
        Ok(module)
    }

    fn encode(&self) -> Result<MaterialValue<G>> {
        MaterialValue::new(
            <Self as Material<G>>::shader()?,
            self.0.map(f32::to_bits).into(),
        )
    }
}

objects::mixture! {
    LightMixture {
        light: Radiance,
        inactive: Absorbing,
    }
}

type Space<const K: i8> = Embedded3<f64, K>;
type Map<const K: i8> = EmbeddedIsometry<f64, K>;
type MappedShape<const K: i8> = Mapped<Space<K>, (Vec<Aperture>, Vec<Plane>), Map<K>>;
type Object<const K: i8> =
    Mapped<Space<K>, Covered<Space<K>, Vec<MappedShape<K>>, Colored<LightMixture>>, Map<K>>;
type Builder<const K: i8> =
    SceneImpl<Space<K>, Mapped<Space<K>, PointView<Space<K>>, Map<K>>, Vec<Object<K>>, ConstBg, 1>;

fn builder<const K: i8>() -> Builder<K>
where
    Space<K>: Geometry<Map = Map<K>>,
{
    let space = Space3::<f64, K>::unit();
    let patch = Mapped::new(
        (vec![Aperture { extent: 0.75 }], vec![]),
        space.translation([1.0, 0.0, 0.0].into(), 0.1).unwrap(),
    );
    let material = Colored::new(
        LightMixture::new(
            (Radiance([0.25, 0.5, 1.0]), 1.0).into(),
            (Absorbing, 0.0).into(),
        ),
        [0.5; 3].into(),
    );
    SceneImpl::new(
        Mapped::new(
            PointView::new(0.05),
            space.translation([0.0, 0.0, 1.0].into(), 1.0).unwrap(),
        ),
        vec![Mapped::new(
            Covered::new(vec![patch], material),
            space.translation([0.0, 1.0, 0.0].into(), 0.1).unwrap(),
        )],
        ConstBg::new([0.0; 3].into()),
    )
}

fn verify_cpu<const K: i8>()
where
    Space<K>: Geometry<Map = Map<K>>,
{
    let mut builder = builder::<K>();
    let original = shader::compile(&builder.definition().unwrap()).unwrap();
    assert!(
        original
            .source
            .contains("// module \"downstream.aperture\"")
    );
    assert!(
        original
            .source
            .contains("// module \"downstream.radiance\"")
    );
    assert_eq!(
        original
            .source
            .matches("// module \"hypertrace.shape.plane\"")
            .count(),
        1,
        "the static tuple child and downstream dependency share one implementation"
    );
    builder.object[0].inner.material.inner.light.material.0 = [1.0, 0.5, 0.25];
    builder.object[0].inner.shape[0].inner = (vec![], vec![Plane]);
    let changed = shader::compile(&builder.definition().unwrap()).unwrap();
    assert_ne!(changed.words, original.words);
    assert_eq!(changed.source, original.source);
    builder.object[0].inner.shape.clear();
    assert_eq!(
        shader::compile(&builder.definition().unwrap())
            .unwrap()
            .source,
        original.source
    );
    builder.object.clear();
    assert_eq!(
        shader::compile(&builder.definition().unwrap())
            .unwrap()
            .source,
        original.source
    );

    let mut invalid = self::builder::<K>();
    invalid.object[0].inner.shape[0].inner = (vec![Aperture { extent: f32::NAN }], vec![]);
    let error = shader::compile(&invalid.definition().unwrap()).unwrap_err();
    assert!(format!("{error:#}").contains("aperture extent"));
    let mut invalid = self::builder::<K>();
    invalid.object[0].inner.material.inner.light.material.0[0] = -1.0;
    let error = shader::compile(&invalid.definition().unwrap()).unwrap_err();
    assert!(format!("{error:#}").contains("radiance"));
}

#[test]
fn downstream_components_compile_validate_and_compose_in_every_curvature() {
    verify_cpu::<-1>();
    verify_cpu::<0>();
    verify_cpu::<1>();
}

fn verify_gpu<const K: i8>(gpu: &Gpu)
where
    Space<K>: Geometry<Map = Map<K>>,
{
    let mut builder = builder::<K>();
    let scene =
        |source: &Builder<K>| Scene::from_definition(&source.definition().unwrap()).unwrap();
    let mut renderer = Renderer::new(&gpu.device, &gpu.queue, (5, 3), scene(&builder), 19).unwrap();
    let revision = renderer.pipeline_revision();
    for (stage, expected) in [
        (0, [0.125, 0.25, 0.5, 1.0]),
        (1, [0.5, 0.25, 0.125, 1.0]),
        (2, [0.5, 0.25, 0.125, 1.0]),
        (3, [0.0, 0.0, 0.0, 1.0]),
        (4, [0.0, 0.0, 0.0, 1.0]),
    ] {
        match stage {
            1 => builder.object[0].inner.material.inner.light.material.0 = [1.0, 0.5, 0.25],
            2 => builder.object[0].inner.shape[0].inner = (vec![], vec![Plane]),
            3 => builder.object[0].inner.shape.clear(),
            4 => builder.object.clear(),
            _ => {}
        }
        renderer.update_scene(scene(&builder)).unwrap();
        assert_eq!(
            renderer.pipeline_revision(),
            revision,
            "K={K}, stage={stage}"
        );
        renderer.render();
        assert_eq!(
            renderer.snapshot().unwrap(),
            vec![expected; 15],
            "K={K}, stage={stage}"
        );
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn downstream_components_render_and_reuse_pipelines_in_every_curvature() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    verify_gpu::<-1>(&gpu);
    verify_gpu::<0>(&gpu);
    verify_gpu::<1>(&gpu);
}
