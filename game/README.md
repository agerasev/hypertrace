# Hypergame

Hypergame is a native first-person triangle rasterizer for hyperbolic space.
Its Earth and three Suns are geodesic planes. Each Sun is separated from Earth, as in the original
[`hearth` example](https://github.com/agerasev/hypertrace/blob/20565c3ad9f441358e6a148b98416978133a85d7/main/examples/hearth.rs).
A finite triangulated horosphere forms the Moon. The opening view looks
across vegetation toward a warm Sun, a broad red Sun and a small, bright blue
Sun. Each lights the surfaces and air around it. Stars show through the dark sky.

From the Hypertrace workspace root, run

```sh
cargo run --release -p hypergame -- --config game/config.toml
```

Walk with WASD or the arrow keys. Left-drag to look, or press Tab to capture
the mouse. The horizon stays level as you turn and look up or down. Press
Space to jump, scroll to change the field of view, R to reset, and Escape to close.

In the supplied world, the observer stands `0.01875R` above Earth and walks at
`0.1R` per second, where `R` is the curvature radius. Forward and sideways steps follow the ground
even when looking straight up toward the Sun. A jump gives the observer an
upward impulse along the ground normal. Under constant gravity, the observer
rises about a quarter of their standing eye height and lands after 0.58 seconds.
You can keep walking and turning during the jump.

## Configuring a world

Pass a TOML file with `--config` on every launch. Copy [config.toml](config.toml)
to make another world. World settings and material tables are required. Within
a material table, list only the terms that contribute. The game reports unknown
keys and missing required fields at startup. Light colors specify the final
linear RGB radiance. Values above one are valid. Distances and speeds use
curvature radii and seconds.

| Section | Settings |
| --- | --- |
| `earth`, `moon` | Earth material and ground grid, Moon ground location, perpendicular gap, tessellation and material |
| `[[suns]]` | One Sun's ground location, perpendicular gap, surface and air radiances, daytime ambient response and vegetation |
| `ambient` | Shared night fill radiance |
| `atmosphere` | Air layer thickness and background sky radiance |
| `stars` | Stars per square curvature radius, seed, range of gaps above Earth, two RGB colors |
| `vegetation` | Placement seed, candidate sites per species per square curvature radius, maturity, night occupancy, minimum size, shapes, vertex colors and materials |
| `observer`, `view` | Spawn, yaw and pitch, eye height, walk speed, jump height, gravity, window size, field of view, near clip, mouse sensitivity and frame time limit |

Give each Sun a `[[suns]]` entry (up to three), or set `suns = []` at the
top of the file for a world with only night ambient. Sun, Moon and observer
ground offsets start at the world origin and follow local X then local Z.
Each Sun's `distance` is the perpendicular gap above its ground offset.
Keep a Sun that contributes air scattering above the air layer, and the lunar gap above the highest
jumping eye so the edge of the finite Moon stays hidden. Set `jump_height = 0`
to disable jumping, or set `stars.density = 0` for a starless sky.

`reflection` weights light arriving on the visible side, `transmission` weights
light arriving on the opposite side, and `emission` adds the surface's own
light. Both lit terms receive ambient light. For example, a Moon that reflects
gray light and glows faintly blue could use

```toml
[moon.material]
reflection = { weight = 1.0, color = [0.76, 0.76, 0.74] }
emission = { weight = 0.02, color = [0.2, 0.4, 1.0] }
```

The omitted transmission term contributes zero. An empty `material = {}`
absorbs all incident light and has no emission of its own. Each listed term
requires a nonnegative `weight`. Its optional `color` is linear RGB and defaults
to white. The shader uses `weight × color × vertex color` for that term,
multiplying channel by channel. Weights are independent. For example, the
supplied grass sets `material.transmission = { weight = 0.22 }`, so backlighting
uses the blade's own color. Deciduous trees have a `bark` color and a `foliage`
color. Pines have a `bark` color and three foliage tiers named `foliage.lower`,
`foliage.middle` and `foliage.upper`. Grass blades use `colors.root`,
`colors.middle` and `colors.tip` under `vegetation.grass`.

## Permanent day and night

Each Sun has a permanent noon at the foot of its perpendicular to Earth.
As you walk away, its disk shrinks and approaches the horizon. The supplied
warm, red and blue Suns have perpendicular gaps of `3R`, `1.65R` and
`4.3R`, respectively.

Each Sun owns a `[suns.vegetation]` table. The warm Sun supports deciduous
trees, pines and grass. The red Sun favors pines, and the blue Sun supports
grass alone. Omit a species to give it no contribution from that Sun, or
set `vegetation = {}` on a Sun with no biome. Where regions overlap, each
species takes the highest occupancy and the largest growth response offered
by any Sun. In deep night the shared `night_occupancy` and `minimum_size`
leave occasional tufts of grass and rarer small pines. [Vegetation model](src/world/ecology.rs)

To extend one Sun's forest into dusk, lower both ends of its species'
`light_range`. This interval measures direct irradiance at the root in units
of the brightest individual noon on Earth, using the strongest RGB channel.
`max_occupancy` is the bright-end probability of a candidate becoming a plant.
Lower `half_saturation` to approach `max_size` in dimmer light. For pines,
`daylight_range` and `daylight_suppression` can thin that Sun's forest in
its brightest region.

Vegetation grows on a regular `{5,4}` grid. Each tile is a geodesic pentagon
with five right angles, and four tiles meet at each corner. Each pentagon
covers `πR²/2`. The setting `vegetation.site_density_per_species` counts
candidate roots for one species per `R²`. The supplied density of about
102.89 gives each species 161 or 162 candidates per tile, chosen by seeded
rounding to preserve the mean. Roots are sampled uniformly by ground area
before applying the species' light-dependent occupancy.

Crossing a tile boundary updates the loaded ground objects. The game keeps every
tile that could come within `earth.grid.view_radius` of the observer's feet anywhere in
the current pentagon. At `1.5R` this takes 41 tiles, including the current one.
At `1.0R` the current pentagon and its five edge and five corner neighbors
suffice. Each tile has a stable address and seed, so returning
by a different route restores the same plants. [Pentagonal grid](src/world/ground_cells.rs)

To place a building, rock, or other finite mesh on Earth, pass its
`Object<Polygon>` to `World::place_on_ground` with a world-space pose, or
supply it in the ground objects passed to `world::world` at construction.
The object's origin projected onto Earth selects its tile. Its height,
orientation and material are preserved when the tile is loaded again. For
structures spanning many tiles, place sections in their respective tiles so
each section streams with the ground beneath it. [Ground objects](src/world.rs)

At each fragment, each Sun plane fills a disk of incoming directions. If the
perpendicular distance to the Sun is `d` and the curvature radius is `R`,
the disk's angular radius `a` satisfies `sin(a) = 1/cosh(d/R)`.
The shader integrates the incoming light over the part of this disk above
the receiving face's horizon. Thin grass blades also transmit some direct
light, so a backlit blade still shows its green color. Each Sun's
`surface_radiance` colors its visible disk, reflections and transmitted light.
Its `ambient.color` adds diffuse fill above the shared `ambient.night` floor.
The weight of that fill is `clamp(response_factor × peak × sin²(a), 0, 1)^response_exponent`,
where `peak` is the largest channel of that Sun's surface radiance. The shader
sums all contributions in linear RGB before mapping highlights to the display.
The model omits cast shadows.

Each Earth pentagon generates its own stars. A star's foot is sampled uniformly
by area inside that pentagon. The perpendicular from the foot to the star
plane has a length sampled uniformly from `stars.gap`.
Moving toward the foot makes the apparent disk grow. The plane extends across
tile boundaries and keeps its emitted color, sampled uniformly between
`stars.colors.start` and `stars.colors.end` in linear RGB.

`stars.density` sets the mean count per `R²` everywhere on Earth. Each tile's
address and seed reproduce its stars on a return visit. Stars and vegetation
load and unload together under `earth.grid.view_radius`. Each tile batches its
stars in the plane pass. A visible star can disappear when its tile unloads.
[Star field](src/world/stars.rs)

## Scattered daylight

The atmosphere occupies the first `0.45R` above Earth. For each sightline,
the shader adds the sunlight scattered from the air between the eye and the
first visible surface. The same calculation gives an empty sightline its sky
color. Surface and stellar radiance pass through the air at full strength.
This lets a night observer see daylit ground with blue light around it, while
a blade of grass close to the eye picks up very little haze.

The shader clips the ray to the air layer once. For each scattering Sun, it
integrates the full disk with the Rayleigh angular law over that segment.
The closed form also covers sky rays extending to the ideal horizon. Each
Sun's `scattering_radiance` is its effective RGB source in air, including the
air's color response. Set it to `[0, 0, 0]` to keep that Sun's surface lighting
while turning off its sky glow.
[Atmosphere shader](src/atmosphere.wgsl)

## Shapes and materials

Each `Object<S>` has a shape, a hyperbolic pose, and a material. Earth, Sun,
and stars use the `InfinitePlane` shape. The Moon, trees, and grass use indexed
Wgame polygons. Both shapes use the same material blend. Transmission lights
the backlit portion of a surface, which still occludes objects behind it.
[Scene construction](src/world.rs)

Mesh vertices use object-local Poincaré half-space coordinates `(u, v, h)`,
with horizontal coordinates `u, v` and positive height `h`. The local origin
sits at `(0, 0, 1)`. Three indices define a face in the geodesic plane through
its vertices. One uploaded mesh can be drawn at many poses.

The Moon has a regular hexagonal grid with three faces meeting at each vertex.
The six corners of each hexagon sit at chart height `h=1` and lie in one
geodesic plane. Its four triangles share a face normal, so sunlight reveals
the creases between hexagons. Lower `moon.subdivisions` for larger facets.
Their width across opposite sides is `4 / moon.subdivisions` in the
horosphere chart. The mesh extends past the lunar limb as seen from Earth, including
during a jump. [Moon geometry](src/world/moon.rs)

Wgame draws the atmospheric sky first. A depth-tested pass then draws complete
planes using a shared pair of triangles. The plane shader clips each pair to
its open Klein disk, giving the plane a continuous horizon as the viewer moves.
The mesh pass uses the same depth buffer, so the nearer surface is visible
where a mesh overlaps a plane.

## How the camera works

Mesh creation converts each local point once to a hyperboloid point
`(W, X, Y, Z)` and uploads its homogeneous representative
`(X/W, Y/W, Z/W, 1)`, keeping local vertex components bounded. For ground
objects, each loaded tile caches the mesh instances and their object-to-tile
poses. On movement the host composes the inverse eye pose with each occupied
tile's pose in `f64`. The vertex shader applies the fixed object-to-tile pose
followed by the tile-to-eye pose in `f32`. Global objects such as the Moon use
their own camera-relative poses. The eye stays at the origin of the rendering
frame, looking along negative Z. [Tile rendering](src/projection/tiles.rs)

For a camera-relative point, `(X/W, Y/W, Z/W)` are its Klein coordinates.
Geodesic triangles are planar in this model. Passing `(X, Y, Z, W)` to
Wgame's perspective matrix lets the hardware clip triangles that cross the near
plane and rasterize each face directly. Wgame supplies the mesh shader hook,
instancing, depth buffer, and presentation.

The [host code](src/projection.rs) rescales both quaternions of a relative pose
by the same positive factor before upload. This keeps the shader coefficients
bounded and scales all four clip coordinates equally, preserving the image.
Nearby compact meshes retain local detail even when the camera and mesh share
a large world displacement. Distant meshes of fixed physical size occupy fewer
pixels, and depth values crowd together. For a long wall that reaches the
viewer, give sections their own local origins so nearby vertices also have a
nearby mesh origin.

## Checks

```sh
cargo test -p hypergame
WGPU_BACKEND=vulkan cargo test -p hypergame -- --ignored --test-threads=1 --nocapture
cargo run -p hypergame -- --config game/config.toml --smoke
```

GPU checks read back projected triangles, check occlusion between meshes and
planes, and exercise illumination independently of the demo layout.
CPU tests cover walking speed, horizon alignment, and jump
height and landing. The windowed smoke check exercises walking, looking,
jumping, landing, reset, resize, and presentation.
