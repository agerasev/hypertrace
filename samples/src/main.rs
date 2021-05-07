use std::{
    rc::Rc,
    thread::sleep,
    time::Duration,
};
use vecmat::{Vector, transform::Shift};
use base::Image;
use view::{Window, EmptyController};
use processing::{Context, Render, Canvas, Converter, Buffer};
use types::{
    Config, config::{AddressWidth, Endian},
};
use objects::{Scene, view::{PointView, MappedView}, shape::eu as shapes};

fn main() -> base::Result<()> {
    let matches = clap::App::new("Sample")
        .about("Hypertrace sample")
        .arg(clap::Arg::with_name("platform_no")
            .help("Platform number")
            .index(1))
        .arg(clap::Arg::with_name("device_no")
            .help("Device number")
            .index(2))
        .arg(clap::Arg::with_name("list")
            .short("l")
            .long("list")
            .help("List available platform and devices")
            .takes_value(false))
        .get_matches();

    if matches.is_present("list") {
        for (pi, platform) in ocl::Platform::list().into_iter().enumerate() {
            for (di, device) in ocl::Device::list_all(&platform)?.into_iter().enumerate() {
                println!("Platform [{}]: {}", pi, platform.name()?);
                println!("    Device [{}]: {} {}", di, device.vendor()?, device.name()?);
            }
        }
        return Ok(());
    }

    let shape = (800, 600);

    let platform_no = matches.value_of("platform_no").map(|s| s.parse::<usize>().unwrap()).unwrap_or(0);
    let device_no = matches.value_of("device_no").map(|s| s.parse::<usize>().unwrap()).unwrap_or(0);

    let platform = *ocl::Platform::list().get(platform_no).unwrap();
    let device = *ocl::Device::list_all(&platform)?.get(device_no).unwrap();

    let ocl_context = ocl::Context::builder().platform(platform).devices(device).build()?;
    let ocl_queue = ocl::Queue::new(&ocl_context, device, None)?;

    let config = Config {
        address_width: AddressWidth::X32,
        endian: Endian::Little,
        double_support: false,
    };
    let context = Context {
        config,
        ocl: ocl_context
    };

    let view = MappedView::new(PointView::new(1.0), Shift::from(Vector::from([0.0, 0.0, 4.0])));
    let object = shapes::Sphere::default();
    let scene = Scene::new(view, object);
    let buffer = Buffer::new(&context, &scene)?;
    let render = Render::new(&context)?;
    let converter = Converter::new(&context.ocl, shape)?;

    let sdl_context = Rc::new(sdl2::init()?);
    let mut window = Window::new(sdl_context, shape, "Sample")?;

    let mut canvas = Canvas::new(&context.ocl, shape)?;
    let mut image = Image::new(shape);

    loop {
        if window.poll(&mut EmptyController)? {
            break Ok(());
        }
        render.render(&ocl_queue, &buffer, &mut canvas)?;
        converter.convert_canvas_to_image(&ocl_queue, &canvas, &mut image)?;
        window.draw(&image)?;
        sleep(Duration::from_millis(40));
    }
}
