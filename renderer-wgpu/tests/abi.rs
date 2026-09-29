//! Read the actual production WGSL records on the GPU. Distinct values make
//! matrix ordering, padding and array stride errors observable independently
//! of whether a rendered image looks plausible.
use hypertrace_wgpu::{Background, Gpu, Material, Object, Renderer, Scene, read_buffer};
use objects::Scene as _;
use wgpu::util::DeviceExt;

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn generated_uniform_carries_physical_radius_and_medium() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    let mut definition = examples::sp::scene::<6>().wgsl_scene().unwrap();
    definition.radius = 2.5;
    definition.medium = hypertrace_wgpu::wgsl::Medium::Homogeneous {
        extinction: 0.125,
        albedo: [0.2, 0.4, 0.7],
    };
    let scene = Scene::from_definition(&definition).unwrap();
    let renderer = Renderer::new(&gpu.device, &gpu.queue, (13, 7), scene, 123).unwrap();
    let source = format!(
        "{}\n{}",
        include_str!("../src/shaders/generated_trace.wgsl")
            .split("struct Object")
            .next()
            .unwrap(),
        r#"
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage,read_write> output: array<vec4<f32>>;
@compute @workgroup_size(1)
fn probe() {
    output[0]=params.camera0; output[1]=params.camera1;
    output[2]=params.background0; output[3]=params.background1;
    output[4]=params.background_axis; output[5]=vec4<f32>(params.info);
    output[6]=vec4<f32>(params.options); output[7]=params.misc;
    output[8]=params.medium;
}
"#
    );
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("generated uniform ABI probe"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("generated uniform ABI probe"),
            layout: None,
            module: &shader,
            entry_point: Some("probe"),
            compilation_options: Default::default(),
            cache: None,
        });
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 144,
        mapped_at_creation: false,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let entries = [(0, renderer.params_buffer()), (1, &output)].map(|(binding, buffer)| {
        wgpu::BindGroupEntry {
            binding,
            resource: buffer.as_entire_binding(),
        }
    });
    let group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &entries,
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    gpu.queue.submit([encoder.finish()]);
    let bytes = read_buffer(&gpu.device, &gpu.queue, &output, 144).unwrap();
    let actual: Vec<[f32; 4]> = bytes
        .as_chunks::<16>()
        .0
        .iter()
        .map(|row| bytemuck::pod_read_unaligned(row))
        .collect();
    assert_eq!(
        actual,
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0; 4],
            [0.0; 4],
            [0.0; 4],
            [0.0; 4],
            [13.0, 7.0, 2.0, 6.0],
            [1.0, 6.0, 0.0, 0.0],
            [1.0, 2.5, 1.0, 0.0],
            [0.2, 0.4, 0.7, 0.125],
        ]
    );
}

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn production_storage_and_uniform_layouts() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    eprintln!("ABI adapter: {:?}", gpu.adapter.get_info());
    let mut scene = Scene::hy();
    scene.fov = 1.25;
    scene.bounces = 7;
    scene.background = Background::Gradient {
        colors: [[0.1, 0.2, 0.3], [0.4, 0.5, 0.6]],
        axis: [0.0, -1.0, 0.0],
        power: 2.4,
    };
    let mut expected = Vec::from(scene.camera.gpu_rows());
    expected.extend([
        [0.1, 0.2, 0.3, 0.0],
        [0.4, 0.5, 0.6, 0.0],
        [0.0, -1.0, 0.0, 0.0],
        [17.0, 9.0, 1.0, 4.0],
        [3.0, 7.0, 1.0, 0.0],
        [1.25, 1.0, 2.4, 0.0],
    ]);
    let mut renderer = Renderer::new(&gpu.device, &gpu.queue, (17, 9), scene.clone(), 123).unwrap();
    renderer.set_samples_per_dispatch(3).unwrap();
    let objects: [Object; 2] = scene.objects[..2].try_into().unwrap();
    let materials = [
        Material {
            diffuse: [0.11, 0.12, 0.13, 0.2],
            emission: [0.21, 0.22, 0.23, 0.3],
            transmission: [0.31, 0.32, 0.33, 0.1],
            properties: [0.15, 1.33, 0.0, 0.0],
        },
        Material {
            diffuse: [0.41, 0.42, 0.43, 0.5],
            emission: [0.51, 0.52, 0.53, 0.1],
            transmission: [0.61, 0.62, 0.63, 0.2],
            properties: [0.15, 1.75, 0.0, 0.0],
        },
    ];
    for object in objects {
        expected.extend([
            object.map0,
            object.map1,
            object.info.map(|x| x as f32),
            object.extra.map(|x| x as f32),
            object.props,
        ]);
    }
    for material in materials {
        expected.extend([
            material.diffuse,
            material.emission,
            material.transmission,
            material.properties,
        ]);
    }
    let source = format!(
        "{}\n{}",
        include_str!("../src/shaders/trace.wgsl")
            .split("struct SceneHit")
            .next()
            .unwrap(),
        r#"
@group(0) @binding(5) var<storage,read_write> output: array<vec4<f32>>;
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage,read> objects: array<Object>;
@group(0) @binding(2) var<storage,read> materials: array<Material>;
@compute @workgroup_size(1)
fn probe_abi() {
    output[0]=params.camera0; output[1]=params.camera1;
    output[2]=params.background0; output[3]=params.background1; output[4]=params.background_axis;
    output[5]=vec4<f32>(params.info); output[6]=vec4<f32>(params.options); output[7]=params.misc;
    for (var i=0u;i<2u;i+=1u) {
        output[8+5*i]=objects[i].map0; output[9+5*i]=objects[i].map1;
        output[10+5*i]=vec4<f32>(objects[i].info); output[11+5*i]=vec4<f32>(objects[i].extra);
        output[12+5*i]=objects[i].props;
        output[18+4*i]=materials[i].diffuse; output[19+4*i]=materials[i].emission;
        output[20+4*i]=materials[i].transmission; output[21+4*i]=materials[i].properties;
    }
}
"#
    );
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("production ABI probe"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("production ABI probe"),
            layout: None,
            module: &shader,
            entry_point: Some("probe_abi"),
            compilation_options: Default::default(),
            cache: None,
        });
    let storage = |contents| {
        gpu.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents,
                usage: wgpu::BufferUsages::STORAGE,
            })
    };
    let objects = storage(bytemuck::cast_slice(&objects));
    let materials = storage(bytemuck::cast_slice(&materials));
    let bytes = (expected.len() * 16) as u64;
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: bytes,
        mapped_at_creation: false,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let entries = [
        (0, renderer.params_buffer()),
        (1, &objects),
        (2, &materials),
        (5, &output),
    ]
    .map(|(binding, buffer)| wgpu::BindGroupEntry {
        binding,
        resource: buffer.as_entire_binding(),
    });
    let group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &entries,
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    gpu.queue.submit([encoder.finish()]);
    let actual = read_buffer(&gpu.device, &gpu.queue, &output, bytes).unwrap();
    let actual: Vec<[f32; 4]> = actual
        .as_chunks::<16>()
        .0
        .iter()
        .map(|row| bytemuck::pod_read_unaligned(row))
        .collect();
    assert_eq!(actual, expected);
}
