//! Probe the production directional samplers independently of image transport.
use ccgeom::{Embedded3, EmbeddedIsometry, Space3};
use hypertrace_renderer::{Gpu, Renderer, Scene, shader::*};
use objects::{
    light::{LightSampler, SphereBound},
    shape,
};

type G<const K: i8> = Embedded3<f64, K>;
type Map<const K: i8> = EmbeddedIsometry<f64, K>;

// A downstream proposal deliberately samples every direction. No renderer or
// compiler edit is needed to add it, and it composes as a typed dependency.
struct UniformDirections;
impl<T: Geometry> LightSampler<T> for UniformDirections {
    fn shader() -> Result<LightModule<T>> {
        Ok(LightModule::new(
            "downstream.uniform-directions",
            r#"
fn {{self}}_sample(base:u32,position:vec4<f32>,rng:ptr<function,u32>)->LightSample {
    let z=2*geo_uniform_open(rng)-1;
    let phi=2*PI*geo_uniform_open(rng);
    let r=sqrt(max(0.0,1-z*z));
    return LightSample(geo_from_local(position,vec3<f32>(r*cos(phi),r*sin(phi),z)),1/(4*PI),1u);
}
fn {{self}}_pdf(base:u32,ray:GeoRay)->LightPdf {return LightPdf(1/(4*PI),1u);}
"#,
            Some(0),
        ))
    }
    fn encode(&self) -> Result<LightValue<T>> {
        LightValue::new(Self::shader()?, vec![])
    }
}

