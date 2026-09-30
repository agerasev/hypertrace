//! Reproducible native WGPU rendering without a window.
use anyhow::{Context, bail, ensure};
use hypertrace_gallery::{Example, with_example};
use hypertrace_renderer::{Gpu, Renderer, Result, Scene};
use objects::shader::{Geometry, SceneDefinition};
use std::{
    fs::File,
    io::{BufWriter, Write},
    time::{Duration, Instant},
};

mod support;

fn main() -> Result<()> {
    let mut name = String::from("hyperbolic");
    let (mut width, mut height, mut samples, mut seed) = (320u32, 240u32, 64u32, 3735928559u32);
    let mut bounces = None;
    let mut batch = 16u32;
    let mut fov = None;
    let (mut yaw, mut pitch) = (0.0_f64, 0.0_f64);
    let mut output = String::from("render");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--list-scenes" {
            println!("{}", support::catalog());
            return Ok(());
        }
        if arg == "--help" || arg == "-h" {
            println!(
                "headless [--scene NAME] [--width 320] [--height 240] [--samples 64] [--batch 16] [--seed 3735928559] [--bounces 1..64] [--fov SCALE] [--yaw RADIANS] [--pitch RADIANS] [--output PREFIX]\nCamera angles are local offsets from the example camera; fov is tan(vertical_angle/2).\nWrites PREFIX.{{rgba32f,ppm,json}}; Use --list-scenes to see examples. WGPU_BACKEND selects a native backend."
            );
            return Ok(());
        }
        let value = args
            .next()
            .with_context(|| format!("missing value for {arg}"))?;
        match arg.as_str() {
            "--scene" => name = value,
            "--width" => width = value.parse()?,
            "--height" => height = value.parse()?,
            "--samples" => samples = value.parse()?,
            "--batch" => batch = value.parse()?,
            "--seed" => seed = value.parse()?,
            "--bounces" => bounces = Some(value.parse()?),
            "--output" => output = value,
            "--fov" => fov = Some(value.parse()?),
            "--yaw" => yaw = value.parse()?,
            "--pitch" => pitch = value.parse()?,
            _ => bail!("unknown option {arg}"),
        }
    }
    ensure!(samples > 0, "samples must be positive");
    ensure!((1..=64).contains(&batch), "batch must be in 1..=64");
    ensure!(
        yaw.is_finite() && pitch.is_finite(),
        "camera angles must be finite"
    );
    let options = Options {
        width,
        height,
        samples,
        batch,
        seed,
        bounces,
        fov,
        yaw,
        pitch,
        output,
    };
    with_example!(name.as_str(), |example, factory| run(
        example, factory, options
    ))
}

struct Options {
    width: u32,
    height: u32,
    samples: u32,
    batch: u32,
    seed: u32,
    bounces: Option<u32>,
    fov: Option<f32>,
    yaw: f64,
    pitch: f64,
    output: String,
}

fn run<G: Geometry>(
    example: Example,
    factory: fn() -> Result<SceneDefinition<G>>,
    options: Options,
) -> Result<()> {
    let Options {
        width,
        height,
        samples,
        batch,
        seed,
        bounces,
        fov,
        yaw,
        pitch,
        output,
    } = options;
    let name = example.id;
    let mut scene = Scene::from_definition(&factory()?)?;
    if let Some(b) = bounces {
        scene.bounces = b;
    }
    if let Some(fov) = fov {
        scene.fov = fov;
    }
    scene
        .camera
        .move_local([0.0; 3], [pitch, yaw, 0.0], f64::from(scene.radius))?;
    scene.validate()?;
    let fov = scene.fov;
    let bounces = scene.bounces;
    let curvature = G::SIGN;
    let radius = scene.radius;
    let [red, green, blue, extinction] = scene.medium.gpu_row();
    let gpu = futures::executor::block_on(Gpu::headless())?;
    let adapter = gpu.adapter.get_info();
    println!(
        "WGPU adapter: {} ({:?}, {:?})",
        adapter.name, adapter.backend, adapter.device_type
    );
    let mut renderer = Renderer::new(&gpu.device, &gpu.queue, (width, height), scene, seed)?;
    let start = Instant::now();
    let mut remaining = samples;
    while remaining > 0 {
        let count = remaining.min(batch);
        renderer.set_samples_per_dispatch(count)?;
        renderer.render();
        remaining -= count;
        // Complete each batch before queuing more. --batch 1 also bounds a
        // single dispatch on drivers with short GPU watchdog timeouts.
        gpu.device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(Duration::from_secs(60)),
        })?;
        if (samples - remaining) / 128 != (samples - remaining - count) / 128 {
            eprintln!("{name}: {}/{} samples", samples - remaining, samples);
        }
    }
    let pixels = renderer.snapshot()?;
    ensure!(
        pixels.iter().all(|p| p[3] == 1.0),
        "incomplete GPU frame: some pixels have no samples"
    );
    let elapsed = start.elapsed();
    let mut raw = BufWriter::new(File::create(format!("{output}.rgba32f"))?);
    let mut ppm = BufWriter::new(File::create(format!("{output}.ppm"))?);
    write!(ppm, "P6\n{width} {height}\n255\n")?;
    for pixel in pixels {
        for channel in pixel {
            raw.write_all(&channel.to_le_bytes())?;
        }
        for channel in &pixel[..3] {
            ppm.write_all(&[(channel.max(0.0).powf(1.0 / 2.2).min(1.0) * 255.0) as u8])?;
        }
    }
    raw.flush()?;
    ppm.flush()?;
    std::fs::write(
        format!("{output}.json"),
        format!(
            "{{\n  \"backend\": \"wgpu\",\n  \"adapter\": {:?},\n  \"scene\": {:?},\n  \"width\": {width},\n  \"height\": {height},\n  \"samples\": {samples},\n  \"seed\": {seed},\n  \"batch\": {batch},\n  \"camera\": {{\"fov\": {fov}, \"yaw\": {yaw}, \"pitch\": {pitch}}},\n  \"bounces\": {bounces},\n  \"curvature_sign\": {curvature},\n  \"curvature_radius\": {radius},\n  \"medium\": {{\"extinction\": {extinction}, \"albedo\": [{red}, {green}, {blue}]}},\n  \"linear_format\": \"little-endian rgba32f, row-major, top row first\",\n  \"display_gamma\": 2.2\n}}\n",
            adapter.name, name
        ),
    )?;
    println!(
        "WGPU {name}: {width}x{height}, {samples} samples, {bounces} bounces in {:.3}s; wrote {output}.{{rgba32f,ppm,json}}",
        elapsed.as_secs_f64()
    );
    Ok(())
}
