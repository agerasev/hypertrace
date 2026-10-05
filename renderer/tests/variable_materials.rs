//! A downstream material can resize parameters inside standard combinators.
use ccgeom::{Flat3, Geometry3};
use hypertrace_renderer::{Gpu, Renderer, Scene, shader::*};
use objects::{Material, material, material::MaterialValueExt, object::tiling, shape};

struct Bands(Vec<[f32; 3]>);
impl Material<Flat3> for Bands {
    fn shader() -> Result<MaterialModule<Flat3>> {
        let mut module = MaterialModule::new(
            "tests.variable-bands",
            r#"
fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
    var emission=vec3<f32>(0);
    for(var i=0u;i<load_u32(base);i+=1u) {emission+=load_vec3(base+1u+3u*i);}
    (*sample).emission+=(*sample).attenuation*emission;
    (*sample).alive=0u;
}

fn {{self}}_evaluate(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>,outgoing:vec3<f32>)->MaterialEvaluation {
    return MaterialEvaluation(vec3<f32>(0),0,1u);
}
fn {{self}}_emission(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>)->MaterialEmission {
    var emission=vec3<f32>(0);
    for(var i=0u;i<load_u32(base);i+=1u) {emission+=load_vec3(base+1u+3u*i);}
    return MaterialEmission(emission,1u);
}
"#,
            None,
        );
        module.validate_words = |_, _, words| {
            let count = *words
                .first()
                .ok_or_else(|| anyhow::anyhow!("missing band count"))?
                as usize;
            anyhow::ensure!(
                count.checked_mul(3) == Some(words.len() - 1),
                "invalid band payload length"
            );
            anyhow::ensure!(
                words[1..].iter().all(|word| {
                    let value = f32::from_bits(*word);
                    value.is_finite() && value >= 0.0
                }),
                "invalid band emission"
            );
            Ok(())
        };
        Ok(module)
    }
    fn encode(&self) -> Result<MaterialValue<Flat3>> {
        let mut words = vec![u32::try_from(self.0.len())?];
        words.extend(self.0.iter().flatten().map(|value| value.to_bits()));
        MaterialValue::new(Self::shader()?, words)
    }
}

fn definition(
    ordinary: Vec<[f32; 3]>,
    border: Vec<[f32; 3]>,
    use_border: bool,
) -> Result<SceneDefinition<Flat3>> {
    let ordinary = material::mixture(vec![
        (1.0, Bands(ordinary).encode()?),
        (0.0, material::transparent()),
    ])?
    .emissive([0.125, 0.25, 0.5])?
    .colored([0.5, 0.25, 1.0])?;
    let border = material::mixture(vec![
        (0.0, material::transparent()),
        (1.0, Bands(border).encode()?),
    ])?
    .colored([1.0, 0.5, 0.25])?;
    // This selector lets the test visit ordinary and border dispatch branches
    // using only parameter updates, independently of pixel jitter and geometry.
    let selector = LibraryModule::new(
        "tests.branch-selector",
        "fn {{self}}(position:vec4<f32>,cell:f32,width:f32,count:u32)->u32 {return select(0u,count,cell>1.5);}",
        Some(2),
    );
    let material = tiling::tiled(
        selector,
        vec![ordinary],
        border,
        if use_border { 2.0 } else { 1.0 },
        0.0,
    )?;
    Ok(SceneDefinition {
        view: View {
            map: Transform::from_isometry(Flat3::shift_z(3.0)).unwrap(),
            fov: 0.1,
        },
        background: Background::constant([0.0; 3]),
        bounces: 1,
        radius: 1.0,
        medium: Medium::vacuum(),
        objects: vec![EncodedObject {
            map: Transform::identity(),
            shape: shape::plane(),
            material,
        }],
        modules: Modules::default(),
    })
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn resized_materials_keep_pipeline_through_mixtures_modifiers_and_tiling() {
    let gpu = futures::executor::block_on(Gpu::headless()).unwrap();
    let mut renderer = Renderer::new(
        &gpu.device,
        &gpu.queue,
        (7, 5),
        Scene::from_definition(&definition(vec![], vec![], false).unwrap()).unwrap(),
        37,
    )
    .unwrap();
    let revision = renderer.pipeline_revision();
    let cases = [
        (vec![], vec![], false),
        (vec![[0.25, 0.5, 0.125]], vec![[0.5, 0.25, 0.125]], false),
        (vec![[0.125, 0.25, 0.5]; 64], vec![[0.5, 0.25, 0.125]], true),
        (vec![], vec![[0.125, 0.5, 0.25]; 5], true),
        (vec![[0.25, 0.125, 0.5]; 2], vec![], false),
        (vec![], vec![], true),
    ];
    for (ordinary, border, use_border) in cases {
        let sum = |values: &[[f32; 3]]| {
            values.iter().fold([0.0; 3], |mut total, value| {
                for i in 0..3 {
                    total[i] += value[i];
                }
                total
            })
        };
        let expected = if use_border {
            let rgb = sum(&border);
            [rgb[0], 0.5 * rgb[1], 0.25 * rgb[2], 1.0]
        } else {
            let rgb = sum(&ordinary);
            [
                0.5 * (0.125 + rgb[0]),
                0.25 * (0.25 + rgb[1]),
                0.5 + rgb[2],
                1.0,
            ]
        };
        renderer
            .update_scene(
                Scene::from_definition(&definition(ordinary, border, use_border).unwrap()).unwrap(),
            )
            .unwrap();
        assert_eq!(
            renderer.pipeline_revision(),
            revision,
            "child parameter lengths must remain buffer data"
        );
        renderer.render();
        assert_eq!(renderer.snapshot().unwrap(), vec![expected; 35]);
    }
}
