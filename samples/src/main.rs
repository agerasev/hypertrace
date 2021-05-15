use ccgeom::{Euclidean3, Homogenous3};
use objs::{
    background::GradBg,
    material::{Colored, Lambertian},
    object::{Covered, ObjectVector},
    shape::{euclidean::Sphere, MappedShape},
    view::{MappedView, PointView},
    SceneImpl,
};
use proc::{filter::GammaFilter, Context, OclContext, Pipeline};
use std::{
    rc::Rc,
    time::{Duration, Instant},
};
use types::{
    config::{AddressWidth, Endian},
    Config,
};
use vecmat::{
    transform::{Rotation3, Shift},
    Transform, Vector,
};
use view::{controllers::IsotropicController, Controller, Window, PollStatus};

fn main() -> proc::Result<()> {
    let matches = clap::App::new("Sample")
        .about("Hypertrace sample")
        .arg(
            clap::Arg::with_name("platform_no")
                .help("Platform number")
                .index(1),
        )
        .arg(
            clap::Arg::with_name("device_no")
                .help("Device number")
                .index(2),
        )
        .arg(
            clap::Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List available platform and devices")
                .takes_value(false),
        )
        .get_matches();

    if matches.is_present("list") {
        for (pi, platform) in ocl::Platform::list().into_iter().enumerate() {
            for (di, device) in ocl::Device::list_all(&platform)?.into_iter().enumerate() {
                println!("Platform [{}]: {}", pi, platform.name()?);
                println!(
                    "    Device [{}]: {} {}",
                    di,
                    device.vendor()?,
                    device.name()?
                );
            }
        }
        return Ok(());
    }

    let size = (800, 600);

    let platform_no = matches
        .value_of("platform_no")
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap_or(0);
    let device_no = matches
        .value_of("device_no")
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap_or(0);

    let platform = *ocl::Platform::list().get(platform_no).unwrap();
    let device = *ocl::Device::list_all(&platform)?.get(device_no).unwrap();

    let ocl_context = ocl::Context::builder()
        .platform(platform)
        .devices(device)
        .build()?;
    let ocl_queue = ocl::Queue::new(&ocl_context, device, None)?;

    let config = Config {
        address_width: AddressWidth::X32,
        endian: Endian::Little,
        double_support: false,
    };
    let context = Context {
        config,
        backend: OclContext {
            context: ocl_context,
            queue: ocl_queue,
        },
    };

    let view = MappedView::new(PointView::new(1.0), Homogenous3::identity());
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
        2.4,
    );
    let mut scene = SceneImpl::<_, _, _, _, 4>::new(view, objects, background);
    let filter = GammaFilter::new(&context.backend, 1.0 / 2.2)?;
    let mut pipeline = Pipeline::new(&context, size, &scene, filter)?;

    let sdl_context = Rc::new(sdl2::init()?);
    let mut window = Window::new(sdl_context, size, "Sample")?;

    let mut controller = IsotropicController::<Euclidean3>::new(
        Homogenous3::new(
            Shift::from(Vector::from([0.0, 0.0, 4.0])),
            Rotation3::identity(),
        ),
        1.0,
    );
    let delay = 0.04;
    loop {
        if let PollStatus::Exit = window.poll(&mut controller)? {
            break Ok(());
        }
        let updated = controller.updated();
        controller.step(delay);
        if updated {
            scene.view.map = *controller.map();
            scene.view.inner.fov = 1.0 / controller.zoom();
            pipeline.clean()?;
        }
        pipeline.set_scene(&scene)?;

        let now = Instant::now();
        while now.elapsed() < Duration::from_secs_f64(delay) {
            pipeline.render()?;
        }

        window.draw(pipeline.get_image()?)?;
    }
}
