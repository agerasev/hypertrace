use ccgeom::{Euclidean3, Homogenous3};
use objs::{
    background::GradBg,
    material::{Colored, Lambertian},
    object::Covered,
    shape::{Sphere, Cube},
    view::PointView,
    Mapped,
    SceneImpl,
    object_choice,
};
use proc::{filter::GammaFilter, Context, Pipeline};
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
use lib::cli::{OclApp, get_ocl_context};

object_choice! {
    Choice(ChoiceCache) {
        Sphere(Covered<Euclidean3, Mapped<Euclidean3, Sphere, Shift<f64, 3>>, Colored<Lambertian>>),
        Cube(Covered<Euclidean3, Mapped<Euclidean3, Cube, Shift<f64, 3>>, Colored<Lambertian>>),
    }
}

fn main() -> proc::Result<()> {
    let matches = clap::App::new("Sample")
        .about("Hypertrace sample")
        .ocl_args()
        .get_matches();

    let ocl_context = match get_ocl_context(&matches)? {
        Some(ctx) => ctx,
        None => { return Ok(()); },
    };
    let config = Config {
        address_width: AddressWidth::X32,
        endian: Endian::Little,
        double_support: false,
    };
    let context = Context {
        config,
        backend: ocl_context,
    };

    let size = (800, 600);

    let view = Mapped::new(PointView::new(1.0), Homogenous3::identity());
    let objects: Vec<Choice> = vec![
        Covered::new(
            Mapped::new(
                Sphere::default(),
                Shift::from_vector([0.0, 1.0, 0.0].into()),
            ),
            Colored::new(Lambertian, [0.8, 0.2, 0.2].into()),
        ).into(),
        Covered::new(
            Mapped::new(
                Cube::default(),
                Shift::from_vector([0.0, -1.0, 0.0].into()),
            ),
            Colored::new(Lambertian, [0.2, 0.2, 0.8].into()),
        ).into(),
    ];
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
            Shift::from(Vector::from([0.0, 0.5, 2.0])),
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
