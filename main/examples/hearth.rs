use ccgeom::{Geometry3, Hyperbolic3};
use hypertrace::{
    cli::{get_ocl_context, OclApp},
    objects::{
        background::ConstBg,
        material::{Absorbing, Colored, Emissive, Lambertian, Specular},
        mixture,
        object::Covered,
        shape::{Horosphere, Plane},
        shape_choice,
        view::PointView,
        Mapped, SceneImpl,
    },
    proc::{filter::GammaFilter, Context, Pipeline},
    types::{
        config::{AddressWidth, Endian},
        Config,
    },
    view::{controllers::IsotropicController, Controller, PollStatus, Window},
};
use std::{
    rc::Rc,
    time::{Duration, Instant},
};
use vecmat::transform::Moebius;

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
        luminous: Emissive<Absorbing>,
    }
}

fn main() -> proc::Result<()> {
    let matches = clap::App::new("Hearth")
        .about("Hyper earth")
        .ocl_args()
        .get_matches();

    let ocl_context = match get_ocl_context(&matches)? {
        Some(ctx) => ctx,
        None => {
            return Ok(());
        }
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
        // Earth
        Mapped::new(
            Covered::new(
                Choice::from(Plane::default()),
                Mixture::new(
                    (Colored::new(Lambertian, [0.4, 0.8, 0.2].into()), 1.0).into(),
                    (Specular, 0.0).into(),
                    (Emissive::new(Absorbing, [0.0, 0.0, 0.0].into()), 0.0).into(),
                ),
            ),
            Moebius::identity(),
        ),
        // Sun
        Mapped::new(
            Covered::new(
                Choice::from(Plane::default()),
                Mixture::new(
                    (Colored::new(Lambertian, [1.0, 1.0, 1.0].into()), 0.0).into(),
                    (Specular, 0.0).into(),
                    (Emissive::new(Absorbing, [100.0, 100.0, 80.0].into()), 1.0).into(),
                ),
            ),
            Hyperbolic3::shift_z(-3.0),
        ),
        // Moon
        Mapped::new(
            Covered::new(
                Choice::from(Plane::default()),
                Mixture::new(
                    (Colored::new(Lambertian, [0.8, 0.8, 0.8].into()), 1.0).into(),
                    (Specular, 0.0).into(),
                    (Emissive::new(Absorbing, [1.0, 1.0, 1.0].into()), 0.0).into(),
                ),
            ),
            Hyperbolic3::shift_x(-1.0).chain(Hyperbolic3::shift_z(-2.0)),
        ),
    ];
    let background = ConstBg::new([0.01, 0.01, 0.02].into());
    let mut scene = SceneImpl::<Hyperbolic3, _, _, _, 4>::new(view, objects, background);
    let filter = GammaFilter::new(&context.backend, 1.0 / 2.2)?;
    let mut pipeline = Pipeline::new(&context, size, &scene, filter)?;

    let sdl_context = Rc::new(sdl2::init()?);
    let mut window = Window::new(sdl_context, size, "Sample")?;

    let mut controller = IsotropicController::<Hyperbolic3>::new(Hyperbolic3::shift_z(1.0), 1.0);

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
