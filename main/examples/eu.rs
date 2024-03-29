use ccgeom::{Euclidean3, Homogenous3};
use hypertrace::{
    cli::{get_ocl_context, OclApp},
    objects::{
        background::GradBg,
        material::{Colored, Lambertian, Refractive, Specular},
        mixture,
        object::Covered,
        shape::{Cube, Plane, Sphere},
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
use vecmat::{
    transform::{Rotation3, Shift},
    Transform, Vector,
};

shape_choice! {
    Choice {
        Plane(Plane),
        Sphere(Sphere),
        Cube(Cube),
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

    let view = Mapped::new(PointView::new(1.0), Homogenous3::identity());
    let objects = vec![
        Mapped::new(
            Covered::new(
                Choice::from(Sphere::default()),
                Mixture::new(
                    (Colored::new(Lambertian, [1.0, 0.2, 0.2].into()), 0.0).into(),
                    (Specular, 0.1).into(),
                    (
                        Colored::new(Refractive::new(1.2), [1.0, 1.0, 0.2].into()),
                        0.9,
                    )
                        .into(),
                ),
            ),
            Shift::from_vector([0.0, 1.0, 0.0].into()),
        ),
        Mapped::new(
            Covered::new(
                Choice::from(Cube::default()),
                Mixture::new(
                    (Colored::new(Lambertian, [0.2, 0.8, 0.8].into()), 1.0).into(),
                    (Specular, 0.0).into(),
                    (
                        Colored::new(Refractive::new(1.0), [1.0, 1.0, 1.0].into()),
                        0.0,
                    )
                        .into(),
                ),
            ),
            Shift::from_vector([0.0, -1.0, 0.0].into()),
        ),
        Mapped::new(
            Covered::new(
                Choice::from(Plane::default()),
                Mixture::new(
                    (Colored::new(Lambertian, [1.0, 1.0, 1.0].into()), 0.9).into(),
                    (Specular, 0.1).into(),
                    (
                        Colored::new(Refractive::new(1.0), [1.0, 1.0, 1.0].into()),
                        0.0,
                    )
                        .into(),
                ),
            ),
            Shift::from_vector([0.0, 0.0, -1.0].into()),
        ),
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
