# Hypergame

Hypergame is a native first-person triangle rasterizer for hyperbolic space.
Its Earth, Sun, and Moon are three disjoint geodesic planes, as in the original
[`hearth` example](https://github.com/agerasev/hypertrace/blob/20565c3ad9f441358e6a148b98416978133a85d7/main/examples/hearth.rs). The opening view looks across green vegetation toward the Sun
and Moon. The sky is black.

From the Hypertrace workspace root, run

```sh
cargo run --release -p hypergame
```

Move with WASD or the arrow keys, rise with Space, descend with C, and roll
with Q/E. Left-drag to look around or press Tab to capture the mouse. Scroll
to change the field of view, press R to reset, and press Escape to close.

## Permanent day and night

The common perpendicular from Earth to the Sun is three curvature radii long.
Its foot on Earth has permanent noon. Travel along Earth away from that point
and the Sun occupies less of the sky and approaches the horizon. Sunlight fades
continuously into night. The low-poly vegetation grows in the daylight region.
The Moon is gray and diffuse. Its visible brightness comes from the same Sun
that lights the ground and the plants.

Wgame's mesh shader uses a shared uniform for the Sun's radiance. At each
fragment, the Sun plane fills a disk of incoming directions. If the
perpendicular distance to the Sun is `d` and the curvature radius is `R`,
the disk's angular radius `a` satisfies `sin(a) = 1/cosh(d/R)`.
The shader integrates the incoming light over the part of this disk above
the receiving face's horizon. Thin grass blades also transmit some direct
light, so a backlit blade still shows its green color. The model
includes direct diffuse light. It omits cast shadows and indirect light.

## Building a mesh

Each object has an indexed triangle mesh and a hyperbolic pose. The
[scene construction](src/world.rs) takes object-local Poincaré half-space
coordinates `(u, v, h)`, with horizontal coordinates `u, v` and positive
height `h`. The local origin sits at `(0, 0, 1)`. Three indices define a
face in the geodesic plane through its vertices. The same uploaded mesh can
be drawn at many poses.

Each complete plane uses two triangles covering its Klein disk. The fragment
shader clips the disk at the ideal boundary, giving the planes a continuous
horizon as the viewer moves.

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
cargo run -p hypergame -- --smoke
```

GPU checks read back projected triangles and exercise illumination independently
of the demo layout. The windowed smoke check exercises movement, reset, resize,
and presentation.