fn probe<const K: i8>(gpu: &Gpu, sampler: LightModule<G<K>>, radius: f32, distance: f64) -> [f32; 3]
where
    G<K>: Geometry<Map = Map<K>>,
{
    let mut module = MaterialModule::new(
        "tests.light-proposal-probe",
        r#"
fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
    let position=load_vec4(base);
    var integral=0.0;
    var long_arcs=0.0;
    var hits=0.0;
    for(var i=0u;i<2048u;i+=1u) {
        // Uniform-solid-angle quadrature, independent of the proposal draws.
        let z=2*(f32(i)+0.5)/2048-1;
        let phi=2*PI*fract(f32(i)*0.61803398875);
        let radial=sqrt(max(0.0,1-z*z));
        let ray=GeoRay(position,geo_from_local(position,vec3<f32>(radial*cos(phi),radial*sin(phi),z)));
        let pdf={{dep0}}_pdf(base+4u,ray);
        if pdf.valid==0u {(*sample).emission=vec3<f32>(-1);(*sample).alive=0u;return;}
        integral+=pdf.value*(4*PI)/2048;
        let proposal={{dep0}}_sample(base+4u,position,rng);
        let proposed_ray=GeoRay(position,proposal.tangent);
        let reverse={{dep0}}_pdf(base+4u,proposed_ray);
        if proposal.valid==0u || reverse.valid==0u || !geo_ray_supported(proposed_ray) ||
            abs(proposal.pdf-reverse.value)>1e-4*proposal.pdf {
            (*sample).emission=vec3<f32>(-2);(*sample).alive=0u;return;
        }
        let hit={{dep1}}(base+4u,proposed_ray,0xffffffffu).hit;
        if hit.valid==2u {(*sample).emission=vec3<f32>(-3);(*sample).alive=0u;return;}
        if hit.valid==1u {
            hits+=1.0/2048;
            if GEO_K>0 && hit.distance>PI*params.misc.y {long_arcs+=1.0/2048;}
        }
    }
    (*sample).emission=vec3<f32>(integral,long_arcs,hits);
    (*sample).alive=0u;
}
fn {{self}}_evaluate(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>,outgoing:vec3<f32>)->MaterialEvaluation {return MaterialEvaluation(vec3<f32>(0),0,1u);}
fn {{self}}_emission(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>)->MaterialEmission {return MaterialEmission(vec3<f32>(0),1u);}
"#,
        Some(5),
    );
    module.dependencies = vec![
        sampler.into_source(),
        shape::geodesic_sphere_schema::<G<K>>().into_source(),
    ];
    let (w, z) = match K {
        -1 => (distance.cosh(), distance.sinh()),
        0 => (1.0, distance),
        _ => (distance.cos(), distance.sin()),
    };
    let scene = SceneDefinition {
        view: View {
            map: Transform::from_isometry(
                Space3::<f64, K>::unit()
                    .translation([0.0, 0.0, 1.0].into(), 0.2)
                    .unwrap(),
            )
            .unwrap(),
            fov: 0.01,
        },
        background: Background::constant([0.0; 3]),
        bounces: 1,
        radius: 1.0,
        medium: Medium::vacuum(),
        objects: vec![EncodedObject {
            map: Transform::identity(),
            shape: shape::plane(),
            material: MaterialValue::new(
                module,
                vec![
                    (w as f32).to_bits(),
                    0,
                    0,
                    (z as f32).to_bits(),
                    radius.to_bits(),
                ],
            )
            .unwrap(),
            sampling: None,
        }],
        modules: Modules::default(),
    };
    let renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (1, 1),
        Scene::from_definition(&scene).unwrap(),
        731,
    )
    .unwrap();
    renderer.render();
    let p = renderer.snapshot().unwrap()[0];
    [p[0], p[1], p[2]]
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn bounds_normalize_and_sample_both_spherical_routes_and_degenerate_poles() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    eprintln!("light proposal adapter: {:?}", gpu.adapter.get_info());
    fn check<const K: i8>(gpu: &Gpu, d: f64, r: f32, expect_long: bool)
    where
        G<K>: Geometry<Map = Map<K>>,
    {
        let result = probe::<K>(
            gpu,
            <SphereBound as LightSampler<G<K>>>::shader().unwrap(),
            r,
            d,
        );
        assert!(
            (result[0] - 1.0).abs() < 0.025,
            "K={K}, d={d}, r={r}: {result:?}"
        );
        assert_eq!(
            result[2], 1.0,
            "every proposed ray meets the actual sphere: K={K}, d={d}, r={r}: {result:?}"
        );
        if expect_long {
            assert!((result[1] - 0.5).abs() < 0.05, "long arcs: {result:?}");
        }
    }
    check::<-1>(&gpu, 1.0, 0.4, false);
    check::<0>(&gpu, 1.0, 0.4, false);
    check::<1>(&gpu, 1.0, 0.4, true);
    check::<-1>(&gpu, 0.0, 0.4, false);
    check::<0>(&gpu, 0.0, 0.4, false);
    check::<1>(&gpu, 0.0, 0.4, false);
    check::<1>(&gpu, std::f64::consts::PI, 0.4, false);
    check::<1>(&gpu, std::f64::consts::PI - 0.1, 0.4, false);
    check::<1>(&gpu, 1.0, 2.0, false);
    // A custom distribution is valid in every curvature through the same ABI.
    for result in [
        probe::<-1>(
            &gpu,
            <UniformDirections as LightSampler<G<-1>>>::shader().unwrap(),
            0.4,
            0.0,
        ),
        probe::<0>(
            &gpu,
            <UniformDirections as LightSampler<G<0>>>::shader().unwrap(),
            0.4,
            0.0,
        ),
        probe::<1>(
            &gpu,
            <UniformDirections as LightSampler<G<1>>>::shader().unwrap(),
            0.4,
            0.0,
        ),
    ] {
        assert!((result[0] - 1.0).abs() < 1e-5);
        assert_eq!(result[2], 1.0);
    }
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn downstream_light_sampler_renders_through_generated_dispatch_in_every_curvature() {
    use objects::material::{self, MaterialValueExt};
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    fn check<const K: i8>(gpu: &Gpu)
    where
        G<K>: Geometry<Map = Map<K>>,
    {
        let space = Space3::<f64, K>::unit();
        let map = Transform::from_isometry(space.translation([0.0, 0.0, 1.0].into(), 0.8).unwrap())
            .unwrap();
        let mut sampling =
            <UniformDirections as LightSampler<G<K>>>::encode(&UniformDirections).unwrap();
        sampling.map = map;
        let definition = SceneDefinition {
            view: View {
                map: Transform::from_isometry(
                    space.translation([0.0, 0.0, 1.0].into(), 0.3).unwrap(),
                )
                .unwrap(),
                fov: 1e-7,
            },
            background: Background::constant([0.0; 3]),
            bounces: 2,
            radius: 1.0,
            medium: Medium::vacuum(),
            objects: vec![
                EncodedObject {
                    map: Transform::identity(),
                    shape: shape::plane(),
                    material: material::lambertian(),
                    sampling: None,
                },
                EncodedObject {
                    map,
                    shape: shape::geodesic_sphere(0.25).unwrap(),
                    material: material::absorbing().emissive([1.0; 3]).unwrap(),
                    sampling: Some(sampling),
                },
            ],
            modules: Modules::default(),
        };
        let mut renderer = Renderer::new(
            &gpu.device,
            &gpu.queue,
            (128, 1),
            Scene::from_definition(&definition).unwrap(),
            79,
        )
        .unwrap();
        renderer.set_samples_per_dispatch(64).unwrap();
        for _ in 0..8 {
            renderer.render();
        }
        let mean = renderer
            .snapshot()
            .unwrap()
            .iter()
            .map(|p| p[0])
            .sum::<f32>()
            / 128.0;
        let sine = |x: f64| match K {
            -1 => x.sinh(),
            0 => x,
            _ => x.sin(),
        };
        let expected = (sine(0.25) / sine(0.8)).powi(2) as f32;
        assert!(
            (mean - expected).abs() < 0.05 * expected,
            "K={K}: downstream {mean} vs {expected}"
        );
    }
    check::<-1>(&gpu);
    check::<0>(&gpu);
    check::<1>(&gpu);
}
