use std::{
    rc::Rc,
    time::{Duration, Instant},
    f64::consts::PI,
};
use vecmat::{transform::{Moebius}};
use ccgeom::{Geometry3, Hyperbolic3};
use hypertrace::{
    objects::{
        background::ConstBg,
        material::{Colored, Lambertian, Specular, Refractive},
        object::Covered,
        shape::{Plane, Horosphere},
        view::PointView,
        Mapped,
        SceneImpl,
        shape_choice,
        mixture,
    },
    proc::{filter::GammaFilter, Context, Pipeline},
    types::{
        config::{AddressWidth, Endian},
        Config,
    },
    view::{controllers::IsotropicController, Controller, Window, PollStatus},
    cli::{OclApp, get_ocl_context},
};

shape_choice! {
    Choice {
        Plane(Plane),
        Horosphere(Horosphere),
    }
}

mixture! {
    Mixture {
        diffuse: Colored<Lambertian>,
        specular: Specular,
        refractive: Colored<Refractive>,
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

    let view = Mapped::new(PointView::new(1.0), Moebius::identity());
    let objects = vec![
        Covered::new(
            Mapped::new(
                Plane::default(),
                Hyperbolic3::rotate_y(-0.25 * PI),
            ),
            Colored::new(Lambertian, [0.8, 0.2, 0.2].into()),
        ),
        Covered::new(
            Mapped::new(
                Plane::default(),
                Hyperbolic3::rotate_y(0.25 * PI),
            ),
            Colored::new(Lambertian, [0.2, 0.2, 0.8].into()),
        ),
        Covered::new(
            Mapped::new(
                Plane::default(),
                Hyperbolic3::rotate_y(0.25 * PI).chain(Hyperbolic3::shift_z(0.1)),
            ),
            Colored::new(Lambertian, [0.2, 0.8, 0.2].into()),
        ),
    ];
    let background = ConstBg::new([1.0, 1.0, 1.0].into());
    let mut scene = SceneImpl::<Hyperbolic3, _, _, _, 3>::new(view, objects, background);
    let filter = GammaFilter::new(&context.backend, 1.0 / 2.2)?;
    let mut pipeline = Pipeline::new(&context, size, &scene, filter)?;

    let sdl_context = Rc::new(sdl2::init()?);
    let mut window = Window::new(sdl_context, size, "Sample")?;

    let mut controller = IsotropicController::<Hyperbolic3>::new(
        Hyperbolic3::shift_z(2.0),
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
