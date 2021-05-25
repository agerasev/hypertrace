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
        material::{Colored, Lambertian, Specular},
        object::{Covered, TiledHorosphere, tiling},
        shape::Plane,
        view::PointView,
        Mapped,
        SceneImpl,
        object_choice,
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

mixture! {
    Mixture {
        diffuse: Colored<Lambertian>,
        specular: Specular,
    }
}

object_choice! {
    Choice(ChoiceCache) {
        Plane(Covered<Hyperbolic3, Plane, Mixture>),
        Horosphere(TiledHorosphere<Mixture, tiling::Hexagonal, 3>),
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
        Mapped::new(
            Choice::from(TiledHorosphere::new(
                [
                    Mixture::new(
                        (Colored::new(Lambertian, [0.99, 0.01, 0.01].into()), 0.9).into(),
                        (Specular, 0.1).into(),
                    ),
                    Mixture::new(
                        (Colored::new(Lambertian, [1.0, 0.66, 0.01].into()), 0.9).into(),
                        (Specular, 0.1).into(),
                    ),
                    Mixture::new(
                        (Colored::new(Lambertian, [0.21, 0.68, 0.68].into()), 0.9).into(),
                        (Specular, 0.1).into(),
                    ),
                ],
                0.5,
                0.02,
                Mixture::new(
                    (Colored::new(Lambertian, [1.0, 1.0, 1.0].into()), 0.9).into(),
                    (Specular, 0.1).into(),
                ),
            )),
            Hyperbolic3::shift_z(-1.0).chain(Hyperbolic3::rotate_x(PI)),
        ),
        Mapped::new(
            Choice::from(
                Covered::new(
                    Plane::default(),
                    Mixture::new(
                        (Colored::new(Lambertian, [0.4, 0.8, 0.2].into()), 1.0).into(),
                        (Specular, 0.0).into(),
                    ),
                ),
            ),
            Hyperbolic3::rotate_x(0.5 * PI),
        ),
        Mapped::new(
            Choice::from(
                Covered::new(
                    Plane::default(),
                    Mixture::new(
                        (Colored::new(Lambertian, [0.4, 0.4, 0.4].into()), 0.8).into(),
                        (Specular, 0.2).into(),
                    ),
                ),
            ),
            Hyperbolic3::shift_x(-1.0).chain(Hyperbolic3::rotate_y(0.5 * PI)),
        ),
        /*
        make_horosphere(
            Moebius<comp>::identity(),
            MyHorosphere::Tiling::HEXAGONAL,
            MyHorosphereMaterials(
                make_material(make_color(0xfe0000), 0.1, 0.1),
                make_material(make_color(0xffaa01), 0.1, 0.1),
                make_material(make_color(0x35adae), 0.1, 0.1)
            ),
            0.5, 0.02,
            make_material(border_color, 0.0, 0, float3(1.0))
        ),
        make_horosphere(
            Moebius<comp>(1_r, math::sqrt(2_r), 0_r, 1_r)*Hy::xrotate(PI),
            MyHorosphere::Tiling::SQUARE,
            MyHorosphereMaterials(
                make_material(make_color(0xfe7401), 0.1, 0.1),
                make_material(make_color(0xfe0000), 0.1, 0.1),
                make_material(make_color(0xffaa01), 0.1, 0.1),
                make_material(make_color(0xfed601), 0.1, 0.1)
            ),
            0.5, 0.02,
            make_material(border_color, 0.0, 0, float3(1.0))
        ),
        make_plane(
            Moebius<comp>::identity(),
            MyPlane::Tiling::PENTASTAR,
            MyPlaneMaterials(
                make_material(make_color(0xfe7401), 0.1),
                make_material(make_color(0x35adae), 0.1)
            ),
            0.01,
            make_material(border_color, 0.0, 0, float3(1.0))
        ),
        make_plane(
            Moebius<comp>(1_r, 2*1_i, 0_r, 1_r),
            MyPlane::Tiling::PENTAGONAL,
            MyPlaneMaterials(
                make_material(make_color(0xfe0000), 0.1),
                make_material(make_color(0xfed601), 0.1)
            ),
            0.02,
            make_material(border_color, 0.0, 0, float3(1.0))
        ),
        */
    ];
    let background = ConstBg::new([1.0, 1.0, 1.0].into());
    let mut scene = SceneImpl::<Hyperbolic3, _, _, _, 3>::new(view, objects, background);
    let filter = GammaFilter::new(&context.backend, 1.0 / 2.2)?;
    let mut pipeline = Pipeline::new(&context, size, &scene, filter)?;

    let sdl_context = Rc::new(sdl2::init()?);
    let mut window = Window::new(sdl_context, size, "Sample")?;

    let mut controller = IsotropicController::<Hyperbolic3>::new(
        Hyperbolic3::shift_z(1.0),
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
