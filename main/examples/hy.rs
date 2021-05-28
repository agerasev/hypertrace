use std::{
    rc::Rc,
    time::{Duration, Instant},
    f64::{NAN, consts::PI},
};
use vecmat::{transform::{Moebius}, Vector, Matrix, Complex};
use ccgeom::{Geometry3, Hyperbolic3};
use hypertrace::{
    objects::{
        background::ConstBg,
        material::*,
        object::{Covered, TiledHorosphere, TiledPlane, tiling},
        shape::{Plane, Horosphere},
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

fn unpack_color(rgb: u32) -> Vector<f32, 3> {
    make_color((Vector::from([rgb >> 16, rgb >> 8, rgb]) & Vector::fill(0xff)).map(|x| (x as f32) / 255.0))
}

fn make_color(rgb: Vector<f32, 3>) -> Vector<f32, 3> {
    rgb.powf(2.2)
}

mixture! {
    Mixture {
        diffuse: Colored<Lambertian>,
        specular: Specular,
        transparent: Transparent,
    }
}

fn make_material(
    diffuse: Vector<f32, 3>,
    specularity: f64,
    transparency: f64,
    emission: Option<Vector<f32, 3>>,
) -> Emissive<Mixture> {
    Emissive::new(
        Mixture::new(
            (Colored::new(Lambertian, diffuse), 1.0 - specularity - transparency).into(),
            (Specular, specularity).into(),
            (Transparent, transparency).into(),
        ),
        emission.unwrap_or_else(|| [0.0, 0.0, 0.0].into()),
    )
}

type MyMaterial = Emissive<Mixture>;

object_choice! {
    Choice(ChoiceCache) {
        Plane(Covered<Hyperbolic3, Plane, MyMaterial>),
        PlaneStar(TiledPlane<MyMaterial, tiling::Pentastar, 2>),
        PlanePenta(TiledPlane<MyMaterial, tiling::Pentagonal, 2>),
        Horo(Covered<Hyperbolic3, Horosphere, MyMaterial>),
        HoroHexa(TiledHorosphere<MyMaterial, tiling::Hexagonal, 3>),
        HoroSquare(TiledHorosphere<MyMaterial, tiling::Square, 4>),
    }
}

fn main() -> proc::Result<()> {
    let title = "Hyperbolic";
    let matches = clap::App::new(title)
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

    let background = ConstBg::new(unpack_color(0xeeeeee));
    let border_material = make_material(unpack_color(0xe4e4e4), 0.0, 0.0, Some(make_color(Vector::fill(1.0))));
    let objects = vec![
        Mapped::new(
            Choice::HoroHexa(TiledHorosphere::new(
                [
                    make_material(unpack_color(0xfe0000), 0.1, 0.1, None),
                    make_material(unpack_color(0xffaa01), 0.1, 0.1, None),
                    make_material(unpack_color(0x35adae), 0.1, 0.1, None),
                ],
                0.5,
                0.02,
                border_material.clone(),
            )),
            Moebius::identity(),
        ),
        Mapped::new(
            Choice::HoroSquare(TiledHorosphere::new(
                [
                    make_material(unpack_color(0xfe7401), 0.1, 0.1, None),
                    make_material(unpack_color(0xfe0000), 0.1, 0.1, None),
                    make_material(unpack_color(0xffaa01), 0.1, 0.1, None),
                    make_material(unpack_color(0xfed601), 0.1, 0.1, None),
                ],
                0.5,
                0.02,
                border_material.clone(),
            )),
            Moebius::from(Matrix::from([
                [Complex::new(1.0, 0.0), Complex::new(2.0f64.sqrt(), 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            ])).chain(Hyperbolic3::rotate_x(PI)),
        ),
        Mapped::new(
            Choice::PlaneStar(TiledPlane::new(
                [
                    make_material(unpack_color(0xfe7401), 0.1, 0.0, None),
                    make_material(unpack_color(0x35adae), 0.1, 0.0, None)
                ],
                NAN,
                0.01,
                border_material.clone(),
            )),
            Moebius::identity(),
        ),
        Mapped::new(
            Choice::PlanePenta(TiledPlane::new(
                [
                    make_material(unpack_color(0xfe0000), 0.1, 0.0, None),
                    make_material(unpack_color(0xfed601), 0.1, 0.0, None)
                ],
                NAN,
                0.02,
                border_material.clone(),
            )),
            Moebius::from(Matrix::from([
                [Complex::new(1.0, 0.0), Complex::new(0.0, 2.0)],
                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            ])),
        ),
        Mapped::new(
            Choice::Horo(Covered::new(
                Horosphere,
                make_material(unpack_color(0x279eff), 0.1, 0.1, None),
            )),
            Moebius::from(Matrix::from([
                [Complex::new(1.0, 0.0), Complex::new(0.0, 2.0)],
                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            ]))
                .chain(Hyperbolic3::shift_x(2.0))
                .chain(Hyperbolic3::rotate_y(PI / 2.0)),
        ),
    ];

    let mut scene = SceneImpl::<Hyperbolic3, _, _, _, 3>::new(view, objects, background);
    let filter = GammaFilter::new(&context.backend, 1.0 / 2.2)?;
    let mut pipeline = Pipeline::new(&context, size, &scene, filter)?;

    let sdl_context = Rc::new(sdl2::init()?);
    let mut window = Window::new(sdl_context, size, title)?;

    let mut controller = IsotropicController::<Hyperbolic3>::new(
        Hyperbolic3::rotate_z(5.0 * PI / 6.0)
            .chain(Hyperbolic3::rotate_x(2.0 * PI / 5.0))
            .chain(Hyperbolic3::shift_z(2.0)),
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
