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
cargo run --release -p hypertrace-examples --bin compare-sp
cargo run --release -p hypertrace-gallery --bin headless -- \
  --scene sp-loop-fog --width 640 --height 480 --samples 256 --output /tmp/sp-loop-fog
```

Headless output includes a PPM preview, linear RGBA floats, and JSON settings
recording the curvature sign, radius, and medium alongside the render options.
Start with 64 samples for the geometric comparisons and 256 or more for fog;
scattering and indirect illumination need more samples to settle.

| Example | What to observe | Default path events |
| --- | --- | ---: |
| `eu` | Glass, diffuse surfaces, and a directional background in flat space. | 4 |
| `hy` | Pentagonal plane tilings and tiled horospheres. | 3 |
| `sp` | Diffuse, glass, and mirror spheres illuminated by emissive objects. | 6 |
| `sp-fog` | The same spherical studio with scattering fog. | 12 |
| `compare-eu` | Equal physical spheres shrinking with Euclidean distance. | 1 |
| `compare-hy` | Faster apparent shrinking at curvature −1. | 1 |
| `compare-sp` | Apparent size growing again beyond the spherical equator. | 1 |
| `compare-hy-flat` | The hyperbolic comparison at radius 3, curvature −1/9. | 1 |
| `compare-sp-flat` | The spherical comparison at radius 3, curvature +1/9. | 1 |
| `sp-loop` | Lights behind the camera arriving from ahead by the long route. | 1 |
| `sp-loop-fog` | The same long route with scattering and absorption. | 12 |

Surface and volume interactions both consume the path-event budget. Headless
and benchmark tools accept `--bounces` to override the defaults.

## Compare the curvatures

The five `compare-*` examples each construct the same physical layout locally. Every sphere has physical
radius 0.12. The amber top row is 0.8 units from the camera, the green middle row
1.6 units, and the blue bottom row 2.5 units. Their initial viewing directions,
physical distances, colors, and radii match in every preset.

At spherical radius 1, the blue row lies beyond the equator and looks larger
than the green row. In hyperbolic space it becomes especially small. Increasing
the curvature radius to 3 brings both curved layouts closer to the Euclidean
view without changing physical placements or object sizes. Curvature is `K/R²`,
where `K` is −1, 0, or +1 and `R` is the physical curvature radius.

These markers emit light and absorb incoming paths, producing clear silhouettes
without indirect-light noise. The `sp` studio supplies the shaded material
comparison; the spherical examples keep a black miss background.

## Follow the long route

In `sp-loop`, all four beacons are behind the initial camera. The central cyan
beacon is one physical unit away and has radius 0.24. Looking forward reaches
its surface after `2π − 1 − 0.24 ≈ 5.04` units; turning around reaches it after
`1 − 0.24 = 0.76` units. There is no floor to intercept the long route.

Compare that view with `sp-loop-fog`, then turn around to see the shorter route.
Fog has extinction 0.1 per physical unit, mean free flight 10 units, and
scattering albedo 0.9. A ray that misses every beacon can travel several complete
circuits before a sampled scattering event. The renderer retains that full
distance; these color images do not display or measure individual cycle counts.

Left-drag to look, scroll to zoom, use WASD/arrows to move, Space/C for up/down,
and Q/E to roll. R restores the initial camera. Reset before comparing presets
and leave the camera still while samples accumulate. Escape exits the native
viewer or toggles pause in the browser.

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
The [fog example](src/bin/sp-fog/scene.rs) sets the medium explicitly.

The complete `main.rs` files show how to obtain a device and queue from Wgame,
resize the renderer, rebind presentation, update the camera, and submit frames.
They share no application runner or source includes. Small amounts of window and
input scaffolding repeat intentionally so each directory is readable on its own.
Use the example dependencies in [Cargo.toml](Cargo.toml) when creating a separate
package: `objects`, `hypertrace-renderer`, `ccgeom`, `vecmat`, `wgame`, `wgpu`, and
`anyhow`. Wgame owns the window; Hypertrace accepts caller-owned devices.

All eleven scene binaries are native applications and accept `--help` and a
bounded `--smoke` check. The separate gallery package enables its `viewer` feature
by default; `cargo build -p hypertrace-gallery --no-default-features` builds its
headless tools without Wgame. The optional `viewer`, `headless`, and
`benchmark` applications use `hypertrace_gallery` to select a concrete typed
example at startup; they never become dependencies of a standalone scene binary.

The dependency direction is `examples` → `renderer` / `objects` → `scene`.
Application tests, gallery controls, and web assets live under `gallery/`. Renderer tests
use their own fixtures and remain independent of these demonstration sources.
