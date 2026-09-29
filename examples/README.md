# Example scenes

Native tools and the browser share the `examples::EXAMPLES` registry. Run any
example with `--scene NAME`; `viewer`, `headless`, and `benchmark` also accept
`--list-scenes`. The browser's grouped Example menu uses the same list, and
`?scene=NAME` selects an example directly.

From the repository root:

```sh
cargo run --release -p hypertrace-wgpu --example headless -- --list-scenes
cargo run --release -p hypertrace-wgpu --features viewer --example viewer -- \
  --scene compare-sp
cargo run --release -p hypertrace-wgpu --example headless -- \
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

The five `compare-*` presets use one generic builder. Every sphere has physical
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
comparison; it and the new spherical presets keep a black miss background.

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

## Use the builders

Scene construction needs no GPU. Lower a builder to the portable scene
definition, then create the renderer scene:

```rust,ignore
use objects::Scene as _;

// The same physical comparison at spherical curvature +1/9.
let source = examples::comparison::scene::<1, 1>(3.0)?;
let definition = source.wgsl_scene()?;
let scene = hypertrace_wgpu::Scene::from_definition(&definition)?;

// Unit-radius spherical recurrence with twelve surface/volume events and fog.
let source = examples::recurrence::scene::<12>(true);
let definition = source.wgsl_scene()?;
```

`comparison::scene::<K,H>(radius)` supports all three signs; Euclidean radius
must be one. The factory rejects spherical radii that would wrap the layout
past the camera's antipode. `recurrence::scene::<H>(false)` selects vacuum.
Existing factories remain available as `eu::scene`, `hy::scene`, `sp::scene`,
and `sp::fog_scene`. See the [renderer guide](../renderer-wgpu/README.md) for
custom shaders and [root README](../README.md) for browser setup.
