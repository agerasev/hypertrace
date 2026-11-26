# Hypergame

Hypergame is a native first-person triangle rasterizer for hyperbolic space.
The demo places congruent octagonal gates at equal hyperbolic intervals.
Walk forward through them or step sideways to see how their apparent size and
direction change. The scene uses unit curvature radius and unlit vertex colors.

From the Hypertrace workspace root, run

```sh
cargo run --release -p hypergame
```

Move with WASD or the arrow keys, rise with Space, descend with C, and roll
with Q/E. Left-drag to look around or press Tab to capture the mouse. Scroll
to change the field of view, press R to reset, and press Escape to close.

## Building a mesh

Each object has an indexed triangle mesh and a hyperbolic pose. The
[scene construction](src/world.rs) takes object-local Poincaré half-space
coordinates `(u, v, h)`, with horizontal coordinates `u, v` and positive
height `h`. The local origin sits at `(0, 0, 1)`. Three indices define a
face in the geodesic plane through its vertices. The same uploaded mesh can
be drawn at many poses.

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

The GPU check reads back overlapping triangles before and after moving the
camera. The windowed smoke check exercises movement, reset, resize, and
presentation.
