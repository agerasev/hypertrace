use std::{
    rc::Rc,
    time::{Instant, Duration},
};
use vecmat::{Vector, Transform, transform::{Shift, Rotation3}};
use base::Image;
use view::{Window, Controller, controllers::IsotropicController};
use processing::{Context, Render, Canvas, Extractor, Packer, Buffer, ImageBuffer, Filter, filter::GammaFilter};
use types::{
    Config, config::{AddressWidth, Endian},
};
use objects::{
    SceneImpl,
    view::{PointView, MappedView},
    shape::{euclidean::Sphere, MappedShape},
    object::{Covered, ObjectVector},
    material::{Lambertian, Colored},
    background::{GradBg},
};
use ccgeom::{Euclidean3, Homogenous3};

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

    let size = (800, 600);

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

    let view = MappedView::new(
        PointView::new(1.0),
        Homogenous3::identity(),
    );
    let objects = ObjectVector::from_objects(vec![
        Covered::new(
            MappedShape::new(
                Sphere::default(),
                Shift::from_vector([1.0, 0.0, 0.0].into()),
            ),
            Colored::new(Lambertian, [0.8, 0.2, 0.2].into()),
        ),
        Covered::new(
            MappedShape::new(
                Sphere::default(),
                Shift::from_vector([-1.0, 0.0, 0.0].into()),
            ),
            Colored::new(Lambertian, [0.2, 0.2, 0.8].into()),
        ),
    ]);
    let background = GradBg::new(
        [0.0, 1.0, 0.0].into(),
        [[1.0, 1.0, 1.0].into(), [0.0, 0.0, 0.0].into()],
    );
    let mut scene = SceneImpl::<_, _, _, _, 4>::new(view, objects, background);
    let render = Render::new(&context)?;
    let extractor = Extractor::new(&context.ocl)?;
    let packer = Packer::new(&context.ocl)?;
    let filter = GammaFilter::new(&context.ocl, 1.0 / 2.2)?;

    let sdl_context = Rc::new(sdl2::init()?);
    let mut window = Window::new(sdl_context, size, "Sample")?;

    let mut canvas = Canvas::new(&context.ocl, size)?;
    let mut image_buf_src = ImageBuffer::new(&context.ocl, size)?;
    let mut image_buf_dst = ImageBuffer::new(&context.ocl, size)?;
    let mut image_buf_packed = ImageBuffer::new(&context.ocl, size)?;
    let mut image = Image::new(size);

    let mut controller = IsotropicController::<Euclidean3>::new(
        Homogenous3::new(Shift::from(Vector::from([0.0, 0.0, 4.0])), Rotation3::identity()),
        1.0,
    );
    let delay = 0.04;
    loop {
        if window.poll(&mut controller)? {
            break Ok(());
        }
        let updated = controller.updated();
        controller.step(delay);
        if updated {
            scene.view.map = *controller.map();
            scene.view.inner.fov = 1.0 / controller.zoom();
            canvas.clean(&ocl_queue)?;
        }
        let buffer = Buffer::new(&context, &scene)?;

        let now = Instant::now();
        while now.elapsed() < Duration::from_secs_f64(delay) {
            render.render(&ocl_queue, &buffer, &mut canvas)?;
        }

        extractor.process(&ocl_queue, &canvas, &mut image_buf_src)?;
        filter.process(&ocl_queue, &image_buf_src, &mut image_buf_dst)?;
        packer.process(&ocl_queue, &image_buf_dst, &mut image_buf_packed)?;
        image_buf_packed.load(&ocl_queue, &mut image)?;

        window.draw(&image)?;
    }
}
