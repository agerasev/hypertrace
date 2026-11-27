# Hypergame

Hypergame is a native first-person triangle rasterizer for hyperbolic space.
Its Earth and Sun are disjoint geodesic planes inspired by the original
[`hearth` example](https://github.com/agerasev/hypertrace/blob/20565c3ad9f441358e6a148b98416978133a85d7/main/examples/hearth.rs).
A finite triangulated horosphere forms the Moon. The opening view looks
across green vegetation toward the Sun and Moon. Sunlit air gives the day sky
its blue color, and stars show through the dark sky at night.

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
to make another world. Each setting is required, and the game reports missing or
unknown fields at startup. Light colors specify the final linear RGB radiance.
Values above one are valid. Distances and speeds use curvature radii and seconds.

| Section | Settings |
| --- | --- |
| `earth`, `sun`, `moon` | Earth material, perpendicular Sun distance and emission, Moon ground location, perpendicular gap, tessellation and material |
| `ambient` | Night and day fill radiances, response factor and exponent |
| `atmosphere` | Air layer thickness, RGB scattering coefficients, background sky radiance |
| `stars` | Number of planes, seed, ground footprint radius, range of gaps above Earth, two RGB colors |
| `vegetation` | Placement seed, candidate sites per species per square curvature radius, maturity, grid, species occupancy and growth, shapes, vertex colors and materials |
| `observer`, `view` | Spawn, yaw and pitch, eye height, walk speed, jump height, gravity, window size, field of view, near clip, mouse sensitivity and frame time limit |

The Sun's perpendicular meets Earth at the world origin. Moon and observer
ground offsets start there and follow local X then local Z. Heights are measured
along the normal at the resulting foot. Keep the lunar gap above the highest
jumping eye so the edge of the finite Moon stays hidden. Set `jump_height = 0`
to disable jumping, or set the star count to zero for an empty catalogue.

Every material has three required coefficients. `reflection` weights light
arriving on the visible side, `transmission` weights light arriving on the
opposite side, and `emission` adds light of its own. Both lit terms receive
ambient light. The shader adds the three contributions in linear RGB, with no
requirement that the coefficients sum to one.

Earth and Moon use an RGB triple for each coefficient, so a surface can reflect
one color and emit another. Plants use three scalar coefficients with their
bark, foliage or blade vertex colors. The supplied grass uses
`{ reflection = 0.78, transmission = 0.22, emission = 0.0 }`.

## Permanent day and night

The supplied world separates Earth and Sun by three curvature radii along their
common perpendicular. Its foot on Earth has permanent noon. Travel along Earth away from that point
and the Sun occupies less of the sky and approaches the horizon. Sunlight fades
continuously into night. The Moon is gray and diffuse. Its visible brightness
comes from the same Sun that lights the ground and the plants.

Broadleaf trees are most common in daylight. Pines become the more common tree
at dusk, and grass persists farther into the dark. Deep in the night, a patch of
ground is usually bare, but you can still encounter a tuft of grass or, about
twenty times less often, a dwarf pine. Each species also grows taller in better
light. The plants use the direct sunlight at their roots as a fraction of the
light at permanent noon. [Vegetation model](src/world/ecology.rs)

Vegetation grows on a regular `{5,4}` grid. Each tile is a geodesic pentagon
with five right angles, and four tiles meet at each corner. Each pentagon
covers `πR²/2`. The setting `vegetation.site_density_per_species` counts
candidate roots for one species per `R²`. The supplied density of about
102.89 gives each species 161 or 162 candidates per tile, chosen by seeded
rounding to preserve the mean. Roots are sampled uniformly by ground area
before applying the species' light-dependent occupancy.

Crossing a tile boundary updates the loaded vegetation. The game keeps every
tile that could come within `view_radius` of the observer's feet anywhere in
the current pentagon. At `1.5R` this takes 41 tiles, including the current one.
At `1.0R` the current pentagon and its five edge and five corner neighbors
suffice. Each tile has a stable address and seed, so returning
by a different route restores the same plants. [Pentagonal grid](src/world/ground_cells.rs)

Wgame's mesh shader uses a shared uniform for the Sun's radiance. At each
fragment, the Sun plane fills a disk of incoming directions. If the
perpendicular distance to the Sun is `d` and the curvature radius is `R`,
the disk's angular radius `a` satisfies `sin(a) = 1/cosh(d/R)`.
The shader integrates the incoming light over the part of this disk above
the receiving face's horizon. Thin grass blades also transmit some direct
light, so a backlit blade still shows its green color. Direct sunlight is slightly
yellow in the supplied world. The shader interpolates between the night and day
ambient radiances with weight `clamp(factor × peak × sin²(a), 0, 1)^exponent`,
where `peak` is the largest RGB component of the Sun's emitted radiance. This
gives cool, nearly white fill in daylight and a dim floor at night. Overbright
colors retain their hue when mapped to the display. The model omits cast shadows.

Stars are distant emissive geodesic planes. Each star draws one uniform
fraction between zero and one and uses it to interpolate all three channels of
`stars.colors`. Equal-luminance endpoints give every star the same emitted
luminance, as in the supplied world. The renderer draws the entire fixed
catalogue together. Walking toward a star's nearest point on Earth
makes its disk grow, and walking away shrinks it until it covers less than a
pixel. Its emitted radiance stays constant. The nearest points are scattered
uniformly by ground area within a finite disk around noon, so the sky looking
outward beyond that region grows sparse. [Star field](src/world/stars.rs)

## Scattered daylight

The atmosphere occupies the first `0.45R` above Earth. For each sightline,
the shader adds the sunlight scattered from the air between the eye and the
first visible surface. The same calculation gives an empty sightline its sky
color. Surface and stellar radiance pass through the air at full strength.
This lets a night observer see daylit ground with blue light around it, while
a blade of grass close to the eye picks up very little haze.

The shader integrates the full solar disk with the Rayleigh angular law and
constant density inside the layer. The result depends only on the endpoints of
the segment and its direction. It uses a closed form that also covers sky rays
extending to the ideal horizon. RGB scattering strengths favor blue.
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

The lunar mesh samples chart height `h=1`. Its interpolated tangent normals
give the surface continuous shading, while geodesic triangles form the limb.
In this chart Earth is a hemisphere below the Moon. Every above-ground observer
below the lunar gap sees a first-hit disk contained in the mesh, including
during a jump. [Moon geometry](src/world/moon.rs)

Wgame draws the atmospheric sky first. A depth-tested pass then draws complete
planes using a shared pair of triangles. The plane shader clips each pair to
its open Klein disk, giving the plane a continuous horizon as the viewer moves.
The mesh pass uses the same depth buffer, so the nearer surface is visible
where a mesh overlaps a plane.

## How the camera works

Mesh creation converts each local point once to a hyperboloid point
`(W, X, Y, Z)` and uploads its homogeneous representative
`(X/W, Y/W, Z/W, 1)`, keeping local vertex components bounded. Moving the
camera composes its inverse pose with each object's pose in CPU `f64`.
The [vertex shader](src/projection.wgsl) applies
that relative isometry to the local vertex in `f32`. The eye stays at the
origin of the rendering frame, looking along negative Z. Canonical poses and
mesh buffers stay in their original frames.

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
