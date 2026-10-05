//! Independent values for the continuous BSDF, marginal PDF and emission queries.
use ccgeom::{Flat3, Geometry3};
use hypertrace_renderer::{Gpu, Renderer, Scene, shader::*};
use objects::{
    material::{self, MaterialValueExt},
    object::tiling,
    shape,
};

fn query(gpu: &Gpu, child: MaterialValue<Flat3>, outgoing_z: f32) -> [f32; 3] {
    let mut module = MaterialModule::new(
        "tests.material-query",
        r#"
fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
    let context=GeoMaterialContext(ctx.position,vec3<f32>(0,0,1));
    let incoming=vec3<f32>(0,0,-1);
    let outgoing=vec3<f32>(0.6,0,load_f32(base));
    let value={{dep0}}_evaluate(base+1u,context,incoming,outgoing);
    let emission={{dep0}}_emission(base+1u,context,incoming);
    (*sample).emission=vec3<f32>(value.value.x,value.pdf,emission.value.x);
    if value.valid==0u || emission.valid==0u {(*sample).emission=vec3<f32>(-1);}
    (*sample).alive=0u;
}
fn {{self}}_evaluate(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>,outgoing:vec3<f32>)->MaterialEvaluation {
    return MaterialEvaluation(vec3<f32>(0),0,1u);
}
fn {{self}}_emission(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>)->MaterialEmission {
    return MaterialEmission(vec3<f32>(0),1u);
}
"#,
        None,
    );
    module.dependencies.push(child.schema.into_source());
    let mut words = vec![outgoing_z.to_bits()];
    words.extend(child.words);
    let definition = SceneDefinition {
        view: View {
            map: Transform::from_isometry(Flat3::shift_z(1.0)).unwrap(),
            fov: 0.01,
        },
        background: Background::constant([0.0; 3]),
        bounces: 1,
        radius: 1.0,
        medium: Medium::vacuum(),
        objects: vec![EncodedObject {
            sampling: None,
            map: Transform::identity(),
            shape: shape::plane(),
            material: MaterialValue::new(module, words).unwrap(),
        }],
        modules: Modules::default(),
    };
    let renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (1, 1),
        Scene::from_definition(&definition).unwrap(),
        19,
    )
    .unwrap();
    renderer.render();
    let pixel = renderer.snapshot().unwrap()[0];
    [pixel[0], pixel[1], pixel[2]]
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn composition_queries_preserve_marginal_densities_and_emission_order() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    eprintln!("material evaluation adapter: {:?}", gpu.adapter.get_info());
    let diffuse = material::lambertian()
        .emissive([2.0; 3])
        .unwrap()
        .colored([0.4; 3])
        .unwrap();
    let inner = material::mixture(vec![(0.25, diffuse), (0.5, material::specular())]).unwrap();
    let outer = material::mixture(vec![(0.5, inner)])
        .unwrap()
        .colored([0.5; 3])
        .unwrap()
        .emissive([3.0; 3])
        .unwrap();
    let cosine_pdf = 0.8 / std::f32::consts::PI;
    let expected = [0.025 * cosine_pdf, 0.125 * cosine_pdf, 3.05];
    for (actual, expected) in query(&gpu, outer.clone(), 0.8).into_iter().zip(expected) {
        assert!((actual - expected).abs() < 2e-6, "{actual} vs {expected}");
    }
    let back = query(&gpu, outer, -0.8);
    assert_eq!(&back[..2], &[0.0, 0.0]);
    assert!((back[2] - 3.05).abs() < 2e-6);
    let selector = LibraryModule::new(
        "tests.query-selector",
        "fn {{self}}(p:vec4<f32>,cell:f32,width:f32,count:u32)->u32 {return u32(cell);}",
        Some(2),
    );
    let tiled = tiling::tiled(
        selector.clone(),
        vec![material::lambertian()],
        material::absorbing().emissive([4.0; 3]).unwrap(),
        1.0,
        0.0,
    )
    .unwrap();
    assert_eq!(query(&gpu, tiled, 0.8), [0.0, 0.0, 4.0]);
    let invalid = tiling::tiled(
        selector,
        vec![material::lambertian()],
        material::absorbing(),
        7.0,
        0.0,
    )
    .unwrap();
    assert_eq!(query(&gpu, invalid, 0.8), [-1.0; 3]);
}
