//! Synchronized OpenCL benchmark using the shared Euclidean/hyperbolic scenes.

use hypertrace::{
    cli::{get_ocl_context, OclApp},
    objects::Scene,
    proc::{Canvas, Context, EntityBuffer, Render},
    types::{
        config::{AddressWidth, Endian},
        Config, Geometry,
    },
};
use std::{fmt::Write as _, time::Instant};

struct Options {
    scene: String,
    width: usize,
    height: usize,
    samples: usize,
    trials: usize,
    warmup: usize,
    seed: u32,
    output: String,
}

fn pixel_seed(seed: u32, index: u32) -> u32 {
    let mut value = seed ^ index.wrapping_add(1).wrapping_mul(0x9e37_79b9);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^ (value >> 16)
}

fn reset(context: &Context, canvas: &mut Canvas, seeds: &[u32]) -> proc::Result<()> {
    canvas.clean()?;
    canvas.seeds_mut().raw_mut().write(seeds).enq()?;
    context.backend.queue.finish()?;
    Ok(())
}

fn snapshot(context: &Context, canvas: &Canvas, count: usize) -> proc::Result<Vec<f32>> {
    let mut linear = vec![0.0f32; 4 * count];
    canvas.raw_image().raw().read(&mut linear).enq()?;
    context.backend.queue.finish()?;
    let samples = canvas.passes() as f32;
    for pixel in linear.chunks_exact_mut(4) {
        for value in pixel.iter_mut() {
            *value /= samples;
            if !value.is_finite() {
                return Err("OpenCL benchmark produced a non-finite pixel".into());
            }
        }
        if pixel[3] != 1.0 {
            return Err("OpenCL benchmark produced an invalid sample count (alpha != 1)".into());
        }
    }
    Ok(linear)
}

fn json_string(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            c if c < ' ' => write!(&mut output, "\\u{:04x}", c as u32).unwrap(),
            c => output.push(c),
        }
    }
    output.push('"');
    output
}

fn benchmark<G: Geometry, S: Scene<G>>(
    context: &Context,
    scene: S,
    options: &Options,
    bounces: usize,
    device_setup_ms: f64,
) -> proc::Result<()> {
    let count = options
        .width
        .checked_mul(options.height)
        .filter(|&count| count <= u32::MAX as usize / 4)
        .ok_or("Image dimensions exceed the 32-bit renderer limits")?;
    let device = context.backend.queue.device();
    let device_name = device.name()?;
    let device_type = device.info(ocl::enums::DeviceInfo::Type)?.to_string();
    let driver = device
        .info(ocl::enums::DeviceInfo::DriverVersion)?
        .to_string();
    println!(
        "OpenCL device: {} ({}, driver {})",
        device_name, device_type, driver
    );

    let setup_start = Instant::now();
    let scene_buffer = EntityBuffer::new(context, &scene)?;
    let mut canvas = Canvas::new(&context.backend, (options.width, options.height))?;
    let seeds: Vec<_> = (0..count)
        .map(|index| pixel_seed(options.seed, index as u32))
        .collect();
    let kernel_start = Instant::now();
    let renderer = Render::<G, S>::new(context)?;
    let kernel_setup_ms = kernel_start.elapsed().as_secs_f64() * 1000.0;
    context.backend.queue.finish()?;
    let setup_ms = setup_start.elapsed().as_secs_f64() * 1000.0;

    reset(context, &mut canvas, &seeds)?;
    let warmup_start = Instant::now();
    for _ in 0..options.warmup {
        renderer.render(&scene_buffer, &mut canvas)?;
    }
    let warmup_ms = warmup_start.elapsed().as_secs_f64() * 1000.0;
    println!(
        "Setup: device {:.3} ms, renderer {:.3} ms (kernel setup {:.3} ms); warmup {} samples {:.3} ms",
        device_setup_ms, setup_ms, kernel_setup_ms, options.warmup, warmup_ms
    );

    let mut render_ms = Vec::with_capacity(options.trials);
    let mut readback_ms = Vec::with_capacity(options.trials);
    let mut checksum = [0.0f64; 3];
    let pixel_samples = count as f64 * options.samples as f64;
    for trial in 0..options.trials {
        reset(context, &mut canvas, &seeds)?;
        let render_start = Instant::now();
        for _ in 0..options.samples {
            // Render::render waits for completion after every sample.
            renderer.render(&scene_buffer, &mut canvas)?;
        }
        let elapsed_render_ms = render_start.elapsed().as_secs_f64() * 1000.0;
        let readback_start = Instant::now();
        // Match the WGPU snapshot scope: allocation, read, normalization, validation.
        let linear = snapshot(context, &canvas, count)?;
        let elapsed_readback_ms = readback_start.elapsed().as_secs_f64() * 1000.0;
        checksum = [0.0; 3];
        for pixel in linear.chunks_exact(4) {
            for channel in 0..3 {
                checksum[channel] += pixel[channel] as f64;
            }
        }
        println!(
            "Trial {}/{}: render {:.3} ms, readback {:.3} ms, {:.0} pixel-samples/s",
            trial + 1,
            options.trials,
            elapsed_render_ms,
            elapsed_readback_ms,
            pixel_samples * 1000.0 / elapsed_render_ms,
        );
        render_ms.push(elapsed_render_ms);
        readback_ms.push(elapsed_readback_ms);
    }

    let report = format!(
        concat!(
            "{{\n  \"schema_version\": 1,\n  \"backend\": \"opencl\",\n",
            "  \"device\": {},\n  \"device_type\": {},\n  \"driver\": {},\n",
            "  \"backend_api\": \"OpenCL\",\n  \"scene\": {},\n",
            "  \"width\": {},\n  \"height\": {},\n  \"samples\": {},\n",
            "  \"bounces\": {},\n  \"seed\": {},\n  \"batch_size\": 1,\n",
            "  \"sync_per_batch\": true,\n  \"warmup_samples\": {},\n  \"trials\": {},\n",
            "  \"device_setup_ms\": {},\n  \"setup_ms\": {},\n  \"kernel_setup_ms\": {},\n",
            "  \"warmup_ms\": {},\n  \"render_ms\": {:?},\n",
            "  \"readback_ms\": {:?},\n  \"checksum\": {:?}\n}}\n"
        ),
        json_string(&device_name),
        json_string(&device_type),
        json_string(&driver),
        json_string(&options.scene),
        options.width,
        options.height,
        options.samples,
        bounces,
        options.seed,
        options.warmup,
        options.trials,
        device_setup_ms,
        setup_ms,
        kernel_setup_ms,
        warmup_ms,
        render_ms,
        readback_ms,
        checksum,
    );
    std::fs::write(&options.output, report)?;
    println!("Wrote {}", options.output);
    Ok(())
}

