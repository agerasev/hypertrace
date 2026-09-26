//! Deterministic, window-free OpenCL reference for backend comparisons.

#[path = "support/eu.rs"]
mod eu;
#[path = "support/hy.rs"]
mod hy;

use hypertrace::{
    cli::{get_ocl_context, OclApp},
    objects::Scene,
    proc::{Canvas, Context, EntityBuffer, Render},
    types::{
        config::{AddressWidth, Endian},
        Config, Geometry,
    },
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    time::Instant,
};

struct Options {
    scene: String,
    width: usize,
    height: usize,
    samples: usize,
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

fn render<G: Geometry, S: Scene<G>>(
    context: &Context,
    scene: S,
    options: &Options,
    bounces: usize,
) -> proc::Result<()> {
    let count = options
        .width
        .checked_mul(options.height)
        .filter(|&count| count <= u32::MAX as usize / 4)
        .ok_or("Image dimensions exceed the 32-bit reference renderer limits")?;
    let scene = EntityBuffer::new(context, &scene)?;
    let mut canvas = Canvas::new(&context.backend, (options.width, options.height))?;
    let seeds: Vec<_> = (0..count)
        .map(|i| pixel_seed(options.seed, i as u32))
        .collect();
    canvas.seeds_mut().raw_mut().write(&seeds).enq()?;
    let render = Render::<G, S>::new(context)?;
    let start = Instant::now();
    for _ in 0..options.samples {
        render.render(&scene, &mut canvas)?;
    }
    let elapsed = start.elapsed();
    let mut linear = vec![0.0f32; 4 * count];
    canvas.raw_image().raw().read(&mut linear).enq()?;
    context.backend.queue.finish()?;
    for value in &mut linear {
        *value /= options.samples as f32;
    }
    if linear.iter().any(|x| !x.is_finite()) {
        return Err("Reference renderer produced a non-finite pixel".into());
    }
    let mut raw = BufWriter::new(File::create(format!("{}.rgba32f", options.output))?);
    for value in &linear {
        raw.write_all(&value.to_le_bytes())?;
    }
    raw.flush()?;
    let mut ppm = BufWriter::new(File::create(format!("{}.ppm", options.output))?);
    write!(ppm, "P6\n{} {}\n255\n", options.width, options.height)?;
    for pixel in linear.chunks_exact(4) {
        for value in &pixel[..3] {
            ppm.write_all(&[(255.0 * value.max(0.0).powf(1.0 / 2.2).min(1.0)) as u8])?;
        }
    }
    ppm.flush()?;
    let metadata = format!(
        "{{\n  \"backend\": \"opencl\",\n  \"scene\": \"{}\",\n  \"width\": {},\n  \"height\": {},\n  \"samples\": {},\n  \"seed\": {},\n  \"bounces\": {},\n  \"linear_format\": \"little-endian rgba32f, row-major, top row first\",\n  \"display_gamma\": 2.2\n}}\n",
        options.scene, options.width, options.height, options.samples, options.seed, bounces,
    );
    std::fs::write(format!("{}.json", options.output), metadata)?;
    println!(
        "OpenCL {}: {}x{}, {} samples, {} bounces in {:.3}s; wrote {}.{{rgba32f,ppm,json}}",
        options.scene,
        options.width,
        options.height,
        options.samples,
        bounces,
        elapsed.as_secs_f64(),
        options.output
    );
    Ok(())
}

fn render_scene<const H: usize>(context: &Context, options: &Options) -> proc::Result<()> {
    match options.scene.as_str() {
        "eu" => render(context, eu::scene::<H>(), options, H),
        "hy" => render(context, hy::scene::<H>(), options, H),
        _ => unreachable!("clap validates the scene"),
    }
}

fn positive(value: String) -> Result<(), String> {
    match value.parse::<usize>() {
        Ok(n) if n > 0 => Ok(()),
        _ => Err("expected a positive integer".into()),
    }
}

fn main() -> proc::Result<()> {
    let matches = clap::App::new("OpenCL reference renderer")
        .about("Reproducible eu/hy frames, without an SDL window")
        .ocl_args()
        .arg(
            clap::Arg::with_name("scene")
                .long("scene")
                .takes_value(true)
                .possible_values(&["eu", "hy"])
                .default_value("hy"),
        )
        .arg(
            clap::Arg::with_name("width")
                .long("width")
                .takes_value(true)
                .default_value("320")
                .validator(positive),
        )
        .arg(
            clap::Arg::with_name("height")
                .long("height")
                .takes_value(true)
                .default_value("240")
                .validator(positive),
        )
        .arg(
            clap::Arg::with_name("samples")
                .long("samples")
                .takes_value(true)
                .default_value("64")
                .validator(positive),
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
            clap::Arg::with_name("bounces")
                .long("bounces")
                .takes_value(true)
                .possible_values(&["1", "2", "3", "4", "5", "6", "7", "8"])
                .help("Defaults to 4 for eu and 3 for hy"),
        )
        .arg(
            clap::Arg::with_name("output")
                .long("output")
                .takes_value(true)
                .default_value("reference")
                .help("Output path prefix"),
        )
        .get_matches();
    let backend = match get_ocl_context(&matches)? {
        Some(backend) => backend,
        None => return Ok(()),
    };
    let options = Options {
        scene: matches.value_of("scene").unwrap().into(),
        width: matches.value_of("width").unwrap().parse().unwrap(),
        height: matches.value_of("height").unwrap().parse().unwrap(),
        samples: matches.value_of("samples").unwrap().parse().unwrap(),
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
    let bounces = matches
        .value_of("bounces")
        .map(|s| s.parse().unwrap())
        .unwrap_or(if options.scene == "eu" { 4 } else { 3 });
    match bounces {
        1 => render_scene::<1>(&context, &options),
        2 => render_scene::<2>(&context, &options),
        3 => render_scene::<3>(&context, &options),
        4 => render_scene::<4>(&context, &options),
        5 => render_scene::<5>(&context, &options),
        6 => render_scene::<6>(&context, &options),
        7 => render_scene::<7>(&context, &options),
        8 => render_scene::<8>(&context, &options),
        _ => unreachable!("clap validates the bounce limit"),
    }
}
