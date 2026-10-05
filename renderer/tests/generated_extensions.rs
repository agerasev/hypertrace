//! Execute extension leaves through the same compiler/runtime as scene builders.
use ccgeom::{Flat3, Geometry3};
use hypertrace_renderer::{Gpu, Renderer, Scene, pixel_seed, shader::*};
use objects::{
    Mapped, Shape as _, material,
    shape::{self, GeodesicSphere, Plane, ShapeValueExt as _},
};

fn definition(shape: ShapeValue<Flat3>, material: MaterialValue<Flat3>) -> SceneDefinition<Flat3> {
    SceneDefinition {
        view: View {
            map: Transform::from_isometry(Flat3::shift_z(3.0)).unwrap(),
            fov: 0.1,
        },
        background: Background::constant([0.0; 3]),
        bounces: 1,
        radius: 1.0,
        medium: Default::default(),
        objects: vec![EncodedObject {
            map: Transform::identity(),
            shape,
            material,
        }],
        modules: Modules::default(),
    }
}

// These statically distinct leaves share an exact plane coordinate. They isolate
// tuple identity forwarding from the rounding of independently solved surfaces.
struct IdentityPlane<const NORMAL: i8>;

impl<const NORMAL: i8> objects::Shape<Flat3> for IdentityPlane<NORMAL> {
    fn shader() -> Result<ShapeModule<Flat3>> {
        Ok(ShapeModule::new(
            format!("tests.identity-plane.{NORMAL}"),
            format!(
                r#"
fn {{{{self}}}}(base:u32,ray:GeoRay,previous:u32)->GeoTaggedHit {{
    if base==previous {{return GeoTaggedHit(geo_miss(),base);}}
    let distance=(1.0-ray.position.w)/ray.tangent.w;
    if distance<0.0 {{return GeoTaggedHit(geo_miss(),base);}}
    var position=geo_advance(ray,distance,1.0).position;
    position.w=1.0;
    return GeoTaggedHit(GeoHit(1u,distance,position,ray.tangent,
        vec4<f32>(0,0,0,{NORMAL})),base);
}}
"#
            ),
            Some(1),
        ))
    }