fn positive(value: String) -> Result<(), String> {
    match value.parse::<usize>() {
        Ok(n) if n > 0 => Ok(()),
        _ => Err("expected a positive integer".into()),
    }
}

fn main() -> proc::Result<()> {
    let mut app = clap::App::new("OpenCL benchmark")
        .about("Synchronized render and readback timings for the shared eu/hy scenes")
        .ocl_args()
        .arg(
            clap::Arg::with_name("scene")
                .long("scene")
                .takes_value(true)
                .possible_values(&["eu", "hy"])
                .default_value("hy"),
        );
    for (name, default) in &[
        ("width", "640"),
        ("height", "480"),
        ("samples", "16"),
        ("trials", "5"),
    ] {
        app = app.arg(
            clap::Arg::with_name(name)
                .long(name)
                .takes_value(true)
                .default_value(default)
                .validator(positive),
        );
    }
    let matches = app
        .arg(
            clap::Arg::with_name("warmup")
                .long("warmup")
                .takes_value(true)
                .default_value("2")
                .help("Number of samples rendered before measured trials; zero disables warmup")
                .validator(|s| {
                    s.parse::<usize>()
                        .map(|_| ())
                        .map_err(|_| "expected a nonnegative integer".to_string())
                }),
        )
        .arg(
            clap::Arg::with_name("seed")
                .long("seed")
                .takes_value(true)
                .default_value("3735928559")
                .validator(|s| {
                    s.parse::<u32>()
                        .map(|_| ())
                        .map_err(|_| "expected a 32-bit unsigned decimal integer".to_string())
                }),
        )
        .arg(
            clap::Arg::with_name("output")
                .long("output")
                .takes_value(true)
                .default_value("opencl-benchmark.json")
                .help("Exact path for the machine-readable JSON timing report"),
        )
        .get_matches();
    let device_start = Instant::now();
    let backend = match get_ocl_context(&matches)? {
        Some(backend) => backend,
        None => return Ok(()),
    };
    let device_setup_ms = device_start.elapsed().as_secs_f64() * 1000.0;
    let options = Options {
        scene: matches.value_of("scene").unwrap().into(),
        width: matches.value_of("width").unwrap().parse().unwrap(),
        height: matches.value_of("height").unwrap().parse().unwrap(),
        samples: matches.value_of("samples").unwrap().parse().unwrap(),
        trials: matches.value_of("trials").unwrap().parse().unwrap(),
        warmup: matches.value_of("warmup").unwrap().parse().unwrap(),
        seed: matches.value_of("seed").unwrap().parse().unwrap(),
        output: matches.value_of("output").unwrap().into(),
    };
    let context = Context {
        backend,
        config: Config {
            address_width: AddressWidth::X32,
            endian: Endian::Little,
            double_support: false,
        },
    };
    match options.scene.as_str() {
        "eu" => benchmark(
            &context,
            scenes::eu::scene::<4>(),
            &options,
            4,
            device_setup_ms,
        ),
        "hy" => benchmark(
            &context,
            scenes::hy::scene::<3>(),
            &options,
            3,
            device_setup_ms,
        ),
        _ => unreachable!("clap validates the scene"),
    }
}
