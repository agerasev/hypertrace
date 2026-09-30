# Example scenes

Each scene has a self-contained directory under `src/bin/<name>/`:

- `scene.rs` constructs its camera, shapes, materials, lights, and medium using
  `objects` and `ccgeom` directly.
- `main.rs` lowers that construction into a renderer scene, creates `Renderer`
  and `Presenter`, and owns the window, input, resize, and frame loop.

Start with [Euclidean construction](src/bin/euclidean/scene.rs) and its
[application entry point](src/bin/euclidean/main.rs), or explore the
[hyperbolic](src/bin/hyperbolic/scene.rs) and [spherical](src/bin/spherical/scene.rs) constructions.
You can copy one directory into your own application and edit it without adopting
an examples library, shared runner, or catalogue.

The geometry demonstrations use full names: `euclidean`, `hyperbolic`, and
`spherical`. Descriptive names without a geometry prefix, such as `fog` and
`ball-tilings`, use Euclidean space.

The separate `gallery/` package imports these example-owned scene files. Their
`hypertrace_gallery::EXAMPLES` catalogue contains display metadata.
Run a scene by its binary name; `viewer`, `headless`, and `benchmark` accept
`--scene NAME` and `--list-scenes`. The browser's grouped Example menu uses the same list, and
`?scene=NAME` selects an example directly. Changing the browser selection reloads
the selected application; each running renderer keeps its geometry type.

From the repository root:

```sh
cargo run --release -p hypertrace-gallery --bin headless -- --list-scenes
cargo run --release -p hypertrace-examples --bin spherical
cargo run --release -p hypertrace-examples --bin ball-tilings
cargo run --release -p hypertrace-gallery --bin headless -- \
  --scene fog --width 640 --height 480 --samples 256 --output /tmp/fog
```

Headless output includes a PPM preview, linear RGBA floats, and JSON settings
recording the curvature sign, radius, and medium alongside the render options.
Start with 256 samples and increase the count for fog and soft shadows;
scattering and indirect illumination need more samples to settle.

| Example | What to observe | Default path events |
| --- | --- | ---: |
| `euclidean` | A glass sphere, diffuse cube and square-tiled backdrop. | 4 |
| `hyperbolic` | Pentagonal planes, square/hexagonal horospheres, and a dodecahedrally tiled sphere. | 3 |
| `spherical` | Diffuse and refractive balls on a dodecahedrally tiled floor under asymmetric sunlight. | 6 |
| `fog` | One light with red diffuse, green reflective and blue refractive spheres in Euclidean fog. | 12 |
| `ball-tilings` | All five Platonic tilings, eight lunes and two hemispheres on equal-sized matte balls. | 4 |

Surface and volume interactions both consume the path-event budget. Headless
and benchmark tools accept `--bounces` to override the defaults.

## Surface patterns

