use std::{
    rc::Rc,
    cell::RefCell,
    thread::sleep,
    time::Duration,
};
use base::Image;
use view::{Window, Handler, DummyController};
use proc::{Render, Canvas, Converter, Buffer};
use types::{
    Config, AddressWidth, Endian,
    ObjectType, Object,
    wrap_def, wrap_impl, wrap_impl_sized, wrap_impl_unit,
};
use vecmat::Vector;

type ColorMap = [Vector<f32, 4>; 2];
wrap_def!(DummyType, Dummy, ColorMap);
wrap_impl!(DummyType, Dummy, ColorMap);
wrap_impl_sized!(DummyType, Dummy, ColorMap);
wrap_impl_unit!(DummyType, Dummy, ColorMap);

impl ObjectType for DummyType {
    fn name(&self) -> String {
        String::from("dummy")
    }
    fn source(&self) -> String {
        String::from(r#"
        typedef struct {
            float4 colors[2];
        } dummy;
        float4 dummy_render(__global const dummy *object, float2 pos) {
            return object->colors[0] * pos.x + object->colors[1] * pos.y;
        }
        "#)
    }
}

impl Object for Dummy {}


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
    let object = Dummy([
        Vector::from((0.0, 0.0, 1.0, 1.0)),
        Vector::from((0.0, 1.0, 0.0, 1.0)),
    ]);
    let buffer = Buffer::new(&ocl_context, &config, &object)?;
    let render = Render::new(&ocl_context, DummyType::default())?;
    let converter = Converter::new(&ocl_context, shape)?;

    let sdl_context = Rc::new(sdl2::init()?);
    let mut window = Window::new(sdl_context.clone(), shape, "Sample")?;
    let mut handler = Handler::new(sdl_context, Rc::new(RefCell::new(DummyController)))?;

    let mut canvas = Canvas::new(&ocl_context, shape)?;
    let mut image = Image::new(shape);

    loop {
        if handler.poll()? {
            break Ok(());
        }
        render.render(&ocl_queue, &buffer, &mut canvas)?;
        converter.convert_canvas_to_image(&ocl_queue, &canvas, &mut image)?;
        window.draw(&image)?;
        sleep(Duration::from_millis(40));
    }
}