    fn encode(&self) -> Result<ShapeValue<Flat3>> {
        ShapeValue::new(Self::shader()?, vec![0])
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn heterogeneous_tuple_selects_nearest_child_and_preserves_repeat_identity() {
    let union = |plane_height| {
        (
            Mapped::<Flat3, _, _>::new(Plane, Flat3::shift_z(plane_height)),
            GeodesicSphere::new(1.0),
        )
            .encode()
            .unwrap()
    };
    let nearest_height = custom_material(
        "tests.tuple-nearest-height",
        r#"
fn {{self}}(base:u32,ctx:GeoMaterialContext,
                  sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
    (*sample).emission=vec3<f32>(ctx.position.w);
    (*sample).alive=0u;
}

fn {{self}}_evaluate(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>,outgoing:vec3<f32>)->MaterialEvaluation {
    return MaterialEvaluation(vec3<f32>(0),0,1u);
}
fn {{self}}_emission(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>)->MaterialEmission {
    return MaterialEmission(vec3<f32>(ctx.position.w),1u);
}
"#,
    );
    let scene = |height, material| {
        let mut scene = definition(union(height), material);
        // Tiny jitter keeps the front sphere hit near z=1. Both physical
        // children are present and queried in every scene.
        scene.view.fov = 1e-5;
        scene
    };
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let mut renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (3, 3),
        Scene::from_definition(&scene(0.0, nearest_height.clone())).unwrap(),
        19,
    )
    .unwrap();
    let check = |renderer: &Renderer<Flat3>, expected: f32, stage: &str| {
        renderer.render();
        for (index, pixel) in renderer.snapshot().unwrap().into_iter().enumerate() {
            assert_eq!(pixel[3], 1.0);
            for value in &pixel[..3] {
                assert!(
                    (value - expected).abs() <= 8.0 * f32::EPSILON,
                    "{stage}: pixel {index} {pixel:?}, expected {expected}"
                );
            }
        }
    };
    check(&renderer, 1.0, "sphere before plane");
    let revision = renderer.pipeline_revision();
    renderer
        .update_scene(Scene::from_definition(&scene(2.0, nearest_height)).unwrap())
        .unwrap();
    assert_eq!(renderer.pipeline_revision(), revision);
    check(&renderer, 2.0, "plane before sphere");

    let normal_probe = custom_material(
        "tests.tuple-repeat-normal",
        r#"
fn {{self}}(base:u32,ctx:GeoMaterialContext,
                  sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
    (*sample).emission+=vec3<f32>(1.0+ctx.normal.z);
}

fn {{self}}_evaluate(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>,outgoing:vec3<f32>)->MaterialEvaluation {
    return MaterialEvaluation(vec3<f32>(0),0,1u);
}
fn {{self}}_emission(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>)->MaterialEmission {
    return MaterialEmission(vec3<f32>(1.0+ctx.normal.z),1u);
}
"#,
    );
    let mut coincident = definition(
        (IdentityPlane::<-1>, IdentityPlane::<1>).encode().unwrap(),
        normal_probe,
    );
    coincident.bounces = 2;
    renderer
        .update_scene(Scene::from_definition(&coincident).unwrap())
        .unwrap();
    // The first leaf wins the equal-distance hit (normal -z), then only that
    // leaf is suppressed. The second leaf's zero-distance hit (normal +z)
    // remains eligible. Repeating the first leaf or suppressing both yields
    // zero instead of two. Exact shared plane coordinates avoid testing whether
    // two independently rounded surface solvers agree at a coincident contact.
    check(&renderer, 2.0, "distinct coincident leaves");
}

fn custom_material(key: &str, source: &str) -> MaterialValue<Flat3> {
    MaterialValue::new(MaterialModule::new(key, source, Some(0)), vec![]).unwrap()
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn nested_single_component_mixtures_each_advance_rng_once() {
    let probe = custom_material(
        "tests.rng-state",
        r#"
fn {{self}}(base:u32, ctx:GeoMaterialContext,
                  sample:ptr<function,MaterialSample>, rng:ptr<function,u32>) {
    (*sample).emission=vec3<f32>(f32(*rng)*(1.0/4294967296.0));
    (*sample).alive=0u;
}

fn {{self}}_evaluate(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>,outgoing:vec3<f32>)->MaterialEvaluation {
    return MaterialEvaluation(vec3<f32>(0),0,1u);
}
fn {{self}}_emission(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>)->MaterialEmission {
    return MaterialEmission(vec3<f32>(0.5),1u);
}
"#,
    );
    // Even deterministic, single-component mixtures draw independently. Removing
    // either draw changes the RNG seen by this leaf and all later path events.
    let inner = material::mixture(vec![(1.0, probe)]).unwrap();
    let outer = material::mixture(vec![(1.0, inner)]).unwrap();
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let seed = 37;
    let renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (5, 3),
        Scene::from_definition(&definition(shape::plane(), outer)).unwrap(),
        seed,
    )
    .unwrap();
    renderer.render();
    let expected: Vec<_> = (0..15)
        .map(|index| {
            let mut state = pixel_seed(seed, index);
            // Two camera jitter draws, then one per nested mixture.
            for _ in 0..4 {
                state = state.wrapping_mul(1103515245).wrapping_add(12345);
            }
            let value = state as f32 * (1.0 / 4294967296.0);
            [value, value, value, 1.0]
        })
        .collect();
    assert_eq!(renderer.snapshot().unwrap(), expected);
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn custom_shape_keeps_mapped_shape_and_object_material_frames_distinct() {
    // This downstream wrapper imports its primitive explicitly. The renderer
    // has no knowledge of this implementation or of the imported plane module.
    let mut module = ShapeModule::new(
        "tests.plane-alias",
        "fn {{self}}(base:u32,ray:GeoRay,previous:u32)->GeoTaggedHit { return {{dep0}}(base,ray,previous); }",
        Some(1),
    );
    module
        .dependencies
        .push(shape::plane_schema().into_source());
    let plane = ShapeValue::new(module, vec![0]).unwrap();
    let material = custom_material(
        "tests.local-height",
        r#"
fn {{self}}(base:u32,ctx:GeoMaterialContext,
                     sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
    (*sample).emission=vec3<f32>(ctx.position.w);
    (*sample).alive=0u;
}

fn {{self}}_evaluate(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>,outgoing:vec3<f32>)->MaterialEvaluation {
    return MaterialEvaluation(vec3<f32>(0),0,1u);
}
fn {{self}}_emission(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>)->MaterialEmission {
    return MaterialEmission(vec3<f32>(ctx.position.w),1u);
}
"#,
    );
    let shift = Transform::from_isometry(Flat3::shift_z(1.0)).unwrap();
    let mapped_shape = definition(plane.clone().mapped(shift).unwrap(), material.clone());
    let mut mapped_object = definition(plane, material);
    mapped_object.objects[0].map = shift;
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    for (definition, height) in [(mapped_shape, 1.0), (mapped_object, 0.0)] {
        let renderer = Renderer::new(
            &gpu.device,
            &gpu.queue,
            (3, 3),
            Scene::from_definition(&definition).unwrap(),
            19,
        )
        .unwrap();
        renderer.render();
        for pixel in renderer.snapshot().unwrap() {
            assert_eq!(pixel[3], 1.0);
            // Embedded advancement and the two frame transforms introduce
            // ordinary f32 coordinate roundoff; the frame separation is 1.0.
            for value in &pixel[..3] {
                assert!((value - height).abs() <= 8.0 * f32::EPSILON);
            }
        }
    }
}