The same `Tiled::new(shape, pattern, materials, border)` construction appears in
all three geometry examples. The surface determines its intrinsic domain;
ambient curvature remains a compile-time geometry type. A Euclidean plane and a
hyperbolic horosphere share the same square/hexagonal selector. Every round
sphere supports the regular spherical families, and so does a spherical plane.
See the [tiling API](../renderer/README.md#intrinsic-surface-tilings) for domains,
units and custom extension traits.

The [ball-tilings construction](src/bin/ball-tilings/scene.rs) shows all five
Platonic patterns together, plus the lune and dihedron families. Edit the
`RegularSpherical<P,Q>` parameters to select a family; `P` counts face edges and
`Q` counts faces meeting at a vertex. The fog example keeps its plain materials
so their lighting remains easy to compare.

## Ball tilings

Run `cargo run --release -p hypertrace-examples --bin ball-tilings`, or select
`--scene ball-tilings` in a gallery tool (`?scene=ball-tilings` in the browser).
The [standalone application](src/bin/ball-tilings/main.rs) owns its renderer and
frame loop, just like the other examples.

From the initial camera, read each row left to right:

| Row | Pattern | `{P,Q}` | Faces |
| --- | --- | --- | ---: |
| Top | Tetrahedral | `{3,3}` | 4 triangles |
| Top | Cubic | `{4,3}` | 6 quadrilaterals |
| Top | Octahedral | `{3,4}` | 8 triangles |
| Bottom | Dodecahedral | `{5,3}` | 12 pentagons |
| Bottom | Icosahedral | `{3,5}` | 20 triangles |
| Bottom | Eight lunes (hosohedron) | `{2,8}` | 8 lunes |
| Bottom | Dihedron | `{6,2}` | 2 hemispheres |

All balls have radius 0.25, the same matte palette, and border half-width 0.025
radians. A broad environment gradient illuminates them, so shadows and shiny
reflections do not obscure the patterns. The tiles form spherical polygons on
smooth balls in Euclidean space. The same patterns work on
balls in hyperbolic and spherical space because selection uses the intrinsic
surface directions.

Move around to see the hidden faces and lune poles. The dihedron's six
degree-two vertices subdivide its equator without adding face borders: it still
has just two hemispherical tiles. The scene composes seven concrete pattern types
in a tuple, using a local generic `ball::<P,Q>` helper with no runtime type erasure.

## Spherical shadows

The [spherical studio](src/bin/spherical/scene.rs) has red and clear balls resting at
opposite floor poles, a small weakly refracting blue ball on the sun-facing side
of the red one, and a green emitter intersecting the plane in a dark region.
The low, off-center sun casts long asymmetric shadows. The beacon is a quarter
circuit from the sun's floor footprint, where direct sunlight is weakest; the
antipodal point brightens again as spherical rays converge. The starting view shows
the red and blue balls and the beacon; turn around to find the clear ball.
The floor material is 90% diffuse, 5% specular, and 5% transparent. Look for
contact shadows, refracted light, and faint reflections across the pentagonal floor
tiles. The red and blue spheres use plain materials. There is no ambient light.

## Euclidean fog

The [fog example](src/bin/fog/scene.rs) places one bright neutral emitter
among three spheres: opaque red Lambertian, green specular, and blue refractive.
There is no plane or ambient light. Look for the glow around the source, the red
sphere's shadow, a green mirror highlight, and light refracted through the blue
sphere into the fog. Tune the emitter, extinction and scattering albedo directly
in `scene.rs`.

Begin with thousands of samples. Volumetric caustics converge slowly with the
current camera-path sampler, and the scene-wide medium also occupies the glass.
The fog scatters light from the emitter; it does not emit light itself.

## Camera controls

The Euclidean glass and ball-tiling scenes use 0.25-sized objects, comparable to
the curved-space balls and the 0.28-radius fog balls. Their camera distances and
spacing use the same scale, so the shared movement speed feels consistent.

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
The [fog example](src/bin/fog/scene.rs) sets the medium explicitly.

The complete `main.rs` files show how to obtain a device and queue from Wgame,
resize the renderer, rebind presentation, update the camera, and submit frames.
They share no application runner or source includes. Small amounts of window and
input scaffolding repeat intentionally so each directory is readable on its own.
Use the example dependencies in [Cargo.toml](Cargo.toml) when creating a separate
package: `objects`, `hypertrace-renderer`, `ccgeom`, `vecmat`, `wgame`, `wgpu`, and
`anyhow`, plus `winit` for native cursor capture. Wgame owns the window; Hypertrace accepts caller-owned devices.

All five scene binaries are native applications and accept `--help` and a
bounded `--smoke` check. The separate gallery package enables its `viewer` feature
by default; `cargo build -p hypertrace-gallery --no-default-features` builds its
headless tools without Wgame. The optional `viewer`, `headless`, and
`benchmark` applications use `hypertrace_gallery` to select a concrete typed
example at startup; they never become dependencies of a standalone scene binary.

The dependency direction is `examples` → `renderer` / `objects` → `scene`.
Application tests, gallery controls, and web assets live under `gallery/`. Renderer tests
use their own fixtures and remain independent of these demonstration sources.
