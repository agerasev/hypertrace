//! Read the actual production WGSL records on the GPU. Distinct values make
//! matrix ordering, padding and array stride errors observable independently
//! of whether a rendered image looks plausible.
use hypertrace_renderer::{
    Gpu, Renderer, Scene, read_buffer,
    shader::{GpuObject, MaterialRecord},
};
use objects::Scene as _;
use wgpu::util::DeviceExt;

#[test]
#[ignore = "requires a native WGPU compute adapter"]
fn generated_uniform_carries_physical_radius_and_medium() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    let mut definition = examples::sp::scene::<6>().definition().unwrap();
    definition.radius = 2.5;
    definition.medium = hypertrace_renderer::shader::Medium::Homogeneous {
        extinction: 0.125,
        albedo: [0.2, 0.4, 0.7],
    };
    let scene = Scene::from_definition(&definition).unwrap();
    let renderer = Renderer::new(&gpu.device, &gpu.queue, (13, 7), scene, 123).unwrap();
    let source = format!(
        "{}\n{}",
        include_str!("../src/shaders/abi.wgsl"),
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
fn production_storage_records_and_word_arena_layout() {
    let gpu = futures::executor::block_on(Gpu::headless()).expect("compute adapter required");
    eprintln!("ABI adapter: {:?}", gpu.adapter.get_info());
    let objects: [GpuObject; 2] = std::array::from_fn(|index| {
        let n = index as f32 * 10.0;
        let u = index as u32 * 10;
        GpuObject {
            map0: [n + 0.1, n + 0.2, n + 0.3, n + 0.4],
            map1: [n + 0.5, n + 0.6, n + 0.7, n + 0.8],
            info: [u + 1, u + 2, u + 3, u + 4],
        }
    });
    let materials = [
        MaterialRecord {
            data: [21, 22, 23, 24],
        },
        MaterialRecord {
            data: [31, 32, 33, 34],
        },
    ];
    let words = [41u32, 42, 43, 44];
    let mut expected = Vec::new();
    for object in objects {
        expected.extend([object.map0, object.map1, object.info.map(|x| x as f32)]);
    }
    expected.extend(materials.map(|material| material.data.map(|x| x as f32)));
    expected.push(words.map(|x| x as f32));
    let source = format!(
        "{}\n{}",
        include_str!("../src/shaders/abi.wgsl"),
        r#"
@group(0) @binding(0) var<storage,read> objects: array<Object>;
@group(0) @binding(1) var<storage,read> materials: array<MaterialRecord>;
@group(0) @binding(2) var<storage,read> words: array<u32>;
@group(0) @binding(3) var<storage,read_write> output: array<vec4<f32>>;
@compute @workgroup_size(1)
fn probe_abi() {
    for (var i=0u;i<2u;i+=1u) {
        output[3*i]=objects[i].map0; output[1+3*i]=objects[i].map1;
        output[2+3*i]=vec4<f32>(objects[i].info);
        output[6+i]=vec4<f32>(materials[i].data);
    }
    output[8]=vec4<f32>(f32(words[0]),f32(words[1]),f32(words[2]),f32(words[3]));
}
"#
    );
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("production storage ABI probe"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("production storage ABI probe"),
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
    let words = storage(bytemuck::cast_slice(&words));
    let bytes = (expected.len() * 16) as u64;
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: bytes,
        mapped_at_creation: false,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let entries =
        [(0, &objects), (1, &materials), (2, &words), (3, &output)].map(|(binding, buffer)| {
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
    let actual = read_buffer(&gpu.device, &gpu.queue, &output, bytes).unwrap();
    let actual: Vec<[f32; 4]> = actual
        .as_chunks::<16>()
        .0
        .iter()
        .map(|row| bytemuck::pod_read_unaligned(row))
        .collect();
    assert_eq!(actual, expected);
}
