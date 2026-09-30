# Example scenes

Each scene has a self-contained directory under `src/bin/<name>/`:

- `scene.rs` constructs its camera, shapes, materials, lights, and medium using
  `objects` and `ccgeom` directly.
- `main.rs` lowers that construction into a renderer scene, creates `Renderer`
  and `Presenter`, and owns the window, input, resize, and frame loop.

Start with [Euclidean construction](src/bin/eu/scene.rs) and its
[application entry point](src/bin/eu/main.rs), or explore the
[hyperbolic](src/bin/hy/scene.rs) and [spherical](src/bin/sp/scene.rs) constructions.
You can copy one directory into your own application and edit it without adopting
an examples library, shared runner, or catalogue.

The separate `gallery/` package imports these example-owned scene files. Their
`hypertrace_gallery::EXAMPLES` catalogue contains display metadata.
Run a scene by its binary name; `viewer`, `headless`, and `benchmark` accept
`--scene NAME` and `--list-scenes`. The browser's grouped Example menu uses the same list, and
`?scene=NAME` selects an example directly. Changing the browser selection reloads
the selected application; each running renderer keeps its geometry type.

From the repository root:

```sh
cargo run --release -p hypertrace-gallery --bin headless -- --list-scenes
cargo run --release -p hypertrace-examples --bin sp
cargo run --release -p hypertrace-gallery --bin headless -- \
  --scene eu-fog --width 640 --height 480 --samples 256 --output /tmp/eu-fog
```

Headless output includes a PPM preview, linear RGBA floats, and JSON settings
recording the curvature sign, radius, and medium alongside the render options.
Start with 256 samples and increase the count for fog and soft shadows;
scattering and indirect illumination need more samples to settle.

| Example | What to observe | Default path events |
| --- | --- | ---: |
| `eu` | Glass, diffuse surfaces, and a directional background in flat space. | 4 |
| `hy` | Pentagonal plane tilings and tiled horospheres. | 3 |
| `sp` | Asymmetric sunlight, diffuse and refractive balls, and a glowing sphere crossing the floor. | 6 |
| `eu-fog` | Small lights, opaque companions and a glass sphere in dark, floorless fog. | 12 |

Surface and volume interactions both consume the path-event budget. Headless
and benchmark tools accept `--bounces` to override the defaults.

## Spherical shadows

The [spherical studio](src/bin/sp/scene.rs) has red and clear balls resting at
opposite floor poles, a small weakly refracting blue ball near the red one, and
a green emitter intersecting the plane in a dark region. The low, off-center sun
casts long asymmetric shadows. The beacon is a quarter circuit from the sun's
floor footprint, where direct sunlight is weakest; the antipodal point brightens
again as spherical rays converge. The starting view shows the red and blue balls
and the beacon; turn around to find the clear ball.
The floor material is 85% diffuse, 5% specular, and 10% transparent. Look for
contact shadows, refracted light, and faint reflections. There is no ambient light.

The separate `eu-fog` example has no plane. Two small distant lights are surrounded
by opaque balls, with one larger glass sphere in front. Tune the emitters,
extinction and scattering albedo directly in its `scene.rs`. With small emitters,
look near the lights for scattered illumination and shadows. Begin with thousands
of samples for these small emitters. Volumetric caustics converge slowly with the
current camera-path sampler, and the scene-wide medium also occupies the glass.
Neither example adds environmental light or independently glowing fog.

Left-drag to look, or press Tab to lock/unlock the mouse. Scroll to zoom, use
WASD/arrows to move, Space/C for up/down,
and Q/E to roll. R restores the initial camera. Leave the camera still while
samples accumulate. Mouse lock releases on focus loss. Escape exits native
applications; in the browser it releases mouse lock, or toggles pause when unlocked.

## Build your own scene

An example's construction is ordinary Rust using library traits. For a minimal
spherical scene with an emissive sphere:

```rust,ignore
use ccgeom::Spherical3;
use objects::{Scene as _, SceneImpl, background::ConstBg,
    material::{Absorbing, Emissive}, object::Covered,
    shape::GeodesicSphere, view::PointView};

let source = SceneImpl::<Spherical3, _, _, _, 4>::new(
    PointView::new(1.0),
    Covered::new(
        GeodesicSphere::new(0.7),
        Emissive::new(Absorbing, [1.0, 0.8, 0.6].into()),
    ),
    ConstBg::new([0.0; 3].into()),
);
let definition = source.definition()?;
let scene = hypertrace_renderer::Scene::from_definition(&definition)?;
let renderer = hypertrace_renderer::Renderer::new_async(
    device, queue, (640, 480), scene, 1,
).await?;
```

This camera is inside the glowing sphere. Use `Mapped` and
`EmbeddedIsometry<f64, K>` to place objects and the camera. Tuples compose
heterogeneous objects; vectors hold repeated objects of one type. `Flat3`,
`Hyperboloid3`, and `Spherical3` select geometry at compile time. Curved scenes
set a physical `radius`; construct matching displacements with `Space3::new(R)`.
The [fog example](src/bin/eu-fog/scene.rs) sets the medium explicitly.

The complete `main.rs` files show how to obtain a device and queue from Wgame,
resize the renderer, rebind presentation, update the camera, and submit frames.
They share no application runner or source includes. Small amounts of window and
input scaffolding repeat intentionally so each directory is readable on its own.
Use the example dependencies in [Cargo.toml](Cargo.toml) when creating a separate
package: `objects`, `hypertrace-renderer`, `ccgeom`, `vecmat`, `wgame`, `wgpu`, and
`anyhow`, plus `winit` for native cursor capture. Wgame owns the window; Hypertrace accepts caller-owned devices.

All four scene binaries are native applications and accept `--help` and a
bounded `--smoke` check. The separate gallery package enables its `viewer` feature
by default; `cargo build -p hypertrace-gallery --no-default-features` builds its
headless tools without Wgame. The optional `viewer`, `headless`, and
`benchmark` applications use `hypertrace_gallery` to select a concrete typed
example at startup; they never become dependencies of a standalone scene binary.

The dependency direction is `examples` → `renderer` / `objects` → `scene`.
Application tests, gallery controls, and web assets live under `gallery/`. Renderer tests
use their own fixtures and remain independent of these demonstration sources.
